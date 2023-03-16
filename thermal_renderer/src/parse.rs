use encoding::{all::UTF_8, Encoding, DecoderTrap};
use regex::Regex;
use crate::json::Setting;

pub fn decode(config_path:&str){
    let mut rbm: Regex = Regex::new("").unwrap();
    let mut rp: Regex = Regex::new("").unwrap();
    let mut ra: Regex = Regex::new("").unwrap();
    let mut roi: Regex = Regex::new("").unwrap();
    match std::fs::read(config_path) {
        Ok(buf) =>{
            match serde_json::from_slice::<Setting>(&buf[..]) {
                Ok(setting) => {
                    rbm = Regex::new(&setting.regex_bill_match[0]).unwrap();
                    rp = Regex::new(&setting.regex_payway).unwrap();
                    ra = Regex::new(&setting.regex_amount).unwrap();
                    roi = Regex::new(&setting.regex_order_id).unwrap();
                }
                Err(err) => {
                    println!("error: {:?}", err)
                }
            }
        }
        Err(err) => {
            println!("读取配置文件失败: {:?}", err);
            return
        }
        
    }
    
    let mut content_buf = Vec::new();
    match std::fs::read("output.txt"){
        Ok(content) => {
            content_buf = content;
        }
        Err(err) => {
            println!("读取文件失败: {:?}", err);
            return 
        }
    }
    
    match UTF_8.decode(&content_buf, DecoderTrap::Replace) {
        Ok(ret) => {
            println!("账单内容:{}", ret);
            println!("====================================================");
            println!("名   称:     {}", rbm);
            println!("支   付:     {}", rp);
            println!("金   额:     {}", ra);
            println!("单   号:     {}", roi);
            println!("====================================================");
            println!("");
            if let Some(caps) = rbm.captures(ret.as_str()) {
                if caps.len() > 0 {
                    let bill_match = &caps[0];
                    println!("小票抬头获取成功:     {}", bill_match);
                }
            } else {
                println!("小票抬头没有获取！")
            }
            if let Some(caps) = rp.captures(ret.as_str()) {
                if caps.len() > 0 {
                    let payway = &caps[caps.len() - 1];
                    println!("获取到支付方式:       {}", payway);
                }
            } else {
                println!("支付方式没有获取！")
            }
            if let Some(caps) = ra.captures(ret.as_str()) {
                if caps.len() > 0 {
                    let amount = &caps[caps.len() - 1];
                    println!("获取到金额:           {}", amount);
                }
            } else {
                println!("金额没有获取！")
            }
            if let Some(caps) = roi.captures(ret.as_str()) {
                if caps.len() > 0 {
                    let order_id = &caps[caps.len() - 1];
                    println!("获取到单号:          {}", order_id);
                }
            } else {
                println!("单号没有获取！")
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
