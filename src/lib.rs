use kuchiki::traits::*;
use kuchiki::{parse_html, NodeRef};
use reqwest::blocking::get;
use reqwest::Url;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn process_url(url: &str, tags: &[&str], classes: &[&str], output_dir: &str) {
    let response = get(url).expect("Failed to fetch URL");
    let content = response.text().expect("Failed to read response text");

    let document = parse_html().one(content);

    if !tags.is_empty() {
        for tag in tags {
            filter_tags(&document, &format!("{}", tag));
        }
    }

    if !classes.is_empty() {
        for class in classes {
            filter_classes(&document, class);
        }
    }

    save_filtered_html(&document, url, output_dir);
}

pub fn reverse_process_url(url: &str, tags: &[&str], classes: &[&str], output_dir: &str) {
    let response = get(url).expect("Failed to fetch URL");
    let content = response.text().expect("Failed to read response text");

    let document = parse_html().one(content);

    if !tags.is_empty() {
        for tag in tags {
            reverse_filter_tags(&document, &format!("{}", tag));
        }
    }

    if !classes.is_empty() {
        for class in classes {
            reverse_filter_classes(&document, class);
        }
    }

    save_filtered_html(&document, url, output_dir);
}

fn save_filtered_html(document: &NodeRef, url: &str, output_dir: &str) {
    let filtered_html = document.to_string();
    let filtered_html = remove_empty_lines(filtered_html);

    let output_path = generate_output_path(url, output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    let mut file = File::create(output_path.clone()).expect("Failed to create output file");
    file.write_all(filtered_html.as_bytes()).expect("Failed to write to file");

    println!("Filtered HTML for {} saved to {}", url, output_path);
}

fn filter_tags(document: &NodeRef, rule: &str) {
    let mut nodes_to_remove: Vec<NodeRef> = Vec::new();

    for element in document.select(rule).expect("Failed to select nodes") {
        nodes_to_remove.push(element.as_node().clone());
    }

    for node in nodes_to_remove {
        node.detach();
    }
}

fn filter_classes(document: &NodeRef, class: &str) {
    let mut nodes_to_remove: Vec<NodeRef> = Vec::new();

    for element in document.select("*").expect("Failed to select nodes") {
        if let Some(attr) = element.attributes.borrow().get("class") {
            let class_list: Vec<&str> = attr.split_whitespace().collect();
            if class_list.iter().any(|&c| c == class) {
                nodes_to_remove.push(element.as_node().clone());
            }
        }
    }

    for node in nodes_to_remove {
        node.detach();
    }
}

fn reverse_filter_tags(document: &NodeRef, rule: &str) {
    let mut nodes_to_keep: Vec<NodeRef> = Vec::new();

    for element in document.select(rule).expect("Failed to select nodes") {
        nodes_to_keep.push(element.as_node().clone());
    }

    for node in document.descendants().collect::<Vec<_>>() {
        if !nodes_to_keep.contains(&node) {
            node.detach();
        }
    }
}

fn reverse_filter_classes(document: &NodeRef, class: &str) {
    let mut nodes_to_keep: Vec<NodeRef> = Vec::new();

    for element in document.select("*").expect("Failed to select nodes") {
        if let Some(attr) = element.attributes.borrow().get("class") {
            let class_list: Vec<&str> = attr.split_whitespace().collect();
            if class_list.iter().any(|&c| c == class) {
                nodes_to_keep.push(element.as_node().clone());
            }
        }
    }

    for node in document.descendants().collect::<Vec<_>>() {
        if !nodes_to_keep.contains(&node) {
            node.detach();
        }
    }
}

fn remove_empty_lines(html: String) -> String {
    html.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn generate_output_path(url: &str, output_dir: &str) -> String {
    let uri = Url::parse(url).expect("Failed to parse URL");
    let domain = uri.host_str().unwrap_or("unknown_domain");
    let path = uri.path().trim_start_matches('/').replace('/', "_");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    format!("{}/{}_{}-{}.html", output_dir, domain, path, timestamp)
}

#[test]
fn filter_example() {
    let url = "https://itsfoss.com/ollama/";
    let output_dir = "output";

    //正向过滤
    let tags = vec!["script", "style", "link", "meta", "li", "desc", "title", "svg", "path", "dialog", "select", "head", "header", "foot", "footer", "ul", "nav", "button", "form", "input", "figure", "picture", "time", "h2", "h3", "h4", "i", "aside", "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child", ];
    let classes = vec!["progress-bar", "js-menu", "social-share", "post-info__readtime", "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc", "author-card", "related-posts"];

    process_url(url, &tags, &classes, output_dir);