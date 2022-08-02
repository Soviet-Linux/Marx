use scraper::Html;
use crate::tui;

#[tokio::main]
async fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;

    let body = res.text().await?;

    Ok(body.to_owned())
}

fn parse_html(html: String, selector: String) -> Vec<String> {
    let parsed_html = Html::parse_document(&html);
    let title_selector = scraper::Selector::parse(&selector).unwrap();
    let titles = parsed_html.select(&title_selector).map(|x| x.inner_html());
    let mut title_vec = Vec::new();

    titles
        .zip(1..101)
        .for_each(|(item, _number)| title_vec.push(item.replace(" ", "").replace("\n", "")));

    return title_vec;
}



pub fn get_aur(search_term: String) {
    let html = get_html(&format!(
        "{}{}",
        &"https://aur.archlinux.org/packages?O=0&K=".to_owned(),
        search_term.as_str()
    ));
    let title_vec = &parse_html(html.as_ref().unwrap().to_string(), "td>a".to_owned());
    let url_vec = &parse_html(html.unwrap().to_string(), "td>a[href]".to_owned());

    tui(title_vec.to_vec(), url_vec.to_vec(), "aur"); 
}
