use clap::{Subcommand, Args};
use colored::Colorize;
use std::{fs, process::{Command}, io::{Write}, path::Path, time::Instant, collections::HashMap};

use crate::{utils, tui};

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Installs a package 
    Install(AddPackage),

    /// Unninstalls a package(NOT INPLEMENTED)
    Unninstall(RemovePackage),

    Tui,
}

#[derive(Args, Debug)]
pub struct AddPackage {

    /// List of packages to add
    pub pkg_names: Vec<String>,

}

impl AddPackage {

    pub fn install(&self) {
        utils::call_cccp(&self.pkg_names);
    }
    
}


#[derive(Args, Debug)]
pub struct RemovePackage {
    /// List of packages to remove
    pub pkg_names: Vec<String>,

}

impl RemovePackage {

    pub fn uninstall(&self) {
        
    }
    
}

#[derive(Args, Debug)]
pub struct Tui {
}

impl Tui {

    pub fn start(&self) {
        tui();
    }
    
}

