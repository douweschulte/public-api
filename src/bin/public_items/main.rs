use std::io::Write;
use std::path::Path;

use public_items::Options;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut args = std::env::args_os();

    eprintln!("{:?}", std::env::args());

    if flag_raised("--help") || flag_raised("-h") || args.len() == 1 {
        print_usage()?;
    } else if args.len() == 2 {
        let mut options = Options::default();
        options.with_blanket_implementations = flag_raised("--with-blanket-implementations");
        options.sorted = true;
        print_public_api_items(Path::new(&args.nth(1).unwrap()), options)?;
    } else if args.len() == 3 {

    }

    Ok(())
}

fn print_public_api_items(path: &Path, options: Options) -> Result<()> {
    let json = &std::fs::read_to_string(path)?;

    for public_item in public_items::public_items_from_rustdoc_json_str(json, options)? {
        writeln!(std::io::stdout(), "{}", public_item)?;
    }

    Ok(())
}

fn print_usage() -> std::io::Result<()> {
    writeln!(
        std::io::stdout(),
        r"
NOTE: See https://github.com/Enselic/cargo-public-items for a convenient cargo
wrapper around this program (or to be precise; library) that does everything
automatically.

If you insist of using this low-level utility and thin wrapper, you run it like this:

   public_items <RUSTDOC_JSON_FILE>

where RUSTDOC_JSON_FILE is the path to the output of

  RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc --lib --no-deps

which you can find in

  ./target/doc/${{CRATE}}.json

To diff the public API between two commits, you generate one rustdoc JSON file for each
commit and then pass the path of both files to this utility:

   public_items <RUSTDOC_JSON_FILE_OLD> <RUSTDOC_JSON_FILE_NEW>

To include blanket implementations, pass --with-blanket-implementations.
"
    )
}

/// Helper to check if a flag is raised in command line args.
///
/// Note: I want this Rust package to be simple and without unnecessary
/// dependencies and without the need to select features. For that reason I
/// currently consider it undesirable to for example make this utility depend on
/// `clap` or `anyhow`.
///
/// The convenient wrapper <https://github.com/Enselic/cargo-public-items>
/// depends on clap though which is perfectly fine.
fn flag_raised(flag: &str) -> bool {
    std::env::args_os().into_iter().any(|e| e == flag)
}
