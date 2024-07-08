# LibHTMLFilter

本库提供了一系列函数，用于从指定URL获取HTML内容，并根据用户定义的标签和类名进行过滤，同时支持将相对路径更新为绝对路径，以及移除URL中的引用参数。以下是每个函数的详细说明和使用示例。

#### 1. `get_filtered_html`

**功能描述**：获取过滤后的HTML内容。

**参数**：
- `url: &str`：目标URL。
- `tags: &[&str]`：需要过滤的标签列表。
- `classes: &[&str]`：需要过滤的类名列表。

**返回值**：过滤后的HTML内容字符串。

**示例**：
```rust
let url = "https://example.com";
let tags = vec!["script", "style"];
let classes = vec!["class1", "class2"];
let filtered_html = get_filtered_html(url, &tags, &classes);
println!("{}", filtered_html);
```

#### 2. `get_filtered_html_fullurl`

**功能描述**：获取过滤后的HTML内容，并更新相对路径为绝对路径。

**参数**：
- `url: &str`：目标URL。
- `tags: &[&str]`：需要过滤的标签列表。
- `classes: &[&str]`：需要过滤的类名列表。

**返回值**：过滤后的HTML内容字符串，且所有相对路径已更新为绝对路径。

**示例**：
```rust
let url = "https://example.com";
let tags = vec!["script", "style"];
let classes = vec!["class1", "class2"];
let filtered_html = get_filtered_html_fullurl(url, &tags, &classes);
println!("{}", filtered_html);
```

#### 3. `get_filtered_html_fullurl_removeref`

**功能描述**：获取过滤后的HTML内容，更新相对路径为绝对路径，并移除URL中的引用参数。

**参数**：
- `url: &str`：目标URL。
- `tags: &[&str]`：需要过滤的标签列表。
- `classes: &[&str]`：需要过滤的类名列表。

**返回值**：过滤后的HTML内容字符串，所有相对路径已更新为绝对路径，且移除了URL中的引用参数。

**示例**：
```rust
let url = "https://example.com";
let tags = vec!["script", "style"];
let classes = vec!["class1", "class2"];
let filtered_html = get_filtered_html_fullurl_removeref(url, &tags, &classes);
println!("{}", filtered_html);
```

#### 4. `process_url`

**功能描述**：处理URL并保存过滤后的HTML内容。

**参数**：
- `url: &str`：目标URL。
- `tags: &[&str]`：需要过滤的标签列表。
- `classes: &[&str]`：需要过滤的类名列表。
- `output_dir: &str`：输出目录。

**示例**：
```rust
let url = "https://example.com";
let tags = vec!["script", "style"];
let classes = vec!["class1", "class2"];
let output_dir = "output";
process_url(url, &tags, &classes, output_dir);
```

#### 5. `process_url_full`

**功能描述**：处理URL并保存过滤后的HTML内容，更新相对路径为绝对路径。

**参数**：
- `url: &str`：目标URL。
- `tags: &[&str]`：需要过滤的标签列表。
- `classes: &[&str]`：需要过滤的类名列表。
- `output_dir: &str`：输出目录。

**示例**：
```rust
let url = "https://example.com";
let tags = vec!["script", "style"];
let classes = vec!["class1", "class2"];
let output_dir = "output_fullurl";
process_url_full(url, &tags, &classes, output_dir);
```

#### 6. `process_url_full_removeref`

**功能描述**：处理URL并保存过滤后的HTML内容，更新相对路径为绝对路径，并移除URL中的引用参数。

**参数**：
- `url: &str`：目标URL。
- `tags: &[&str]`：需要过滤的标签列表。
- `classes: &[&str]`：需要过滤的类名列表。
- `output_dir: &str`：输出目录。

**示例**：
```rust
let url = "https://example.com";
let tags = vec!["script", "style"];
let classes = vec!["class1", "class2"];
let output_dir = "output_fullurl_removeref";
process_url_full_removeref(url, &tags, &classes, output_dir);
```

#### 7. `save_filtered_html`

**功能描述**：保存过滤后的HTML内容到指定目录。

**参数**：
- `filtered_html: &str`：过滤后的HTML内容。
- `url: &str`：目标URL。
- `output_dir: &str`：输出目录。

**示例**：
```rust
let filtered_html = "<html><body><h1>Hello, World!</h1></body></html>";
let url = "https://example.com";
let output_dir = "output";
save_filtered_html(filtered_html, url, output_dir);
```

#### 8. `filter_tags`

**功能描述**：过滤指定标签。

**参数**：
- `document: &NodeRef`：HTML文档节点引用。
- `rule: &str`：过滤规则。

**示例**：
```rust
let content = "<html><body><script>alert('test');</script><h1>Hello, World!</h1></body></html>";
let document = parse_html().one(content);
filter_tags(&document, "script");
println!("{}", document.to_string());
```

#### 9. `filter_classes`

**功能描述**：过滤指定类名。

**参数**：
- `document: &NodeRef`：HTML文档节点引用。
- `class: &str`：类名。

**示例**：
```rust
let content = "<html><body><div class='class1'>Hello, World!</div></body></html>";
let document = parse_html().one(content);
filter_classes(&document, "class1");
println!("{}", document.to_string());
```

#### 10. `update_relative_paths`

**功能描述**：更新相对路径为绝对路径。

**参数**：
- `document: &NodeRef`：HTML文档节点引用。
- `base_url: &str`：基础URL。

**示例**：
```rust
let content = "<html><body><img src='image.jpg'/></body></html>";
let document = parse_html().one(content);
update_relative_paths(&document, "https://example.com");
println!("{}", document.to_string());
```

#### 11. `remove_ref_from_urls`

**功能描述**：移除URL中的引用参数。

**参数**：
- `document: &NodeRef`：HTML文档节点引用。

**示例**：
```rust
let content = "<html><body><a href='https://example.com?ref=123'>Link</a></body></html>";
let document = parse_html().one(content);
remove_ref_from_urls(&document);
println!("{}", document.to_string());
```

#### 12. `remove_empty_lines`

**功能描述**：移除空行。

**参数**：
- `html: String`：HTML内容字符串。

**返回值**：移除空行后的HTML内容字符串。

**示例**：
```rust
let html = "<html>\n<body>\n<h1>Hello, World!</h1>\n</body>\n</html>";
let cleaned_html = remove_empty_lines(html);
println!("{}", cleaned_html);
```

#### 13. `generate_output_path`

**功能描述**：生成输出路径。

**参数**：
- `url: &str`：目标URL。
- `output_dir: &str`：输出目录。

**返回值**：输出文件路径字符串。

**示例**：
```rust
let url = "https://example.com/path/to/page";
let output_dir = "output";
let output_path = generate_output_path(url, output_dir);
println!("{}", output_path);
```

### 测试函数

本库还包含了一些测试函数，用于验证主要功能是否正常工作。这些测试函数可以在`tests`模块中找到。

#### `test_save_html`

**功能描述**：测试保存过滤后的HTML内容。

**示例**：
```rust
#[test]
fn test_save_html() {
    let url = "https://itsfoss.com/ollama/";
    let output_dir = "output";
    let tags = vec!["script", "style"];
    let classes = vec!["class1", "class2"];
    process_url(url, &tags, &classes, output_dir);
}
```

#### `test_process_url_full`

**功能描述**：测试处理URL并保存过滤后的HTML内容，更新相对路径为绝对路径。

**示例**：
```rust
#[test]
fn test_process_url_full() {
    let url = "https://itsfoss.com/ollama/";
    let output_dir = "output_fullurl";
    let tags = vec!["script", "style"];
    let classes = vec!["class1", "class2"];
    process_url_full(url, &tags, &classes, output_dir);
}
```

#### `test_process_url_full_removeref`

**功能描述**：测试处理URL并保存过滤后的HTML内容，更新相对路径为绝对路径，并移除URL中的引用参数。

**示例**：
```rust
#[test]
fn test_process_url_full_removeref() {
    let url = "https://itsfoss.com/ollama/";
    let output_dir = "output_fullurl_removeref";
    let tags = vec!["script", "style"];
    let classes = vec!["class1", "class2"];
    process_url_full_removeref(url, &tags, &classes, output_dir);
}
```

### 依赖库

本库依赖以下外部库：
- `kuchiki`：用于HTML解析和操作。
- `reqwest`：用于HTTP请求。

