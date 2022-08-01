use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::path::Path;

use scraper::Html;
extern crate open;

fn main() {
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
    //println!("{}", &url_vec[index_element].replace(" ", ""));
    let url_str = "https://aur.archlinux.org/packages/".to_owned()
        + &url_vec[index_element]
            .replace(" ", "")
            .replace("\n", "")
            .trim();

    let PKGBUILD = download_pkgbuild(url_vec[index_element].to_string());
    eprintln!("{:#?}", &PKGBUILD.as_ref().unwrap());
    println!(
        "{:#?}",
        format!(
            "~/packages/{}/{}",
            url_str.replace("https://aur.archlinux.org/packages/", ""),
            "PKGBUILD"
        )
    );
    write_file(
        format!(
            "~/packages/{}/PKGBUILD",
            url_str.replace("https://aur.archlinux.org/packages/", "")
        ),
        PKGBUILD.unwrap(),
    );

    if open::that(&url_str).is_ok() {
        println!("{}", &url_str);
    } else if open::that(&url_str).is_err() {
        println!("Could not open the url: {}", &url_str.to_owned());
        panic!();
    }
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

fn download_pkgbuild(pkg: String) -> Result<String, reqwest::Error> {
    return get_html(&format!(
        "{}{}",
        "https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=", pkg
    ));
}

fn write_file(mut path: String, data: String) {
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    if Path::new(&path).exists() {
        fs::remove_file(&path).unwrap();
    }

    File::create(&path).unwrap();
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(&path)
        .unwrap();

    if let Err(e) = writeln!(path, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
