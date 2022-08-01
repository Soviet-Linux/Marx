use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use scraper::Html;
use args::Action;
use clap::Parser;

mod args;
mod utils; 

const VERSION : &str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR : &'static str = env!("CARGO_PKG_AUTHORS");

/// Python Project Manager
#[derive(Parser, Debug)]
#[clap(author=AUTHOR, version=VERSION, about=ABOUT, long_about = None)]
struct Cli {

    #[clap(subcommand)]
    command: Action,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Action::Install(add_pro) => add_pro.install(),
        Action::Unninstall(rp) => rp.uninstall(),
        Action::Tui => tui(),
    }
}

fn tui() {
    let search_term: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Search term:")
        .interact_text()
        .unwrap();

    let html = get_html(&format!(
        "{}{}",
        &"https://aur.archlinux.org/packages?O=0&K=".to_owned(),
        search_term.as_str()
    ));
    let title_vec = parse_html(&html.as_ref().unwrap(), "td>a");
    let url_vec = parse_html(&html.unwrap(), "td>a[href]");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Results")
        .default(0)
        .items(&title_vec[..])
        .interact()
        .expect("You are ratelimited");

    let index_element = title_vec
        .iter()
        .position(|x| x == &title_vec[selection].to_owned())
        .unwrap();

    let _ = utils::call_cccp_single(&url_vec[index_element]);
}

#[tokio::main]
async fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;

    let body = res.text().await?;

    Ok(body.to_owned())
}

fn parse_html(html: &str, selector: &str) -> Vec<String> {
    let parsed_html = Html::parse_document(html);
    let title_selector = scraper::Selector::parse(selector).unwrap();
    let titles = parsed_html.select(&title_selector).map(|x| x.inner_html());
    let mut title_vec = Vec::new();

    titles
        .zip(1..101)
        .for_each(|(item, _number)| title_vec.push(item.replace(" ", "").replace("\n", "")));

    return title_vec;
}


