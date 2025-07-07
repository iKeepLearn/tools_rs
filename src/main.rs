use clap::{Arg, Command as ClapCommand};
use regex::Regex;
use reqwest::{Proxy, blocking::Client, redirect::Policy};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
    path::PathBuf,
    process::Command,
};

fn main() {
    let matches = ClapCommand::new("image_downloader")
        .version("1.0")
        .author("CC")
        .about("Downloads images from a webpage")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("Sets the URL to fetch images from")
                .conflicts_with("html")
                .required_unless_present("html"),
        )
        .arg(
            Arg::new("html")
                .short('H')
                .long("html")
                .value_name("FILE")
                .help("Sets the HTML file to parse for images")
                .conflicts_with("url")
                .required_unless_present("url"),
        )
        .arg(
            Arg::new("dir")
                .short('d')
                .long("dir")
                .value_name("DIRECTORY")
                .help("Sets the output directory for downloaded images")
                .default_value("img"),
        )
        .arg(
            Arg::new("proxy")
                .short('p')
                .long("proxy")
                .value_name("PROXY_URL")
                .help("Sets the proxy server to use (e.g., http://proxy.example.com:8080)"),
        )
        .arg(
            Arg::new("user_agent")
                .short('a')
                .long("user-agent")
                .value_name("USER_AGENT")
                .help("Sets the User-Agent header for HTTP requests")
                .default_value("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36"),
        )
        .arg(
            Arg::new("aria2c_proxy")
        .long("aria2c_proxy")
        .help("aria2c apply proxy")
        .action(clap::ArgAction::SetTrue)
        .default_value("false"),
)
        .get_matches();

    let proxy = matches.get_one::<String>("proxy");
    let aria2c_proxy = matches.get_flag("aria2c_proxy");
    let html = if let Some(url) = matches.get_one::<String>("url") {
        let user_agent = matches.get_one::<String>("user_agent").unwrap();
        let client = build_http_client(proxy, user_agent);
        println!("get html content...");
        let response = client
            .get(url)
            .header(
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36",
            )
            .send()
            .expect("Failed to send request");

        response.text().expect("Failed to read response text")
    } else if let Some(html_path) = matches.get_one::<String>("html") {
        let mut file = File::open(html_path).expect("Failed to open HTML file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read HTML file");
        contents
    } else {
        eprintln!("Either --url or --html must be specified");
        std::process::exit(1);
    };

    let image_urls = get_url_list(&html);
    let output_dir = matches.get_one::<String>("dir").unwrap();

    download_images(&image_urls, output_dir, proxy, aria2c_proxy);
}

fn build_http_client(proxy_url: Option<&String>, user_agent: &str) -> Client {
    let mut builder = Client::builder()
        // .https_only(true)
        .timeout(std::time::Duration::from_secs(10))
        .redirect(Policy::limited(5))
        .user_agent(user_agent);

    if let Some(proxy) = proxy_url {
        builder = builder.proxy(Proxy::all(proxy).unwrap_or_else(|_| {
            eprintln!("Invalid proxy URL: {}", proxy);
            std::process::exit(1);
        }));
    }

    builder.build().unwrap_or_else(|e| {
        eprintln!("Failed to create HTTP client: {}", e);
        std::process::exit(1);
    })
}

fn get_url_list(html: &str) -> Vec<String> {
    let regex = Regex::new(r"(http(s?)://)([/|.|\w|\s|-])*\.(?:jpg|gif|png|JPG|GIF|PNG|webp|WEBP)")
        .expect("Failed to compile regex");

    let mut seen = HashSet::new();
    let mut unique_urls = Vec::new();

    for cap in regex.captures_iter(html) {
        if let Some(matched) = cap.get(0) {
            let url = matched.as_str().to_string();
            if seen.insert(url.clone()) {
                unique_urls.push(url);
            }
        }
    }

    unique_urls
}

fn download_images(urls: &[String], output_dir: &str, proxy: Option<&String>, aria2c_proxy: bool) {
    let expanded_dir = PathBuf::from(output_dir);
    if !expanded_dir.exists() {
        fs::create_dir_all(&expanded_dir).expect("Failed to create output directory");
    }

    let total = urls.len();
    for (i, url) in urls.iter().enumerate() {
        let filename = url.split('/').last().unwrap_or("unknown");
        let file_path = expanded_dir.join(filename);

        println!(
            "Downloading {}/{} file name: {}",
            i + 1,
            total,
            file_path.display()
        );

        let mut cmd = Command::new("aria2c");
        cmd.arg("-o")
            .arg(&filename)
            .arg("-d")
            .arg(&expanded_dir)
            .arg(url);

        if let Some(proxy_url) = proxy {
            if aria2c_proxy {
                cmd.arg("--all-proxy").arg(proxy_url);
            }
        }
        match cmd.status() {
            Ok(status) if !status.success() => {
                eprintln!("Failed to download {}", url);
            }
            Err(e) => {
                eprintln!("Failed to execute aria2c for {}: {}", url, e);
            }
            _ => {}
        }
    }
}
