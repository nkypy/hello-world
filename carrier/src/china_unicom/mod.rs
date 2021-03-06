mod controllers;
mod models;

use std::str::FromStr;

use chrono::{Duration, TimeZone, Utc};
use isahc::ResponseExt;

use crate::china_unicom::models::CardReply;
use crate::{CardInfo, CardStatus, CardUsage, CarrierClient, Result};

static API_REST_URL: &str = "https://api.10646.cn/rws/api/v1/";

// 联通帐号密码信息
#[derive(Debug)]
pub struct ChinaUnicomClient {
    pub username: String,
    pub password: String,
    pub soap_license: String,
    pub rest_license: String,
    pub rest_auth: String,
}

impl ChinaUnicomClient {
    pub fn new(username: &str, password: &str, soap_license: &str, rest_license: &str) -> Self {
        let rest_auth: String = dbg!(base64::encode(
            format!("{}:{}", username, rest_license).as_bytes()
        ));
        ChinaUnicomClient {
            username: username.to_owned(),
            password: password.to_owned(),
            soap_license: soap_license.to_owned(),
            rest_license: rest_license.to_owned(),
            rest_auth: rest_auth,
        }
    }
    pub fn get(&self, url: &str) -> Result<String> {
        let url = dbg!(format!("{}{}", API_REST_URL, url));
        dbg!(Ok(crate::http_client()?
            .get(&url)
            .header("Authorization", format!("Basic {}", self.rest_auth))
            .header("Content-Type", "application/json")
            .send()?
            .text()?))
    }
    // pub fn aync_get(&self, url: &str) -> Future<Output = std::result::Result<&'static str, &'static str>> {
    //     let url = dbg!(format!("{}{}", API_REST_URL, url));
    //     dbg!(Ok(crate::async_http_client()?
    //         .get(&url)
    //         .header("Authorization", format!("Basic {}", self.rest_auth))
    //         .header("Content-Type", "application/json")
    //         .send()?
    //         .text()?))
    // }
    pub fn put(&self, url: &str, data: &str) -> String {
        "put".to_string()
    }
}

impl CarrierClient for ChinaUnicomClient {
    fn card_status(&self, iccid: &str) -> Result<CardStatus> {
        let resp = self.get(format!("devices/{}", iccid).as_str())?;
        Ok(CardReply::from_str(&resp)?.into())
    }
    fn card_info(&self, iccid: &str) -> Result<CardInfo> {
        let resp = self.get(format!("devices/{}", iccid).as_str())?;
        Ok(CardReply::from_str(&resp)?.into())
    }
    fn card_usage(&self, iccid: &str, month: &str) -> Result<CardUsage> {
        let dt =
            Utc.datetime_from_str(format!("{}01 09:00:00", month).as_str(), "%Y%m%d %H:%M:%S")?;
        let resp = self.get(
            format!(
                "devices/{}/usageInZone?cycleStartDate={}-27Z",
                iccid,
                (dt - Duration::days(7)).format("%Y-%m")
            )
            .as_str(),
        )?;
        Ok(CardReply::from_str(&resp)?.into())
    }
}
