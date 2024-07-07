# HTML过滤库使用文档

## 概述

本库提供了一个用于过滤HTML内容的工具，支持正向过滤和反向过滤。正向过滤会移除指定的tag和class，而反向过滤则会保留指定的tag和class，移除其他所有内容。此外，本库还支持仅使用tag或class进行过滤。

## 功能

- **正向过滤**：移除指定的tag和class。
- **反向过滤**：保留指定的tag和class，移除其他所有内容。
- **仅使用tag过滤**：仅移除或保留指定的tag。
- **仅使用class过滤**：仅移除或保留指定的class。

## 依赖

本库依赖以下crate：

- `kuchiki`：用于HTML解析和操作。
- `reqwest`：用于HTTP请求。
- `std`：标准库。

## 安装

在`Cargo.toml`文件中添加依赖：

```toml
[dependencies]
libhtmlfilter = "0.1.0"
```

## 使用方法

### 导入库

在你的Rust文件中导入库：

```rust
use html_filter::{process_url, reverse_process_url};
```

### 正向过滤

使用`process_url`函数进行正向过滤：

#### 正向过滤示例（同时使用tag和class）

```rust
fn main() {
    let url = "https://example.com";
    let tags = vec!["script", "style"];
    let classes = vec!["hidden", "no-display"];
    let output_dir = "filtered_html";

    process_url(url, &tags, &classes, output_dir);
}
```

#### 正向过滤示例（仅使用tag）

```rust
fn main() {
    let url = "https://example.com";
    let tags = vec!["script", "style"];
    let classes = vec![];
    let output_dir = "filtered_html";

    process_url(url, &tags, &classes, output_dir);
}
```

#### 正向过滤示例（仅使用class）

```rust
fn main() {
    let url = "https://example.com";
    let tags = vec![];
    let classes = vec!["hidden", "no-display"];
    let output_dir = "filtered_html";

    process_url(url, &tags, &classes, output_dir);
}
```

### 反向过滤

使用`reverse_process_url`函数进行反向过滤：

#### 反向过滤示例（同时使用tag和class）

```rust
fn main() {
    let url = "https://example.com";
    let tags = vec!["div", "p"];
    let classes = vec!["content", "main"];
    let output_dir = "filtered_html";

    reverse_process_url(url, &tags, &classes, output_dir);
}
```

#### 反向过滤示例（仅使用tag）

```rust
fn main() {
    let url = "https://example.com";
    let tags = vec!["div", "p"];
    let classes = vec![];
    let output_dir = "filtered_html";

    reverse_process_url(url, &tags, &classes, output_dir);
}
```

#### 反向过滤示例（仅使用class）

```rust
fn main() {
    let url = "https://example.com";
    let tags = vec![];
    let classes = vec!["content", "main"];
    let output_dir = "filtered_html";

    reverse_process_url(url, &tags, &classes, output_dir);
}
```

### 参数说明

- `url`：要过滤的HTML页面的URL。
- `tags`：要过滤的tag列表。如果为空，则不进行tag过滤。
- `classes`：要过滤的class列表。如果为空，则不进行class过滤。
- `output_dir`：保存过滤后HTML文件的目录。

## 许可证

本库采用 MIT 许可证。详细信息请参阅 LICENSE 文件。
