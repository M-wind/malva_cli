use std::{fmt::Display, fs, io, path::PathBuf, process::exit};

use clap::{Parser, Subcommand, builder::styling};
use malva::{config::FormatOptions, detect_syntax, format_text};

const CLAP_STYLING: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Yellow.on_default().bold())
    .usage(styling::AnsiColor::Yellow.on_default())
    .literal(styling::AnsiColor::Green.on_default())
    .placeholder(styling::AnsiColor::BrightWhite.on_default())
    .error(styling::AnsiColor::Red.on_default())
    .valid(styling::AnsiColor::Green.on_default())
    .invalid(styling::AnsiColor::Yellow.on_default());

#[derive(Debug, Parser)]
#[command(styles = CLAP_STYLING)]
#[command(override_usage = "\n  malva [OPTIONS] format [FILE]")]
#[command(
    version,
    about = "Configurable, smart and fast CSS, SCSS, Sass and Less formatter.\nDocumentation -- https://malva.netlify.app"
)]
struct Malva {
    /// Config path 
    #[clap(short, long)]
    config: Option<PathBuf>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
#[command(disable_help_subcommand(true))]
enum Command {
    /// Format a file
    Format {
        /// File path
        file: PathBuf,
    },
}

fn print_err<T, E>(err: T) -> E
where
    T: Display,
{
    print!("{}", err);
    exit(1)
}

fn main() {
    let args = Malva::parse();

    let options = match args.config {
        Some(path) => {
            let str = fs::read_to_string(path).unwrap_or_else(print_err);
            serde_json::from_str(&str).unwrap_or_else(print_err)
        }
        None => FormatOptions::default(),
    };
    let filepath = match args.command {
        Command::Format { file } => file,
    };

    let Some(syntax) = detect_syntax(&filepath) else {
        print_err(io::Error::new(
            io::ErrorKind::Other,
            format!("Unknown file extension of file: {:#?}", &filepath),
        ))
    };

    let code = fs::read_to_string(filepath).unwrap_or_else(print_err);

    let result = format_text(&code, syntax, &options).unwrap_or_else(print_err);
    print!("{}", result)
}
