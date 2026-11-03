//! Regional formats: how numbers and dates are written for people in a country. Files, formulas
//! and the engine API stay en-US; the interface shows and reads values in the user's region.

/// A set of regional conventions. `EnUs` is the file and API format and the default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Region {
    #[default]
    EnUs,
    /// Spain: `1.234,56`, `31/12/2026`, euro after the amount.
    EsEs,
    /// Ukraine: `1 234,56` (non-breaking space), `31.12.2026`, hryvnia after the amount.
    UkUa,
    /// Czechia: `1 234,56` (non-breaking space), `31.12.2026`, koruna after the amount.
    CsCz,
    /// Romania: `1.234,56`, `31.12.2026`, leu after the amount.
    RoRo,
    /// Lithuania: `1 234,56` (non-breaking space), `2026-12-31`, euro after the amount.
    LtLt,
    /// Poland: `1 234,56` (non-breaking space), `31.12.2026`, złoty after the amount.
    PlPl,
    /// Italy: `1.234,56`, `31/12/2026`, euro after the amount.
    ItIt,
    /// France: `1 234,56` (non-breaking space), `31/12/2026`, euro after the amount.
    FrFr,
    /// Germany: `1.234,56`, `31.12.2026`, euro after the amount.
    DeDe,
}

impl Region {
    pub const ALL: [Region; 10] =
        [Region::EnUs, Region::EsEs, Region::UkUa, Region::CsCz, Region::RoRo, Region::LtLt, Region::DeDe, Region::FrFr, Region::ItIt, Region::PlPl];

    /// The decimal separator.
    pub fn decimal(self) -> char {
        match self {
            Region::EnUs => '.',
            Region::EsEs | Region::UkUa | Region::CsCz | Region::RoRo | Region::LtLt | Region::DeDe | Region::FrFr | Region::ItIt | Region::PlPl => {
                ','
            }
        }
    }

    /// The thousands separator.
    pub fn group(self) -> char {
        match self {
            Region::EnUs => ',',
            Region::EsEs | Region::RoRo | Region::DeDe | Region::ItIt => '.',
            Region::UkUa | Region::CsCz | Region::LtLt | Region::FrFr | Region::PlPl => '\u{a0}',
        }
    }

    /// The format code the short date (built-in format 14, `m/d/yyyy`) shows as.
    pub fn short_date(self) -> &'static str {
        match self {
            Region::EnUs => "m/d/yyyy",
            Region::EsEs | Region::FrFr | Region::ItIt => "dd/mm/yyyy",
            Region::UkUa | Region::CsCz | Region::RoRo | Region::DeDe | Region::PlPl => "dd.mm.yyyy",
            Region::LtLt => "yyyy-mm-dd",
        }
    }

    /// The format code the short date and time (built-in format 22, `m/d/yyyy h:mm`) shows as.
    pub fn short_date_time(self) -> &'static str {
        match self {
            Region::EnUs => "m/d/yyyy h:mm",
            Region::EsEs | Region::FrFr | Region::ItIt => "dd/mm/yyyy h:mm",
            Region::UkUa | Region::CsCz | Region::RoRo | Region::DeDe | Region::PlPl => "dd.mm.yyyy h:mm",
            Region::LtLt => "yyyy-mm-dd h:mm",
        }
    }

    /// Whether typed dates are read day first (`31/12`), as everywhere but the United States.
    /// Lithuania writes dates year first; a four-digit first part is always read that way.
    pub fn day_first(self) -> bool {
        self != Region::EnUs
    }

    /// The local currency symbol, written after the amount.
    pub fn currency(self) -> Option<&'static str> {
        match self {
            Region::EnUs => None,
            Region::EsEs | Region::LtLt | Region::DeDe | Region::FrFr | Region::ItIt => Some("€"),
            Region::UkUa => Some("₴"),
            Region::CsCz => Some("Kč"),
            Region::RoRo => Some("lei"),
            Region::PlPl => Some("zł"),
        }
    }

    /// The region's BCP 47 tag (`es-ES`).
    pub fn tag(self) -> &'static str {
        match self {
            Region::EnUs => "en-US",
            Region::EsEs => "es-ES",
            Region::UkUa => "uk-UA",
            Region::CsCz => "cs-CZ",
            Region::RoRo => "ro-RO",
            Region::LtLt => "lt-LT",
            Region::PlPl => "pl-PL",
            Region::ItIt => "it-IT",
            Region::FrFr => "fr-FR",
            Region::DeDe => "de-DE",
        }
    }

    /// The region for a BCP 47 tag or language code (`es`, `es-ES`, `uk`, `ca`, `cs`, `ro`, `lt`…), if known.
    pub fn from_tag(tag: &str) -> Option<Region> {
        let lang = tag.split(['-', '_']).next().unwrap_or("").to_ascii_lowercase();
        match lang.as_str() {
            "en" => Some(Region::EnUs),
            // The languages of Spain write numbers and dates the Spanish way.
            "es" | "ca" | "gl" | "eu" => Some(Region::EsEs),
            "uk" => Some(Region::UkUa),
            "cs" => Some(Region::CsCz),
            "ro" => Some(Region::RoRo),
            "lt" => Some(Region::LtLt),
            "pl" => Some(Region::PlPl),
            "it" => Some(Region::ItIt),
            "fr" => Some(Region::FrFr),
            "de" => Some(Region::DeDe),
            _ => None,
        }
    }
}
