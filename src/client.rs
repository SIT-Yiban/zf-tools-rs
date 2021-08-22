mod environment;
mod user;

use crate::global_config::USERAGENT;
use crate::session::Session;
use crate::Result;
use reqwest::header::{COOKIE, USER_AGENT};
use reqwest::Response;

#[derive(Debug)]
pub struct ZfClient {
    pub(crate) user: String,
    pub(crate) session: Session,
}

impl ZfClient {
    async fn get_url(&self, url: &str, data: &[(&str, String)]) -> Result<Response> {
        let response = self
            .session
            .client
            .get(url)
            .form(data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        Ok(response)
    }

    async fn post_url(&self, url: &str, data: &[(&str, String)]) -> Result<Response> {
        let response = self
            .session
            .client
            .post(url)
            .form(data)
            .header(USER_AGENT, USERAGENT)
            .header(COOKIE, self.session.get_cookie_string("jwxt.sit.edu.cn"))
            .send()
            .await?;
        Ok(response)
    }
}
