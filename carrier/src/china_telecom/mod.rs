mod model;

use crate::CarrierClient;

const API_GET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/query.do";
const API_SET_URL: &str = "http://api.ct10649.com:9001/m2m_ec/app/serviceAccept.do";

#[derive(Debug)]
pub struct ChinaTelecomClient<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub license: &'a str,
}

impl<'a> ChinaTelecomClient<'a> {
    pub fn hash(&self, mut data: Vec<&str>) -> String {
        data.sort();
        data.join(",")
    }
}

impl<'a> CarrierClient<'a> for ChinaTelecomClient<'a> {
    fn card_status(&self, iccid: &str) -> String {
        "card_status".to_string()
    }
    fn card_online(&self, iccid: &str) -> String {
        "card_online".to_string()
    }
    fn card_info(&self, iccid: &str) -> String {
        "card_info".to_string()
    }
    fn card_usage(&self, iccid: &str) -> String {
        "card_usage".to_string()
    }
    fn card_plan(&self, iccid: &str) -> String {
        "card_plan".to_string()
    }
}