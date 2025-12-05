use crate::Theme;
use dioxus::prelude::*;
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Parser, Tag, TagEnd};
use std::sync::OnceLock;
use syntastica::renderer::HtmlRenderer;
use syntastica_parsers_git::{Lang, LanguageSetImpl};
use syntastica_themes as code_themes;

static LANGUAGE_SET: OnceLock<LanguageSetImpl> = OnceLock::new();

fn language_set() -> &'static LanguageSetImpl {
    LANGUAGE_SET.get_or_init(LanguageSetImpl::new)
}

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: Signal<String>,

    content: ReadSignal<String>,
}

pub fn Markdown(props: MarkdownProps) -> Element {
    let id_signal = props.id;
    let class_signal = props.class;
    let content_signal = props.content;

    let site_theme = use_context::<Signal<Theme>>();

    let html_signal = use_memo(move || {
        let markdown = content_signal.read().clone();
        let use_dark_theme = matches!(site_theme(), Theme::Dark);
        render_markdown_with_syntax(&markdown, use_dark_theme)
    });

    let id_value = id_signal.read().clone();
    let class_value = class_signal.read().clone();

    rsx! {
        div {
            id: "{id_value}",
            class: "{class_value}",
            dangerous_inner_html: "{html_signal()}"
        }
    }
}

fn render_markdown_with_syntax(markdown: &str, use_dark_theme: bool) -> String {
    let parser = Parser::new(markdown);
    let highlighted = HighlightingAdapter::new(parser, use_dark_theme);

    let mut out = String::new();
    pulldown_cmark::html::push_html(&mut out, highlighted);
    out
}

struct HighlightingAdapter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    inner: I,
    collecting: bool,
    lang_label: Option<String>,
    buffer: String,
    pending_html: Option<String>,
    use_dark_theme: bool,
}

impl<'a, I> HighlightingAdapter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    fn new(inner: I, use_dark_theme: bool) -> Self {
        Self {
            inner,
            collecting: false,
            lang_label: None,
            buffer: String::new(),
            pending_html: None,
            use_dark_theme,
        }
    }
}

impl<'a, I> Iterator for HighlightingAdapter<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(html) = self.pending_html.take() {
                return Some(Event::Html(CowStr::Boxed(html.into_boxed_str())));
            }

            let event = self.inner.next()?;

            match event {
                Event::Start(Tag::CodeBlock(kind)) => {
                    self.collecting = true;
                    self.buffer.clear();
                    self.lang_label = match kind {
                        CodeBlockKind::Fenced(info) => {
                            let info = info.trim();
                            if info.is_empty() {
                                None
                            } else {
                                let first = info
                                    .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
                                    .next()
                                    .unwrap_or("")
                                    .to_string();
                                if first.is_empty() { None } else { Some(first) }
                            }
                        }
                        CodeBlockKind::Indented => None,
                    };
                    continue;
                }

                Event::End(TagEnd::CodeBlock) if self.collecting => {
                    let code = std::mem::take(&mut self.buffer);
                    let lang_label = self.lang_label.take();
                    let html =
                        highlight_code_block(&code, lang_label.as_deref(), self.use_dark_theme);

                    self.collecting = false;
                    self.pending_html = Some(html);
                    continue;
                }

                Event::Text(text) if self.collecting => {
                    self.buffer.push_str(&text);
                    continue;
                }

                Event::SoftBreak | Event::HardBreak if self.collecting => {
                    self.buffer.push('\n');
                    continue;
                }

                other => {
                    return Some(other);
                }
            }
        }
    }
}

fn highlight_code_block(code: &str, lang_label: Option<&str>, use_dark_theme: bool) -> String {
    let lang = lang_label.and_then(map_lang);

    if let Some(lang) = lang {
        let language_set = language_set();
        let mut renderer = HtmlRenderer::new();
        let theme = if use_dark_theme {
            code_themes::one::dark()
        } else {
            code_themes::one::light()
        };

        if let Ok(html) = syntastica::highlight(code, lang, language_set, &mut renderer, theme) {
            return html;
        }
    }

    let escaped = escape_html(code);
    let class_attr = lang_label
        .map(|l| format!(" class=\"language-{}\"", l))
        .unwrap_or_default();
    format!(
        "<pre><code{class}>{escaped}</code></pre>",
        class = class_attr
    )
}

fn map_lang(info: &str) -> Option<Lang> {
    match info.to_ascii_lowercase().as_str() {
        "rs" | "rust" => Some(Lang::Rust),
        "hs" | "haskell" => Some(Lang::Haskell),
        "bash" | "sh" | "shell" => Some(Lang::Bash),
        "py" | "python" => Some(Lang::Python),
        "js" | "javascript" => Some(Lang::Javascript),
        "ts" | "typescript" => Some(Lang::Typescript),
        "toml" => Some(Lang::Toml),
        "yaml" | "yml" => Some(Lang::Yaml),
        _ => None,
    }
}

fn escape_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
    out
}

