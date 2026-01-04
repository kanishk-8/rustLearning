use std::{fs, path::PathBuf};

use clap::Parser;
use maud::{Markup, html};
use pulldown_cmark::{Options, Parser as MarkdownParser, html};

#[derive(Debug, Parser)]
struct Args {
    /// Input markdown file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output HTML file path (optional)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn render_html_page(content: &str) -> Markup {
    html! {
        (maud::DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "Markdown to HTML" }
            }
            body {
                (maud::PreEscaped(content.to_string()))
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let markdown_input = fs::read_to_string(&args.input).expect("failed to read the input path");

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let markdown_parser = MarkdownParser::new_ext(&markdown_input, options);
    let mut html_output = String::new();

    html::push_html(&mut html_output, markdown_parser);
    let full_html = render_html_page(&html_output).into_string();

    match &args.output {
        Some(path) => fs::write(path, full_html).expect("Failed to write"),
        None => println!("{}", full_html),
    }
}
