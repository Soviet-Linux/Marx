use crate::tui;
use serde::*;
use serde::Deserialize;
use simsearch::SimSearch;

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    all: Vec<package>
}
#[derive(Deserialize, Serialize, Debug)]
struct package {
    name: String,
    #[serde(rename = "type")]
    type_lol: String,
    version: String
}



pub fn get_our(search_term: String) {

    let mut title_vec = vec![];
    let mut url_vec = vec![]; 


    let json = get_json().unwrap();

    for pkg in json.all {
        let pkg_name = pkg.name;
        title_vec.push(pkg_name.clone());
        url_vec.push(pkg_name);
    }

    let search = &search_vec(title_vec, search_term);

    tui(search.to_vec(), search.to_vec(), "our"); 
    //println!("{:#?}", search_vec(title_vec, search_term)[0].trim());

    vec!["".to_owned(), "".to_owned()];
}

fn search_vec(title_vec: Vec<String>, search_term: String) -> Vec<String> {

    let mut engine = SimSearch::new();

    let count = 0;
    for title in &title_vec {
        if !count > title_vec.len() {
            println!("{}", count);
            engine.insert(format!("{}", &title), &title);
        }
    }

    let results = engine.search(&search_term);

    println!("{:#?}", results);

    results

}


#[tokio::main()]
async fn get_json() -> Result<Response, reqwest::Error> {
    let json: Response = reqwest::get("https://our.sovietlinux.ml/all.json").await?.json().await?;

    Ok(json)
}
