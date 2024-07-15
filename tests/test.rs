mod tests {
    use libhtmlfilter::{process_url, process_url_full, process_url_full_removeref};

    #[tokio::test]
    async fn test_save_html() {
        let url = "https://itsfoss.com/ollama/";
        let output_dir = "output";

        let tags = vec!["script", "style", "link", "meta", "li", "desc", "title", "svg", "path", "dialog", "select", "head", "header", "foot", "footer", "ul", "nav", "button", "form", "input", "picture", "time", "h2", "h3", "h4", "i", "aside", "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child"];
        let classes = vec!["progress-bar", "js-menu", "social-share", "post-info__readtime", "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc", "author-card", "related-posts"];

        process_url(url, &tags, &classes, output_dir).await
    }

    #[tokio::test]
    async fn test_process_url_full() {
        let url = "https://itsfoss.com/ollama/";
        let output_dir = "output_fullurl";

        let tags = vec!["script", "style", "link", "meta", "li", "desc", "title", "svg", "path", "dialog", "select", "head", "header", "foot", "footer", "ul", "nav", "button", "form", "input", "picture", "time", "h2", "h3", "h4", "i", "aside", "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child"];
        let classes = vec!["progress-bar", "js-menu", "social-share", "post-info__readtime", "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc", "author-card", "related-posts"];

        process_url_full(url, &tags, &classes, output_dir).await
    }

    #[tokio::test]
    async fn test_process_url_full_removeref() {
        let url = "https://itsfoss.com/ollama/";
        let output_dir = "output_fullurl_removeref";

        let tags = vec!["script", "style", "link", "meta", "li", "desc", "title", "svg", "path", "dialog", "select", "head", "header", "foot", "footer", "ul", "nav", "button", "form", "input", "picture", "time", "h2", "h3", "h4", "i", "aside", "FreeStarVideoAdContainer", "freestar-video-parent", "reestar-video-child"];
        let classes = vec!["progress-bar", "js-menu", "social-share", "post-info__readtime", "cta__description", "cta__inner", "cta__content", "hide-mobile", "js-toc", "author-card", "related-posts"];

        process_url_full_removeref(url, &tags, &classes, output_dir).await
    }
}
