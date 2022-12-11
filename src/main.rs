use error_chain::error_chain;

error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
} 

use log::error;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cep = match std::env::args().nth(1) {
        Some(valor) => valor,
        None => {
            error!("Parâmetro não foi informado cargo run <CEP>\n");
            panic!();
        }
    };
    
    let url = format!(
        "https://viacep.com.br/ws/{}/json/", cep);

    let res = reqwest::get(url).await?;
        
    println!("Status {}", res.status() );
    println!("Headerss \n {:#?}", res.headers());
    
    let body = res.text().await?;
    println!("Body \n {:#?}", body);
    
    let cidade: Value = serde_json::from_str(&body).unwrap();
    let localidade = cidade["localidade"].as_str().unwrap();
    println!("Localidade \n {}", localidade);


    Ok(())
}