// language file extention API

use std::collections::HashMap;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};

#[get("/lang/{lang_name}")]
async fn lang(web::Path(lang_name): web::Path<String>) -> impl Responder { 
    println!("{}", lang_name);
    let mut lang_dict = HashMap::new();  // lang name: lang file extention
    lang_dict.insert(String::from("c"), String::from("c"));
    lang_dict.insert(String::from("cpp"), String::from("cpp"));
    lang_dict.insert(String::from("haskell"), String::from("hs"));
    lang_dict.insert(String::from("java"), String::from("java"));
    lang_dict.insert(String::from("javascript"), String::from("js"));
    lang_dict.insert(String::from("julia"), String::from("jl"));
    lang_dict.insert(String::from("kotlin"), String::from("kt"));
    lang_dict.insert(String::from("lua"), String::from("lua"));
    lang_dict.insert(String::from("nim"), String::from("nim"));
    lang_dict.insert(String::from("perl"), String::from("pl"));
    lang_dict.insert(String::from("php"), String::from("php"));
    lang_dict.insert(String::from("python"), String::from("py"));
    lang_dict.insert(String::from("ruby"), String::from("rb"));
    lang_dict.insert(String::from("rust"), String::from("rs"));
    lang_dict.insert(String::from("scala"), String::from("scala"));
    lang_dict.insert(String::from("typescript"), String::from("ts"));
    
    if lang_name == "all" {
        let mut all = "".to_string();
        for (k, v) in &lang_dict {
            all = format!("{}\n    {{\"lang_name\": \"{}\", \"lang_file_extension\": \"{}\"}},", all, k, v);
        }
        HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{{{}\n}}", all))
    } else {
        let lang_file_extention = lang_dict.get(&lang_name).unwrap();
    
        HttpResponse::Ok()
            .content_type("application/json")
            .body(format!("{{\"lang_name\": \"{}\", \"lang_file_extention\": \"{}\"}}", lang_name, lang_file_extention))   
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(lang))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

