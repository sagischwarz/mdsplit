use std::{fs, io};

use chrono::NaiveDate;
use regex::RegexBuilder;
use structopt::StructOpt;

fn main() -> io::Result<()> {
    let args: Cli = Cli::from_args();
    let file_content = fs::read_to_string(&args.file_path)?;

    #[allow(clippy::trivial_regex)]
    let seperator = RegexBuilder::new(r"^# ")
        .multi_line(true)
        .build()
        .expect("Invalid regex.");

    let entries: Vec<_> = seperator.split(&file_content).into_iter().collect();

    for entry in entries {
        let date_line =
            translate_month(&entry.lines().next().unwrap().split(" der ").last().unwrap());

        if date_line.starts_with("//") {
            continue;
        }

        let parsed = NaiveDate::parse_from_str(&date_line, "%d. %B %Y")
            .unwrap_or_else(|_| panic!("{}", &date_line));

        let result_path = args.file_path.parent().unwrap().join("result");

        std::fs::create_dir_all(&result_path)?;

        fs::write(
            &result_path.join(format!("{}.md", &parsed.to_string())),
            format!("# {}", &entry),
        )?;
    }

    Ok(())
}

fn translate_month(date: &str) -> String {
    date.replace("Januar", "January")
        .replace("Februar", "February")
        .replace("März", "March")
        .replace("Mai", "May")
        .replace("Juni", "June")
        .replace("Juli", "July")
        .replace("Oktober", "October")
        .replace("Dezember", "December")
}

#[derive(StructOpt)]
struct Cli {
    /// The markdown file to parse
    #[structopt(parse(from_os_str))]
    file_path: std::path::PathBuf,
}
