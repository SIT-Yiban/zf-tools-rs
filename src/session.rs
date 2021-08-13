use base64::{decode, encode};
use rand::rngs::OsRng;
use reqwest::Client;
use regex::Regex;
use reqwest::header::USER_AGENT;
use rsa::{BigUint, PaddingScheme, PublicKey, RsaPublicKey};

use crate::global_config::*;


#[derive(Clone)]
pub struct Session {
    pub(crate) user: String,
    pub(crate) passwd: String,
    pub(crate) client: Client,
    pub(crate) login_flag: bool,
}

#[derive(Default)]
pub struct SessionBuilder {
    user: Option<String>,
    passwd: Option<String>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        SessionBuilder::default()
    }

    pub fn user<T: ToString>(mut self, user: T) -> Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn passwd<T: ToString>(mut self, passwd: T) -> Self {
        self.passwd = Some(passwd.to_string());
        self
    }

    pub fn build(self) -> Session {
        Session {
            user: self.user.unwrap_or_else(|| {
                panic!("User is required in SessionBuilder, please call user method.")
            }),
            passwd: self.passwd.unwrap_or_else(|| {
                panic!("Passwd is required in SessionBuilder, please call passwd method.")
            }),
            client: Client::new(),
            login_flag: false,
        }
    }
}

pub async fn encrypt_in_rsa(
    message: Vec<u8>,
    public_key: Vec<u8>,
    exponent: Vec<u8>,
) -> anyhow::Result<String> {
    let key = BigUint::from_bytes_be(public_key.as_slice());
    let exp = BigUint::from_bytes_be(exponent.as_slice());
    let mut rng = OsRng;
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let publickey = RsaPublicKey::new(key, exp)?;
    let enc_data = publickey
        .encrypt(&mut rng, padding, message.as_slice())
        .expect("failed to encrypt");
    Ok(encode(enc_data))
}

impl Session {
    pub async fn get_ras_public_key(&self) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
        #[derive(Debug, serde::Deserialize)]
        struct RsaPublicKey {
            modulus: String,
            exponent: String,
        }

        let resp = self
            .client
            .get(url::RSA_PUBLIC_KEY)
            .header(USER_AGENT, USERAGENT)
            .send()
            .await?;

        let public_key = resp.json::<RsaPublicKey>().await?;
        let modulus = decode(public_key.modulus)?;
        let exponent = decode(public_key.exponent)?;

        Ok((modulus, exponent))
    }

    pub async fn get_csrf_token(&self, login_page: String) -> anyhow::Result<String> {
        let re = Regex::new(
            "<input type=\"hidden\" id=\"csrftoken\" name=\"csrftoken\" value=\"(.*)\"/>",
        )
        .unwrap();
        let text = login_page.as_str();
        let token_tag = re.captures(text).unwrap();
        let token = &token_tag[2];
        Ok(token.to_string())
    }

    pub async fn get_err_message(&self, content: String) -> String {
        use scraper::{Html, Selector};
        let document = Html::parse_document(content.as_str());
        let err_node = document
            .select(
                &Selector::parse("div#home.tab-pane.in.active p#tips.bg_danger.sl_danger").unwrap(),
            )
            .next()
            .unwrap()
            .inner_html();
        err_node
    }
    pub async fn login(&mut self) -> anyhow::Result<(String)> {
        // Get login page for the first cookie
        let login_page = self
            .client
            .get(url::HOME)
            .header(USER_AGENT, USERAGENT)
            .send()
            .await?;
        if let Ok((public_key, exponent)) = self.get_ras_public_key().await {
            let message = encode(self.passwd.clone());
            let encrypted_passwd =
                encrypt_in_rsa(message.into_bytes(), public_key, exponent).await?;
            let token = self.get_csrf_token(login_page.text().await?).await?;
            let params = [
                ("csrftoken", token.as_str()),
                ("language", "zh_CN"),
                ("yhm", &self.user.clone()),
                ("mm", &encrypted_passwd),
            ];
            let res = self
                .client
                .post(url::LOGIN)
                .header(USER_AGENT, USERAGENT)
                .form(&params)
                .send()
                .await?;
            return if res.url().to_string().starts_with(url::LOGIN) {
                let error = self.get_err_message(res.text().await?).await;
                Err(anyhow::anyhow!("Session error : {:?}.", error))
            } else {
                self.login_flag = true;
                Ok("success".to_string())
            };
        }
        Err(anyhow::anyhow!("Can't get public key"))
    }
}
