use dotenvy::dotenv;
use reqwest::Client;
use std::env;
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use thirtyfour::prelude::*;
use tokio;
use url::Url;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    dotenv().ok();

    println!("Start download");

    let login_url = "https://jpsummit-smp24.awsevents.com/public/mypage?lang=ja";
    let download_page_url = "https://jpsummit-smp24.awsevents.com/public/application/add/273";
    let username = env::var("USERNAME").expect("USERNAME not set");
    let password = env::var("PASSWORD").expect("PASSWORD not set");

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    // Open the login page
    driver.goto(login_url).await?;

    // Find the username and password fields and enter the credentials
    let username_field = driver.find(By::Name("login_id")).await?;
    username_field.send_keys(&username).await?;
    let password_field = driver.find(By::Name("login_password")).await?;
    password_field.send_keys(&password).await?;

    // Find and click the login button
    let login_button = driver
        .find(By::XPath(
            "//button[contains(@class, 'btn-primary') and text()='ログイン']",
        ))
        .await?;
    login_button.click().await?;

    driver
        .find(By::XPath("//a[@title='資料一覧サイト']"))
        .await?;

    println!("success login and move download page");

    driver.goto(download_page_url).await?;

    // Find all download links
    let links = driver
        .find_all(By::XPath(
            "//a[contains(@href, '.pdf') or contains(@href, '.zip')]",
        ))
        .await?;

    println!("Found {} downloadable links on the page.", links.len());

    // Create a directory for downloads if it doesn't exist
    fs::create_dir_all("downloads").expect("Failed to create downloads directory");

    let client = Client::new();

    // Download each file
    for link in links {
        let href = link.attr("href").await?.expect("No href attribute");
        let url = Url::parse(&href).expect("Failed to parse URL");
        let file_name = url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("unknown");
        let file_path = Path::new("downloads").join(file_name);

        let response = client
            .get(&href)
            .send()
            .await
            .expect("Failed to GET from URL");
        let mut dest = File::create(&file_path).expect("Failed to create file");
        let resp_file = response
            .bytes()
            .await
            .expect("Failed to get the full response body as Bytes");
        copy(&mut resp_file.as_ref(), &mut dest).expect("Failed to copy content");

        println!("Downloaded: {}", file_name);
    }

    println!("All files downloaded.");

    // Close the browser
    driver.quit().await?;

    Ok(())
}
