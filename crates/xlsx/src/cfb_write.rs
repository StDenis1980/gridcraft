//! Writes Compound File Binary (CFB) containers ([MS-CFB]), version 3 with 512-byte sectors, for
//! password-encrypted workbooks: the `EncryptionInfo` and `EncryptedPackage` streams plus the
//! `\x06DataSpaces` storage [MS-OFFCRYPTO] describes. Streams under 4096 bytes go in the mini
//! stream, larger ones in regular sectors; FAT sectors past the header's 109 are listed in DIFAT
//! sectors. Directory siblings form a valid red-black tree (balanced, deepest level red).
//!
//! Clean-room from Microsoft's public [MS-CFB] specification. Sizes are checked: a container that
//! can't be addressed with 32-bit sector ids or stream sizes is an error, never a panic.

/// A storage's child: a stream with its bytes, or a storage with children of its own.
pub(crate) enum Entry<'a> {
    Stream(&'a str, &'a [u8]),
    Storage(&'a str, Vec<Entry<'a>>),
}

impl Entry<'_> {
    fn name(&self) -> &str {
        match self {
            Entry::Stream(n, _) | Entry::Storage(n, _) => n,
        }
    }
}

const SECTOR: usize = 512;
const MINI: usize = 64;
const CUTOFF: usize = 4096;
const ENTRY: usize = 128;
/// FAT sector ids per sector, and DIFAT ids per DIFAT sector (the last slot links the next one).
const IDS: usize = SECTOR / 4;
const DIFAT_IDS: usize = IDS - 1;
const HEADER_DIFAT: usize = 109;
const FREE: u32 = 0xFFFF_FFFF;
const END: u32 = 0xFFFF_FFFE;
const FAT_SECT: u32 = 0xFFFF_FFFD;
const DIFAT_SECT: u32 = 0xFFFF_FFFC;
const NO_STREAM: u32 = 0xFFFF_FFFF;
/// Directory names are at most 31 UTF-16 units plus the terminator.
const MAX_NAME: usize = 31;

/// One directory entry as laid out on disk.
struct Dir {
    name: Vec<u16>,
    kind: u8,
    red: bool,
    left: u32,
    right: u32,
    child: u32,
    start: u32,
    size: u64,
}

fn too_big() -> String {
    "the file is too large for a compound file".into()
}

fn id(n: usize) -> Result<u32, String> {
    u32::try_from(n).ok().filter(|&v| v < 0xFFFF_FFFA).ok_or_else(too_big)
}

/// [MS-CFB] 2.6.4: shorter names sort first, then by upper-cased UTF-16 units.
fn sort_key(name: &str) -> (usize, Vec<u16>) {
    let upper: Vec<u16> = name.chars().flat_map(char::to_uppercase).collect::<String>().encode_utf16().collect();
    (name.encode_utf16().count(), upper)
}

/// A compound file whose root storage holds `entries`.
pub(crate) fn write(entries: &[Entry<'_>]) -> Result<Vec<u8>, String> {
    let mut dirs = vec![Dir {
        name: "Root Entry".encode_utf16().collect(),
        kind: 5,
        red: false,
        left: NO_STREAM,
        right: NO_STREAM,
        child: NO_STREAM,
        start: END,
        size: 0,
    }];
    // Each stream's bytes, by directory index.
    let mut data: Vec<(usize, &[u8])> = vec![];
    dirs[0].child = add_children(entries, &mut dirs, &mut data, 0)?;

    // The mini stream: small streams in 64-byte mini sectors, chained by the mini FAT.
    let mut mini_stream: Vec<u8> = vec![];
    let mut mini_fat: Vec<u32> = vec![];
    let mut big: Vec<(usize, &[u8])> = vec![];
    for &(at, bytes) in &data {
        let dir = dirs.get_mut(at).ok_or_else(too_big)?;
        dir.size = u64::try_from(bytes.len()).map_err(|_| too_big())?;
        if bytes.is_empty() {
            dir.start = END;
        } else if bytes.len() < CUTOFF {
            let first = mini_fat.len();
            let n = bytes.len().div_ceil(MINI);
            for k in 1..=n {
                mini_fat.push(if k == n { END } else { id(first + k)? });
            }
            dir.start = id(first)?;
            mini_stream.extend_from_slice(bytes);
            mini_stream.resize((first + n) * MINI, 0);
        } else {
            big.push((at, bytes));
        }
    }
    if let Some(root) = dirs.first_mut() {
        root.size = u64::try_from(mini_stream.len()).map_err(|_| too_big())?;
    }

    // Regular-sector blobs in file order after the FAT: directory, mini FAT, mini stream, big streams.
    let dir_bytes = dirs.len().div_ceil(SECTOR / ENTRY) * SECTOR;
    let mini_fat_bytes: Vec<u8> = mini_fat.iter().flat_map(|v| v.to_le_bytes()).collect();
    let mut blobs: Vec<(Option<usize>, usize)> = vec![(None, dir_bytes), (None, mini_fat_bytes.len()), (None, mini_stream.len())];
    blobs.extend(big.iter().map(|&(at, b)| (Some(at), b.len())));
    let counts: Vec<usize> = blobs.iter().map(|(_, len)| len.div_ceil(SECTOR)).collect();
    let body = counts.iter().try_fold(0usize, |a, &n| a.checked_add(n)).ok_or_else(too_big)?;

    // FAT sectors must also cover themselves and the DIFAT sectors listing them.
    let difat_for = |fat_n: usize| fat_n.saturating_sub(HEADER_DIFAT).div_ceil(DIFAT_IDS);
    let mut fat_n = 1usize;
    loop {
        let need = fat_n.checked_add(difat_for(fat_n)).and_then(|v| v.checked_add(body)).ok_or_else(too_big)?;
        if fat_n.checked_mul(IDS).ok_or_else(too_big)? >= need {
            break;
        }
        fat_n += 1;
    }
    let difat_n = difat_for(fat_n);
    let total = fat_n + difat_n + body;
    id(total)?;
    let mut fat = vec![FREE; fat_n * IDS];
    // Layout: DIFAT sectors, FAT sectors, then the blobs.
    for slot in fat.iter_mut().take(difat_n) {
        *slot = DIFAT_SECT;
    }
    for slot in fat.iter_mut().skip(difat_n).take(fat_n) {
        *slot = FAT_SECT;
    }
    let mut starts = vec![];
    let mut next = difat_n + fat_n;
    for &n in &counts {
        if n == 0 {
            starts.push(END);
            continue;
        }
        starts.push(id(next)?);
        for k in 1..=n {
            let v = if k == n { END } else { id(next + k)? };
            *fat.get_mut(next + k - 1).ok_or_else(too_big)? = v;
        }
        next += n;
    }
    let start_of = |i: usize| starts.get(i).copied().unwrap_or(END);
    if let Some(root) = dirs.first_mut() {
        root.start = if mini_stream.is_empty() { END } else { start_of(2) };
    }
    for (k, &(at, _)) in big.iter().enumerate() {
        if let Some(d) = dirs.get_mut(at) {
            d.start = start_of(3 + k);
        }
    }

    let mut out = vec![0u8; SECTOR.checked_mul(total + 1).ok_or_else(too_big)?];
    let put = |out: &mut Vec<u8>, at: usize, bytes: &[u8]| -> Result<(), String> {
        out.get_mut(at..at + bytes.len()).ok_or_else(too_big)?.copy_from_slice(bytes);
        Ok(())
    };
    // Header.
    put(&mut out, 0, &crate::cfb::SIGNATURE)?;
    put(&mut out, 24, &0x3Eu16.to_le_bytes())?; // minor version
    put(&mut out, 26, &3u16.to_le_bytes())?; // major version 3
    put(&mut out, 28, &0xFFFEu16.to_le_bytes())?; // little-endian
    put(&mut out, 30, &9u16.to_le_bytes())?; // 512-byte sectors
    put(&mut out, 32, &6u16.to_le_bytes())?; // 64-byte mini sectors
    put(&mut out, 44, &id(fat_n)?.to_le_bytes())?;
    put(&mut out, 48, &start_of(0).to_le_bytes())?;
    put(&mut out, 56, &u32::try_from(CUTOFF).map_err(|_| too_big())?.to_le_bytes())?;
    put(&mut out, 60, &start_of(1).to_le_bytes())?;
    put(&mut out, 64, &id(counts.get(1).copied().unwrap_or(0))?.to_le_bytes())?;
    put(&mut out, 68, &(if difat_n == 0 { END } else { 0 }).to_le_bytes())?;
    put(&mut out, 72, &id(difat_n)?.to_le_bytes())?;
    let fat_ids: Vec<u32> = (difat_n..difat_n + fat_n).map(id).collect::<Result<_, _>>()?;
    for i in 0..HEADER_DIFAT {
        put(&mut out, 76 + i * 4, &fat_ids.get(i).copied().unwrap_or(FREE).to_le_bytes())?;
    }
    // DIFAT sectors: 127 FAT sector ids each, then the next DIFAT sector (END on the last).
    for d in 0..difat_n {
        let base = SECTOR * (1 + d);
        for i in 0..DIFAT_IDS {
            let v = fat_ids.get(HEADER_DIFAT + d * DIFAT_IDS + i).copied().unwrap_or(FREE);
            put(&mut out, base + i * 4, &v.to_le_bytes())?;
        }
        let link = if d + 1 == difat_n { END } else { id(d + 1)? };
        put(&mut out, base + DIFAT_IDS * 4, &link.to_le_bytes())?;
    }
    // FAT sectors.
    for (i, v) in fat.iter().enumerate() {
        put(&mut out, SECTOR * (1 + difat_n) + i * 4, &v.to_le_bytes())?;
    }
    // Blobs.
    let mut dir_blob = vec![0u8; dir_bytes];
    for (k, d) in dirs.iter().enumerate() {
        encode_dir(&mut dir_blob, k * ENTRY, d)?;
    }
    for k in dirs.len()..dir_bytes / ENTRY {
        // Unused entries: no siblings, no child.
        for at in [68, 72, 76] {
            put(&mut dir_blob, k * ENTRY + at, &FREE.to_le_bytes())?;
        }
    }
    let content = |i: usize| -> &[u8] {
        match i {
            0 => &dir_blob,
            1 => &mini_fat_bytes,
            2 => &mini_stream,
            _ => big.get(i - 3).map(|&(_, b)| b).unwrap_or_default(),
        }
    };
    for (i, &start) in starts.iter().enumerate() {
        if start != END {
            let at = SECTOR * (1 + usize::try_from(start).map_err(|_| too_big())?);
            put(&mut out, at, content(i))?;
        }
    }
    Ok(out)
}

/// Appends `entries` as one sibling tree and returns its root's index.
fn add_children<'a>(entries: &'a [Entry<'a>], dirs: &mut Vec<Dir>, data: &mut Vec<(usize, &'a [u8])>, depth: usize) -> Result<u32, String> {
    if depth > 8 {
        return Err("compound file storages nest too deeply".into());
    }
    if entries.is_empty() {
        return Ok(NO_STREAM);
    }
    let mut order: Vec<&Entry<'a>> = entries.iter().collect();
    order.sort_by_key(|e| sort_key(e.name()));
    let mut indices = vec![];
    for e in &order {
        let name: Vec<u16> = e.name().encode_utf16().collect();
        if name.is_empty() || name.len() > MAX_NAME {
            return Err(format!("a compound file entry name must be 1 to {MAX_NAME} characters"));
        }
        let at = dirs.len();
        let kind = if matches!(e, Entry::Storage(..)) { 1 } else { 2 };
        dirs.push(Dir {
            name,
            kind,
            red: false,
            left: NO_STREAM,
            right: NO_STREAM,
            child: NO_STREAM,
            start: if kind == 1 { 0 } else { END },
            size: 0,
        });
        indices.push(at);
    }
    for (e, &at) in order.iter().zip(&indices) {
        match e {
            Entry::Stream(_, bytes) => data.push((at, bytes)),
            Entry::Storage(_, kids) => {
                let child = add_children(kids, dirs, data, depth + 1)?;
                if let Some(d) = dirs.get_mut(at) {
                    d.child = child;
                }
            }
        }
    }
    // The deepest level is red unless the tree is perfect (then every leaf is on it).
    let height = (usize::BITS - indices.len().leading_zeros()) as usize;
    let red_level = if (indices.len() + 1).is_power_of_two() { None } else { Some(height) };
    link(dirs, &indices, 1, red_level)
}

/// Builds a balanced tree from sorted `indices`; nodes on the deepest level of an imperfect tree
/// are red, so every path from the root holds the same number of black nodes.
fn link(dirs: &mut [Dir], indices: &[usize], depth: usize, red_level: Option<usize>) -> Result<u32, String> {
    if indices.is_empty() {
        return Ok(NO_STREAM);
    }
    let mid = indices.len() / 2;
    let (left, rest) = indices.split_at(mid);
    let Some((&me, right)) = rest.split_first() else { return Ok(NO_STREAM) };
    let l = link(dirs, left, depth + 1, red_level)?;
    let r = link(dirs, right, depth + 1, red_level)?;
    let d = dirs.get_mut(me).ok_or_else(too_big)?;
    d.left = l;
    d.right = r;
    d.red = red_level == Some(depth);
    id(me)
}

fn encode_dir(blob: &mut [u8], at: usize, d: &Dir) -> Result<(), String> {
    let e = blob.get_mut(at..at + ENTRY).ok_or_else(too_big)?;
    for (i, u) in d.name.iter().enumerate() {
        e.get_mut(i * 2..i * 2 + 2).ok_or_else(too_big)?.copy_from_slice(&u.to_le_bytes());
    }
    let name_len = u16::try_from((d.name.len() + 1) * 2).map_err(|_| too_big())?;
    e.get_mut(64..66).ok_or_else(too_big)?.copy_from_slice(&name_len.to_le_bytes());
    *e.get_mut(66).ok_or_else(too_big)? = d.kind;
    *e.get_mut(67).ok_or_else(too_big)? = u8::from(!d.red);
    for (off, v) in [(68, d.left), (72, d.right), (76, d.child), (116, d.start)] {
        e.get_mut(off..off + 4).ok_or_else(too_big)?.copy_from_slice(&v.to_le_bytes());
    }
    e.get_mut(120..128).ok_or_else(too_big)?.copy_from_slice(&d.size.to_le_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cfb::Compound;

    fn payload(len: usize, seed: u8) -> Vec<u8> {
        (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(seed)).collect()
    }

    #[test]
    fn small_and_large_streams_read_back() {
        for (a, b) in [(0usize, 10usize), (100, 5000), (4095, 4096), (70_000, 3), (1, 1)] {
            let (x, y) = (payload(a, 1), payload(b, 2));
            let file = write(&[Entry::Stream("EncryptionInfo", &x), Entry::Stream("EncryptedPackage", &y)]).unwrap();
            assert_eq!(file.len() % SECTOR, 0);
            let cf = Compound::open(&file).unwrap();
            assert_eq!(cf.root_stream("EncryptionInfo").unwrap().unwrap(), x, "{a}/{b}");
            assert_eq!(cf.root_stream("EncryptedPackage").unwrap().unwrap(), y, "{a}/{b}");
        }
    }

    #[test]
    fn storages_and_many_siblings_read_back() {
        let blobs: Vec<Vec<u8>> = (0..9).map(|i| payload(10 + i * 700, i as u8)).collect();
        let names = ["A", "BB", "Ccc", "D", "Eeeee", "F", "Gg", "H", "Iiii"];
        let inner = vec![Entry::Stream("Deep", &blobs[0])];
        let mut entries: Vec<Entry> = names.iter().zip(&blobs).map(|(n, b)| Entry::Stream(n, b)).collect();
        entries.push(Entry::Storage("\u{6}DataSpaces", inner));
        let file = write(&entries).unwrap();
        let cf = Compound::open(&file).unwrap();
        for (n, b) in names.iter().zip(&blobs) {
            assert_eq!(cf.root_stream(n).unwrap().as_ref(), Some(b), "{n}");
        }
    }

    #[test]
    fn red_black_rules_hold() {
        for n in 1..40usize {
            let names: Vec<String> = (0..n).map(|i| format!("S{i:03}")).collect();
            let data = [0u8; 3];
            let entries: Vec<Entry> = names.iter().map(|s| Entry::Stream(s, &data)).collect();
            let mut dirs = vec![];
            let mut streams = vec![];
            let root = add_children(&entries, &mut dirs, &mut streams, 0).unwrap();
            // Every root-to-leaf path has the same black count, and no red node has a red child.
            fn walk(dirs: &[Dir], at: u32, parent_red: bool) -> usize {
                if at == NO_STREAM {
                    return 1;
                }
                let d = &dirs[at as usize];
                assert!(!(d.red && parent_red), "red child of a red node");
                let (l, r) = (walk(dirs, d.left, d.red), walk(dirs, d.right, d.red));
                assert_eq!(l, r, "unequal black heights");
                l + usize::from(!d.red)
            }
            assert!(!dirs[root as usize].red, "the tree's root is black");
            walk(&dirs, root, false);
        }
    }

    #[test]
    fn many_fat_sectors_use_difat_sectors() {
        // 110 FAT sectors' worth of sectors needs one DIFAT sector.
        let big = payload(110 * IDS * SECTOR, 7);
        let file = write(&[Entry::Stream("EncryptedPackage", &big)]).unwrap();
        assert_eq!(u32::from_le_bytes(file[72..76].try_into().unwrap()), 1, "one DIFAT sector");
        let cf = Compound::open(&file).unwrap();
        assert!(cf.root_stream("EncryptedPackage").unwrap().unwrap() == big);
    }

    #[test]
    fn bad_names_are_errors() {
        assert!(write(&[Entry::Stream("", &[])]).is_err());
        let long = "x".repeat(32);
        assert!(write(&[Entry::Stream(&long, &[])]).is_err());
    }
}
