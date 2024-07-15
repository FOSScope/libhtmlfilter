use kuchiki::traits::*;
use kuchiki::{parse_html, NodeRef};
use reqwest::{get, Url};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

/// 获取过滤后的HTML内容
pub async fn get_filtered_html(url: &str, tags: &[&str], classes: &[&str]) -> String {
    let response = get(url).await.expect("Failed to fetch URL");
    let content = response.text().await.expect("Failed to read response text");

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

    let filtered_html = document.to_string();
    remove_empty_lines(filtered_html)
}

/// 获取过滤后的HTML内容，并更新相对路径为绝对路径
pub async fn get_filtered_html_fullurl(url: &str, tags: &[&str], classes: &[&str]) -> String {
    let response = get(url).await.expect("Failed to fetch URL");
    let content = response.text().await.expect("Failed to read response text");

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

    // 更新相对路径为绝对路径
    update_relative_paths(&document, url);

    let filtered_html = document.to_string();
    remove_empty_lines(filtered_html)
}

/// 获取过滤后的HTML内容，更新相对路径为绝对路径，并移除URL中的引用参数
pub async fn get_filtered_html_fullurl_removeref(url: &str, tags: &[&str], classes: &[&str]) -> String {
    let filtered_html = get_filtered_html_fullurl(url, tags, classes).await;
    let document = parse_html().one(filtered_html);
    remove_ref_from_urls(&document);
    let filtered_html = document.to_string();
    remove_empty_lines(filtered_html)
}

/// 处理URL并保存过滤后的HTML内容
pub async fn process_url(url: &str, tags: &[&str], classes: &[&str], output_dir: &str) {
    let filtered_html = get_filtered_html(url, tags, classes).await;
    save_filtered_html(&filtered_html, url, output_dir);
}

/// 处理URL并保存过滤后的HTML内容，更新相对路径为绝对路径
pub async fn process_url_full(url: &str, tags: &[&str], classes: &[&str], output_dir: &str) {
    let filtered_html = get_filtered_html_fullurl(url, tags, classes).await;
    save_filtered_html(&filtered_html, url, output_dir);
}

/// 处理URL并保存过滤后的HTML内容，更新相对路径为绝对路径，并移除URL中的引用参数
pub async fn process_url_full_removeref(url: &str, tags: &[&str], classes: &[&str], output_dir: &str) {
    let filtered_html = get_filtered_html_fullurl_removeref(url, tags, classes).await;
    save_filtered_html(&filtered_html, url, output_dir);
}

/// 保存过滤后的HTML内容到指定目录
fn save_filtered_html(filtered_html: &str, url: &str, output_dir: &str) {
    let filtered_html = remove_empty_lines(filtered_html.to_string());

    let output_path = generate_output_path(url, output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    let mut file = File::create(output_path.clone()).expect("Failed to create output file");
    file.write_all(filtered_html.as_bytes()).expect("Failed to write to file");

    println!("Filtered HTML for {} saved to {}", url, output_path);
}

/// 过滤指定标签
fn filter_tags(document: &NodeRef, rule: &str) {
    let mut nodes_to_remove: Vec<NodeRef> = Vec::new();

    for element in document.select(rule).expect("Failed to select nodes") {
        nodes_to_remove.push(element.as_node().clone());
    }

    for node in nodes_to_remove {
        node.detach();
    }
}

/// 过滤指定类名
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

/// 更新相对路径为绝对路径
fn update_relative_paths(document: &NodeRef, base_url: &str) {
    let base_url = Url::parse(base_url).expect("Failed to parse base URL");

    for img in document.select("img").expect("Failed to select img tags") {
        let mut attributes = img.attributes.borrow_mut();
        if let Some(src) = attributes.get("src") {
            if let Ok(url) = base_url.join(src) {
                attributes.insert("src", url.to_string());
            }
        }
    }

    for a in document.select("a").expect("Failed to select a tags") {
        let mut attributes = a.attributes.borrow_mut();
        if let Some(href) = attributes.get("href") {
            if let Ok(url) = base_url.join(href) {
                attributes.insert("href", url.to_string());
            }
        }
    }
}

/// 移除URL中的引用参数
fn remove_ref_from_urls(document: &NodeRef) {
    for a in document.select("a").expect("Failed to select a tags") {
        let mut attributes = a.attributes.borrow_mut();
        if let Some(href) = attributes.get("href") {
            let new_href = href.split("?ref=").next().unwrap_or(href).to_string();
            attributes.insert("href", new_href);
        }
    }

    for img in document.select("img").expect("Failed to select img tags") {
        let mut attributes = img.attributes.borrow_mut();
        if let Some(src) = attributes.get("src") {
            let new_src = src.split("?ref=").next().unwrap_or(src).to_string();
            attributes.insert("src", new_src);
        }
    }
}

/// 移除空行
fn remove_empty_lines(html: String) -> String {
    html.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

/// 生成输出路径
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
