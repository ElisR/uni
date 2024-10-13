use clap::{Arg, Command};
use std::collections::HashMap;

fn main() {
    // Define the command line arguments
    let matches = Command::new("Unicode Finder")
        .version("1.0")
        .about("Finds the Unicode character for a given name")
        .arg(
            Arg::new("name")
                .help("The Unicode name (e.g., 'Beta')")
                .required(true)
                .index(1),
        )
        .get_matches();

    let name = matches.get_one::<String>("name").unwrap().as_str();

    let unicode_map = create_unicode_map();

    if let Some(&character) = unicode_map.get(name) {
        println!("{}", character)
    }
}

fn create_unicode_map() -> HashMap<&'static str, char> {
    let mut map = HashMap::new();

    // Uppercase Greek letters
    map.insert("Alpha", '\u{0391}');
    map.insert("Beta", '\u{0392}');
    map.insert("Gamma", '\u{0393}');
    map.insert("Delta", '\u{0394}');
    map.insert("Epsilon", '\u{0395}');
    map.insert("Zeta", '\u{0396}');
    map.insert("Eta", '\u{0397}');
    map.insert("Theta", '\u{0398}');
    map.insert("Iota", '\u{0399}');
    map.insert("Kappa", '\u{039A}');
    map.insert("Lambda", '\u{039B}');
    map.insert("Mu", '\u{039C}');
    map.insert("Nu", '\u{039D}');
    map.insert("Xi", '\u{039E}');
    map.insert("Omicron", '\u{039F}');
    map.insert("Pi", '\u{03A0}');
    map.insert("Rho", '\u{03A1}');
    map.insert("Sigma", '\u{03A3}');
    map.insert("Tau", '\u{03A4}');
    map.insert("Upsilon", '\u{03A5}');
    map.insert("Phi", '\u{03A6}');
    map.insert("Chi", '\u{03A7}');
    map.insert("Psi", '\u{03A8}');
    map.insert("Omega", '\u{03A9}');

    // Lowercase Greek letters
    map.insert("alpha", '\u{03B1}');
    map.insert("beta", '\u{03B2}');
    map.insert("gamma", '\u{03B3}');
    map.insert("delta", '\u{03B4}');
    map.insert("epsilon", '\u{03B5}');
    map.insert("zeta", '\u{03B6}');
    map.insert("eta", '\u{03B7}');
    map.insert("theta", '\u{03B8}');
    map.insert("iota", '\u{03B9}');
    map.insert("kappa", '\u{03BA}');
    map.insert("lambda", '\u{03BB}');
    map.insert("mu", '\u{03BC}');
    map.insert("nu", '\u{03BD}');
    map.insert("xi", '\u{03BE}');
    map.insert("omicron", '\u{03BF}');
    map.insert("pi", '\u{03C0}');
    map.insert("rho", '\u{03C1}');
    map.insert("sigma", '\u{03C3}');
    map.insert("tau", '\u{03C4}');
    map.insert("upsilon", '\u{03C5}');
    map.insert("phi", '\u{03C6}');
    map.insert("chi", '\u{03C7}');
    map.insert("psi", '\u{03C8}');
    map.insert("omega", '\u{03C9}');

    map
}
