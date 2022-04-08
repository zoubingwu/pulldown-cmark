#[macro_use]
extern crate napi_derive;

use pulldown_cmark::{html, CodeBlockKind, CowStr, Event, Options, Parser, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[napi(object)]
pub struct JsOption {
    pub enable_tables: bool,
    pub enable_footnotes: bool,
    pub enable_strikethrough: bool,
    pub enable_tasklists: bool,
    pub enable_smart_punctuation: bool,
    pub enable_heading_attributes: bool,
    pub enable_container: bool,
    pub enable_mdx: bool,
    pub code_block_theme: String,
}

#[napi]
pub fn transform_no_highlight(content: String, opts: JsOption) -> String {
    let mut pulldown_options = Options::empty();
    if opts.enable_tables {
        pulldown_options.insert(Options::ENABLE_TABLES);
    }
    if opts.enable_footnotes {
        pulldown_options.insert(Options::ENABLE_FOOTNOTES);
    }
    if opts.enable_strikethrough {
        pulldown_options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if opts.enable_tasklists {
        pulldown_options.insert(Options::ENABLE_TASKLISTS);
    }
    if opts.enable_smart_punctuation {
        pulldown_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    if opts.enable_heading_attributes {
        pulldown_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }
    let parser = Parser::new_ext(&content, pulldown_options);

    let mut html_output: String = String::with_capacity(&content.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}

#[napi]
pub fn transform(content: String, opts: JsOption) -> String {
    let mut pulldown_options = Options::empty();

    if opts.enable_tables {
        pulldown_options.insert(Options::ENABLE_TABLES);
    }
    if opts.enable_footnotes {
        pulldown_options.insert(Options::ENABLE_FOOTNOTES);
    }
    if opts.enable_strikethrough {
        pulldown_options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if opts.enable_tasklists {
        pulldown_options.insert(Options::ENABLE_TASKLISTS);
    }
    if opts.enable_smart_punctuation {
        pulldown_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    if opts.enable_heading_attributes {
        pulldown_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes[opts.code_block_theme.as_str()];

    let mut new_events: Vec<Event> = Vec::new();
    let mut in_code_block = false;
    let mut code_lang: String = String::new();
    let mut to_highlight = String::new();

    let parser = Parser::new_ext(&content, pulldown_options).map(|event| match event {
        // Event::Start(Tag::CodeBlock(info)) => {
        //     in_code_block = true;
        //     if let CodeBlockKind::Fenced(info) = info {
        //         code_lang.push_str(info.split(' ').next().unwrap());
        //     }
        //     event
        // }
        // Event::End(Tag::CodeBlock(_)) => {
        //     if in_code_block {
        //         let syntax = ss.find_syntax_by_extension(code_lang.as_str()).unwrap();
        //         let html = highlighted_html_for_string(&to_highlight, &ss, &syntax, &theme);
        //         let str: &str = &html;
        //         new_events.push(Event::Html(CowStr::Boxed(str.to_owned().into_boxed_str())));
        //
        //         to_highlight = String::new();
        //         code_lang = String::new();
        //         in_code_block = false;
        //     }
        // }
        // Event::Text(t) => {
        //     if in_code_block {
        //         to_highlight.push_str(&t);
        //     } else {
        //         new_events.push(Event::Text(t))
        //     }
        // }
        _ => event,
    });

    // for event in p {
    //     match event {
    // Event::Start(Tag::CodeBlock(info)) => {
    //     in_code_block = true;
    //     if let CodeBlockKind::Fenced(info) = info {
    //         code_lang.push_str(info.split(' ').next().unwrap());
    //     }
    // }
    // Event::End(Tag::CodeBlock(_)) => {
    //     if in_code_block {
    //         let syntax = ss.find_syntax_by_extension(code_lang.as_str()).unwrap();
    //         let html = highlighted_html_for_string(&to_highlight, &ss, &syntax, &theme);
    //         let str: &str = &html;
    //         new_events.push(Event::Html(CowStr::Boxed(str.to_owned().into_boxed_str())));
    //
    //         to_highlight = String::new();
    //         code_lang = String::new();
    //         in_code_block = false;
    //     }
    // }
    // Event::Text(t) => {
    //     if in_code_block {
    //         to_highlight.push_str(&t);
    //     } else {
    //         new_events.push(Event::Text(t))
    //     }
    // }
    //         e => new_events.push(e),
    //     }
    // }

    let mut html_output: String = String::with_capacity(&content.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}
