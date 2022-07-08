use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};


use scraper::Html;
extern crate open;

fn main() {
    
    let search_term: String = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Search term:")
    .interact_text()
    .unwrap();
    

    let html = get_html(&search_term);
    let title_vec = parse_html(&html.as_ref().unwrap(), "h2.result__title>a");
    let url_vec = parse_html(&html.unwrap(), "a.result__url");

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
    
    if open::that(&url_vec[index_element].replace(" ", "").trim().to_owned()).is_ok() {
        println!("{}", &url_vec[index_element].replace(" ", "").trim().to_owned());
    }else if open::that(&url_vec[index_element].replace(" ", "").trim().to_owned()).is_err() {
        println!("Could not open the url: {}", &url_vec[index_element].replace(" ", "").to_owned());
        panic!();
    }
        


}

#[tokio::main]
async fn get_html(search_term: &str) -> Result<String, reqwest::Error> {
    let url = "https://html.duckduckgo.com/html/search?q=".to_owned() + search_term;
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
        .for_each(|(item, _number)| title_vec.push(item));

    return title_vec;
}

        
    
