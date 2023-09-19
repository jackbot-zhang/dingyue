// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use base64::{Engine as _, engine::general_purpose};
use std::string::String;
use serde::{Deserialize, Serialize};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_proxies])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
struct Proxy {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    server: String,
    port: i32,
    cipher: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uuid: Option<String>,
    #[serde(rename = "alterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    alter_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network: Option<String>,
}

#[derive(Deserialize)]
struct Vmess {
    port: String,
    id: String,
    add: String,
}

#[derive(Serialize)]
struct Proxies {
    proxies: Vec<Proxy>,
}

#[tauri::command]
fn get_proxies() -> Result<String, String> {
    match get_proxies1() {
        Ok(msg) => { Ok(msg) }
        Err(e) => { Err(e.to_string()) }
    }
}

fn get_proxies1() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get("https://jmssub.net/members/getsub.php?service=131783&id=a35aa5ea-5893-41bc-86c9-5d283bd9cd68")?
        .text()?;


    let str = decode_base64(body)?;
    let v: Vec<&str> = str.split("\n").collect();
    let mut proxys = Proxies { proxies: vec![] };
    let mut sscnt = 1;
    let mut vmesscnt = 1;
    for item in v {
        if item.starts_with("ss://") {
            let x = item.strip_prefix("ss://").unwrap();
            let p: Vec<&str> = x.split("#").collect();

            let data = decode_base64_no_pad(p[0])?;
            let proxy = parse_ss_proxy(data.as_str(), &mut sscnt);

            proxys.proxies.push(proxy);
            sscnt += 1;
        } else {
            let x = item.strip_prefix("vmess://").unwrap();

            let data = decode_base64_no_pad(x)?;
            let proxy = parse_vmess_proxy(&data, &mut vmesscnt);
            proxys.proxies.push(proxy);
            vmesscnt += 1;
        }
    }
    Ok(serde_yaml::to_string(&proxys)?)
}

fn parse_ss_proxy(data: &str, sscnt: &mut i32) -> Proxy {
    let parts: Vec<&str> = data.split("@").collect();
    let server_info: Vec<&str> = parts[0].split(":").collect();
    let password = server_info.get(1).map(|&s| s.to_string());

    let address_info: Vec<&str> = parts[1].split(":").collect();
    let port = address_info[1].parse::<i32>().unwrap();

    Proxy {
        name: format!("ss{}", sscnt),
        typ: String::from("ss"),
        server: address_info[0].to_string(),
        port,
        cipher: server_info[0].to_string(),
        password,
        uuid: None,
        alter_id: None,
        network: None,
    }
}

fn parse_vmess_proxy(data: &str, vmesscnt: &mut i32) -> Proxy {
    let parsed: Vmess = serde_json::from_str(data).unwrap();
    let port = parsed.port.parse::<i32>().unwrap();

    Proxy {
        name: format!("vmess{}", vmesscnt),
        typ: String::from("vmess"),
        server: parsed.add,
        port,
        cipher: "auto".to_string(),
        password: None,
        uuid: Some(parsed.id.to_string()),
        alter_id: Some("0".to_string()),
        network: Some("tcp".to_string()),
    }
}

fn decode_base64_no_pad(input: &str) -> Result<String, base64::DecodeError> {
    let mut buffer = Vec::<u8>::new();
    general_purpose::STANDARD_NO_PAD
        .decode_vec(input.as_bytes(), &mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn decode_base64(input: String) -> Result<String, base64::DecodeError> {
    let mut buffer = Vec::<u8>::new();
    general_purpose::STANDARD
        .decode_vec(input.as_bytes(), &mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}


#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_get() {
        let res = get_proxies();
        match res {
            Ok(x) => print!("{}", x),
            Err(e) => print!("err : {}", e),
        }
    }
}
