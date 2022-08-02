use dialoguer::Input;
use dialoguer::{theme::ColorfulTheme, Select};
use minparse::minparse;
mod args;
mod utils;
mod aur;
mod our;


const VERSION : &str = env!("CARGO_PKG_VERSION");
const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR : &'static str = env!("CARGO_PKG_AUTHORS");


fn main() {

    let subcommands = minparse::subcommands();
    let flags = minparse::switches();
    if subcommands.contains(&"tui".to_owned()) {
    start_tui(flags);
    }

}

fn start_tui(flags: Vec<String>) {
    let search_term: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Search term:")
        .interact_text()
        .unwrap();

    let flags = minparse::switches();
    println!("{:#?}", flags);
     if flags.contains(&"--aur".to_owned()) {
        aur::get_aur(search_term)
     }else if flags.contains(&"--our".to_owned()) {
         our::get_our(search_term);
     }
}

pub fn tui(title_vec: Vec<String>, url_vec: Vec<String>, type_lol: &str) {

   
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
     
    println!("{:#?}", &url_vec[index_element]);
    if type_lol == "aur" {
    let _ = utils::call_cccp_single_aur(&url_vec[index_element].trim());
    }else if type_lol == "our" {
        utils::call_cccp_single(&url_vec[index_element].replace("\"", "").trim());
    }
    }


