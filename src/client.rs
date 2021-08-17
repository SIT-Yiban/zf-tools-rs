use reqwest;

use crate::global_config::*;
use crate::parsers::SchoolYear;

pub struct Client {
    user: String,
    session: reqwest::Client,
}

pub trait Environment {
    fn get_major_list(&self, entrance_year: SchoolYear);
}
