//! Every language with a function table (Spanish, Czech…): names round-trip, are unique and
//! leave file syntax English.

use gridcraft_formula::{FormulaLocale, parse_input, print, print_input};

fn localized() -> impl Iterator<Item = FormulaLocale> {
    FormulaLocale::ALL.into_iter().filter(|l| l.is_localized())
}

fn input(src: &str, locale: FormulaLocale) -> String {
    print(&parse_input(src, locale, |_| false).unwrap_or_else(|e| panic!("{src}: {e:?}")))
}

#[test]
fn every_localized_function_name_round_trips_and_is_unique() {
    assert!(localized().count() >= 2, "Spanish and Czech at least");
    for locale in localized() {
        let pairs = locale.function_names();
        for &(en, name) in pairs {
            for typed in [name.to_string(), name.to_lowercase()] {
                let expr = parse_input(&format!("={typed}(1;2,5)"), locale, |_| false).unwrap_or_else(|e| panic!("{locale:?} {typed}: {e:?}"));
                assert_eq!(print(&expr), format!("{en}(1,2.5)"), "{locale:?} {typed}");
                assert_eq!(print_input(&expr, locale, |_| false).unwrap(), format!("{name}(1;2,5)"), "{locale:?} {typed}");
            }
            assert_eq!(locale.function_name(en), name, "{locale:?}");
            assert_eq!(locale.canonical_name(name), en, "{locale:?}");
        }
        let mut english: Vec<&str> = pairs.iter().map(|p| p.0).collect();
        let mut local: Vec<String> = pairs.iter().map(|p| p.1.to_uppercase()).collect();
        english.sort_unstable();
        local.sort_unstable();
        english.dedup();
        local.dedup();
        assert_eq!((english.len(), local.len()), (pairs.len(), pairs.len()), "{locale:?}: one row per function, one function per name");
        // Every table covers the same functions as Spanish's.
        assert_eq!(pairs.len(), FormulaLocale::Es.function_names().len(), "{locale:?}");
    }
}

#[test]
fn czech_formulas_are_stored_in_english() {
    let cs = FormulaLocale::Cs;
    for (src, expected) in [
        ("=suma(A1:A10)", "SUM(A1:A10)"),
        ("KDYŽ(A1>0;\"Ano\";\"Ne\")", "IF(A1>0,\"Ano\",\"Ne\")"),
        ("když(A1>0;1;0)", "IF(A1>0,1,0)"),
        ("SVYHLEDAT(A2;D:E;2;NEPRAVDA)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("PRŮMĚR(A1:A5)+POČET2(B:B)", "AVERAGE(A1:A5)+COUNTA(B:B)"),
        ("ZAOKROUHLIT(1,005;2)", "ROUND(1.005,2)"),
        ("DATUM(2026;11;3)-DNES()", "DATE(2026,11,3)-TODAY()"),
        // Czech LOG is LOG10, and LOGZ is LOG; VAR is VARP.
        ("LOG(100)+LOGZ(8;2)+VAR(A1:A3)", "LOG10(100)+LOG(8,2)+VARP(A1:A3)"),
        // Names Czech Excel keeps in English.
        ("SUMIF(A:A;\">0\")+XLOOKUP(1;A:A;B:B)", "SUMIF(A:A,\">0\")+XLOOKUP(1,A:A,B:B)"),
        ("{1\\2;PRAVDA\\NEPRAVDA}", "{1,2;TRUE,FALSE}"),
        ("A(PRAVDA;NE(NEPRAVDA))", "AND(TRUE,NOT(FALSE))"),
    ] {
        assert_eq!(input(src, cs), expected, "{src}");
    }
    let shown = |src: &str| print_input(&gridcraft_formula::parse(src).unwrap(), cs, |_| false).unwrap();
    assert_eq!(shown("IF(A1>0.5,TRUE,FALSE)"), "KDYŽ(A1>0,5;PRAVDA;NEPRAVDA)");
    assert_eq!(shown("LOG10(100)+VAR(A1:A3)+VAR.S(A1:A3)"), "LOG(100)+VAR.VÝBĚR(A1:A3)+VAR.S(A1:A3)");
    // A defined name spelled like a Czech function is still the name.
    let known = |name: &str| name.eq_ignore_ascii_case("SUMA");
    assert_eq!(print(&parse_input("SUMA+1", cs, known).unwrap()), "SUMA+1");
}

#[test]
fn spanish_booleans_are_not_czech_and_back() {
    // Each language reads only its own TRUE and FALSE.
    assert_eq!(input("{VERDADERO}", FormulaLocale::Es), "{TRUE}");
    assert!(parse_input("{VERDADERO}", FormulaLocale::Cs, |_| false).is_err());
    assert!(parse_input("{PRAVDA}", FormulaLocale::Es, |_| false).is_err());
    assert_eq!(FormulaLocale::Cs.boolean_names(), ("PRAVDA", "NEPRAVDA"));
    assert_eq!(FormulaLocale::En.boolean_names(), ("TRUE", "FALSE"));
    assert!(FormulaLocale::Semicolon.function_names().is_empty());
}

#[test]
fn german_formulas_are_stored_in_english() {
    let de = FormulaLocale::De;
    for (src, expected) in [
        ("=summe(A1:A10)", "SUM(A1:A10)"),
        ("WENN(A1>0;\"Ja\";\"Nein\")", "IF(A1>0,\"Ja\",\"Nein\")"),
        ("SVERWEIS(A2;D:E;2;FALSCH)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("ZÄHLENWENN(A:A;\">0\")+zählenwenns(A:A;1)", "COUNTIF(A:A,\">0\")+COUNTIFS(A:A,1)"),
        ("RUNDEN(MITTELWERT(1;2,5);1)", "ROUND(AVERAGE(1,2.5),1)"),
        ("XVERWEIS(1;A:A;B:B)+JAHR(HEUTE())", "XLOOKUP(1,A:A,B:B)+YEAR(TODAY())"),
        ("{1\\2;WAHR\\FALSCH}", "{1,2;TRUE,FALSE}"),
    ] {
        assert_eq!(input(src, de), expected, "{src}");
    }
    let shown = |src: &str| print_input(&gridcraft_formula::parse(src).unwrap(), de, |_| false).unwrap();
    assert_eq!(shown("IF(A1>0.5,TRUE,SUMIFS(B:B,A:A,1))"), "WENN(A1>0,5;WAHR;SUMMEWENNS(B:B;A:A;1))");
}

#[test]
fn french_formulas_are_stored_in_english() {
    let locale = FormulaLocale::Fr;
    for (src, expected) in [
        ("=somme(A1:A10)", "SUM(A1:A10)"),
        ("SI(A1>0;\"Oui\";\"Non\")", "IF(A1>0,\"Oui\",\"Non\")"),
        ("RECHERCHEV(A2;D:E;2;FAUX)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("NB.SI(A:A;\">0\")+NBVAL(B:B)", "COUNTIF(A:A,\">0\")+COUNTA(B:B)"),
        ("ARRONDI(MOYENNE(1;2,5);1)", "ROUND(AVERAGE(1,2.5),1)"),
        ("SUPPRESPACE(A1)&TRIM(B:B;0,1;0,2)", "TRIM(A1)&MIRR(B:B,0.1,0.2)"),
        ("{1\\2;VRAI\\FAUX}", "{1,2;TRUE,FALSE}"),
    ] {
        assert_eq!(input(src, locale), expected, "{src}");
    }
    let shown = |src: &str| print_input(&gridcraft_formula::parse(src).unwrap(), locale, |_| false).unwrap();
    assert_eq!(shown("IF(A1>0.5,TRUE,SUMIFS(B:B,A:A,1))"), "SI(A1>0,5;VRAI;SOMME.SI.ENS(B:B;A:A;1))");
}

#[test]
fn italian_formulas_are_stored_in_english() {
    let locale = FormulaLocale::It;
    for (src, expected) in [
        ("=somma(A1:A10)", "SUM(A1:A10)"),
        ("SE(A1>0;\"Sì\";\"No\")", "IF(A1>0,\"Sì\",\"No\")"),
        ("CERCA.VERT(A2;D:E;2;FALSO)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("CONTA.SE(A:A;\">0\")+CONTA.VALORI(B:B)", "COUNTIF(A:A,\">0\")+COUNTA(B:B)"),
        ("ARROTONDA(MEDIA(1;2,5);1)", "ROUND(AVERAGE(1,2.5),1)"),
        ("SEN(0)+E(VERO;NON(FALSO))", "SIN(0)+AND(TRUE,NOT(FALSE))"),
        ("{1\\2;VERO\\FALSO}", "{1,2;TRUE,FALSE}"),
    ] {
        assert_eq!(input(src, locale), expected, "{src}");
    }
    let shown = |src: &str| print_input(&gridcraft_formula::parse(src).unwrap(), locale, |_| false).unwrap();
    assert_eq!(shown("IF(A1>0.5,TRUE,SUMIFS(B:B,A:A,1))"), "SE(A1>0,5;VERO;SOMMA.PIÙ.SE(B:B;A:A;1))");
}
