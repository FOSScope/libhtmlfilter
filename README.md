### 文档

#### 概述

本Rust程序用于从指定的URL下载HTML内容，并根据给定的标签和类名过滤HTML内容，然后将过滤后的HTML保存到指定的输出目录中。程序使用了`reqwest`库进行HTTP请求，`kuchiki`库进行HTML解析和操作，以及标准库中的文件操作和时间处理功能。

#### 函数详解

##### `process_url`

```rust
pub fn process_url(url: &str, tags: &[&str], classes: &[&str], output_dir: &str)
```

- **描述**: 处理给定的URL，获取并保存过滤后的HTML内容。
- **参数**:
  - `url`: 要处理的URL字符串。
  - `tags`: 要过滤的HTML标签数组。
  - `classes`: 要过滤的HTML类名数组。
  - `output_dir`: 保存过滤后HTML的目录路径。
- **行为**: 调用`get_filtered_html`获取过滤后的HTML，然后调用`save_filtered_html`保存结果。

##### `reverse_process_url`

```rust
pub fn reverse_process_url(url: &str, tags: &[&str], classes: &[&str], output_dir: &str)
```

- **描述**: 与`process_url`功能相同，但用于反向处理（保留指定标签和类名以外的内容）。
- **参数**: 同`process_url`。
- **行为**: 调用`get_filtered_html`获取过滤后的HTML，然后调用`save_filtered_html`保存结果。

##### `get_filtered_html`

```rust
pub fn get_filtered_html(url: &str, tags: &[&str], classes: &[&str]) -> String
```

- **描述**: 从URL获取HTML内容并根据指定的标签和类名进行过滤。
- **参数**: 同`process_url`。
- **返回值**: 过滤后的HTML字符串。
- **行为**: 使用`reqwest`获取URL内容，使用`kuchiki`解析HTML，根据标签和类名过滤节点，并返回过滤后的HTML字符串。

##### `save_filtered_html`

```rust
fn save_filtered_html(filtered_html: &str, url: &str, output_dir: &str)
```

- **描述**: 将过滤后的HTML内容保存到指定目录。
- **参数**:
  - `filtered_html`: 过滤后的HTML字符串。
  - `url`: 原始URL字符串。
  - `output_dir`: 输出目录路径。
- **行为**: 生成输出文件路径，创建目录（如果不存在），创建文件并写入过滤后的HTML内容。

##### `filter_tags`

```rust
fn filter_tags(document: &NodeRef, rule: &str)
```

- **描述**: 根据指定的标签规则过滤HTML文档中的节点。
- **参数**:
  - `document`: HTML文档节点引用。
  - `rule`: 标签选择器规则。
- **行为**: 选择并移除匹配的节点。

##### `filter_classes`

```rust
fn filter_classes(document: &NodeRef, class: &str)
```

- **描述**: 根据指定的类名过滤HTML文档中的节点。
- **参数**:
  - `document`: HTML文档节点引用。
  - `class`: 类名。
- **行为**: 选择并移除包含指定类名的节点。

##### `remove_empty_lines`

```rust
fn remove_empty_lines(html: String) -> String
```

- **描述**: 移除HTML字符串中的空行。
- **参数**:
  - `html`: 输入的HTML字符串。
- **返回值**: 移除空行后的HTML字符串。

##### `generate_output_path`

```rust
fn generate_output_path(url: &str, output_dir: &str) -> String
```

- **描述**: 生成保存过滤后HTML文件的路径。
- **参数**:
  - `url`: 原始URL字符串。
  - `output_dir`: 输出目录路径。
- **返回值**: 输出文件路径字符串。

#### 测试用例

```rust
#[test]
fn test_save_html() {
    let url = "https://itsfoss.com/ollama/";
    let output_dir = "output";

    let tags = vec!["script", "style", "link", "meta", "li", "desc", "title", "svg", "path", "dialog", "select", "head", "header", "foot", "footer", "ul", "nav", "button", "form", "input", "figure", "picture", "time", "h2", "h3", "h4", "i", "aside", "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child"];
    let classes = vec!["progress-bar", "js-menu", "social-share", "post-info__readtime", "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc", "author-card", "related-posts"];

    process_url(url, &tags, &classes, output_dir);
}
```

- **描述**: 测试`process_url`函数，处理指定URL并保存过滤后的HTML内容到`output`目录。
- **行为**: 调用`process_url`函数，传入URL、标签和类名，验证输出文件是否正确生成。