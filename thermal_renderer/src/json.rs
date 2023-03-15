use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    #[serde(rename = "regex.bill.match")]
    pub regex_bill_match:Vec<String>,
    #[serde(rename = "regex.payway")]
    pub regex_payway:String,
    #[serde(rename = "regex.amount")]
    pub regex_amount:String,
    #[serde(rename = "regex.order.id")]
    pub regex_order_id:String
}