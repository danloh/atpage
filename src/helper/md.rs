use pulldown_cmark::{html, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::fmt::Write;
use std::io::Cursor;

pub struct HTMLOutput {
	pub html: String,
}

/// markdown to html
/// : latex to html
/// Input: string
/// Output: {html: string}
const OPTIONS: Options = Options::all();
pub fn md2html(input: &str) -> HTMLOutput {
	let parser = Parser::new_ext(input, OPTIONS);
	// some parm and result holder
	let mut output: Vec<u8> = Vec::new();
	let mut current_heading: Option<Heading> = None;
	let mut in_blockquote = false;
	let mut current_code: Option<Code> = None;

	let stream = parser.map(|ev| {
		match &ev {
			Event::Start(Tag::Heading { level, id, classes, attrs }) if !in_blockquote => {
				let attrs_vec = attrs
					.iter()
					.map(|a| (a.0.to_string(), a.1.clone().map(|c| c.to_string())))
					.collect();
				current_heading = Some(Heading {
					level: *level,
					frag: id.clone().map(|i| i.to_string()),
					class: classes.iter().map(ToString::to_string).collect(),
					attrs: attrs_vec,
					markup: "".into(),
					plain_text: "".into(),
				});
				return Event::Text("".into());
			}
			Event::Start(Tag::BlockQuote(_)) => {
				in_blockquote = true;
			}
			Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
				current_code = Some(Code { lang: lang.to_string(), source: Default::default() });
				return Event::Text("".into());
			}
			Event::Code(contents) => {
				if let Some(cur_heading) = &mut current_heading {
					cur_heading.markup.push_str(&format!("<code>{contents}</code>"));
					cur_heading.plain_text.push_str(contents.as_ref());
					return Event::Text("".into());
				}
			}
			Event::InlineMath(c) => {
				return Event::Html(
					latex2mathml::latex_to_mathml(&c, latex2mathml::DisplayStyle::Inline)
						.unwrap_or_else(|e| format!("Render math {} failed: {}", c, e))
						.into(),
				);
			}
			Event::DisplayMath(c) => {
				return Event::Html(
					latex2mathml::latex_to_mathml(&c, latex2mathml::DisplayStyle::Block)
						.unwrap_or_else(|e| format!("Render math {} failed: {}", c, e))
						.into(),
				);
			}
			Event::End(TagEnd::Heading(_level)) => {
				if let Some(heading) = current_heading.take() {
					let tag = format!("h{}", heading.level as i32);
					let markup = &heading.markup;
					let anchor = heading.frag.unwrap_or_else(|| slug::slugify(&heading.plain_text));
					let class = format!("anchor {}", heading.class.join(" "));
					let mut style = String::new();
					for attr in heading.attrs {
						if let Some(val) = attr.1 {
							style.push_str(&format!("{}:{};", attr.0, val))
						}
					}

					return Event::Html(
						format!(
							r#"<{tag} id="{anchor}" class="{class}" style="{style}">{markup}</{tag}>"#
						)
						.into(),
					);
				}
			}
			Event::End(TagEnd::CodeBlock) => {
				if let Some(cur_code) = current_code.take() {
					let mut out: String = String::new();
					let lang = &cur_code.lang;
					write!(&mut out, r#"<div class="code-block">"#,).ok();

					if !lang.is_empty() {
						write!(&mut out, r#"<div class="language-tag">{}</div>"#, lang,).ok();
					}
					write!(&mut out, r#"<pre class="code-block-inner" data-lang={:?}>"#, lang).ok();

					write!(&mut out, r#"{}"#, cur_code.source).ok();
					write!(&mut out, "</pre></div>").ok();

					return Event::Html(out.into());
				}
			}
			Event::End(TagEnd::BlockQuote(_)) => {
				in_blockquote = false;
			}
			Event::Text(text) => {
				if let Some(cur_code) = current_code.as_mut() {
					cur_code.source.push_str(text);
					return Event::Text("".into());
				}
				if let Some(cur_heading) = current_heading.as_mut() {
					cur_heading.markup.push_str(text);
					cur_heading.plain_text.push_str(text);
					return Event::Text("".into());
				}
			}
			_ => {}
		}

		ev
	});

	html::write_html_io(Cursor::new(&mut output), stream).unwrap_or(());

	let res =
		String::from_utf8(output).unwrap_or_else(|_| String::from("Failed to render Markdown"));

	HTMLOutput { html: res }
}

struct Code {
	lang: String,
	source: String,
}

struct Heading {
	level: HeadingLevel,
	#[allow(dead_code)]
	frag: Option<String>,
	#[allow(dead_code)]
	class: Vec<String>,
	#[allow(dead_code)]
	attrs: Vec<(String, Option<String>)>,
	markup: String,
	plain_text: String,
}
