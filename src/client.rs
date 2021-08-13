use reqwest;

use crate::global_config::*;

pub struct Client {
    user: String,
    session: reqwest::Client,
}
