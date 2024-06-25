# AWS Summit File Downloader in Rust

> [!IMPORTANT]  
> Inspired by: [Kei-Ta/aws-summit-download](https://github.com/Kei-Ta/aws-summit-download)

This project provides an application that logs into the AWS Summit website, navigates to a specified page, and downloads all PDF and ZIP files linked on that page.

The application uses Selenium WebDriver for browser automation.

## Features

- Automatically logs into the AWS Summit website.
- Navigates to the specified page and searches for all PDF and ZIP files to download.
- Saves the downloaded files to a local directory.

## Usage

### Configuration

Create a .env file and write the login credentials.

```shell
touch .env
```

```
USERNAME=your_username
PASSWORD=your_password
```

### Run application

The application assume you have chromedriver running on your system.

You can use Selenium (see instructions below) or you can use chromedriver directly by downloading the chromedriver that matches your Chrome version, from here: [https://chromedriver.chromium.org/downloads](https://chromedriver.chromium.org/downloads)

```shell
chromedriver
cargo run
```

## Notes

- Use your own login_id and login_password.
- This has only been confirmed in the owner's local environment, so it's not guaranteed to work in all environments. The downloadPageUrl may vary for each user.
- Please download responsibly.
- If warned by the organizers, this repository will be closed.
