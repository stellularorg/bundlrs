use super::parser::Rule;
use pest::iterators::{Pair, Pairs};
use regex::RegexBuilder;
use shared::markdown::parse_markdown as shared_parse_markdown;

pub fn from_tree(tree: &Pairs<'_, Rule>, original_in: String) -> String {
    shared_parse_markdown(
        original_in,
        vec![&mut |mut out: String| {
            for block in tree.clone().into_iter() {
                let btype = block.as_rule();
                let block_string = block.as_span().as_str().to_string();
                let inner = block.into_inner().collect::<Vec<Pair<'_, Rule>>>();

                // e#theme (identifier)#
                if btype == Rule::THEME {
                    let theme = inner.get(0).unwrap();

                    out = out.replace(
                        &format!("e#{}#", block_string),
                        &format!("<theme>{}</theme>", theme.as_span().as_str()),
                    );

                    continue;
                }

                // e#hsl (hue/lit/sat) (percentage/int)#
                if btype == Rule::HSL {
                    let which = inner.get(0).unwrap();
                    let value = inner.get(1).unwrap();

                    out = out.replace(
                        &format!("e#{}#", block_string),
                        &format!(
                            "<{}>{}</{}>",
                            which.as_span().as_str(),
                            value.as_span().as_str(),
                            which.as_span().as_str()
                        ),
                    );

                    continue;
                }

                // e#html (identifier) {attrs}#
                if btype == Rule::HTML {
                    let tag = inner.get(0).unwrap();

                    let attrs = inner
                        .iter()
                        .skip(1)
                        .into_iter()
                        .collect::<Vec<&Pair<'_, Rule>>>();

                    // build attrs string
                    let mut attrs_string = String::new();

                    for attr in attrs {
                        attrs_string += &format!("{} ", attr.as_span().as_str());
                    }

                    // replace
                    out = out.replace(
                        &format!("e#{}#", block_string.replace("\"", "&quot;")),
                        &format!("<{} {}>", tag.as_span().as_str(), attrs_string),
                    );

                    continue;
                }

                // e#chtml (identifier)#
                if btype == Rule::CHTML {
                    let tag = inner.get(0).unwrap();

                    // replace
                    out = out.replace(
                        &format!("e#{}#", block_string),
                        &format!("</{}>", tag.as_span().as_str()),
                    );

                    continue;
                }

                // e#id (identifier)#
                if btype == Rule::ID {
                    let id = inner.get(0).unwrap();

                    // replace
                    out = out.replace(
                        &format!("e#{}#", block_string),
                        &format!("<span id=\"{}\">", id.as_span().as_str()),
                    );

                    continue;
                }

                // e#class (identifier)+#
                if btype == Rule::CLASS {
                    let attrs = inner.into_iter().collect::<Vec<Pair<'_, Rule>>>();

                    // build attrs string
                    let mut attrs_string = String::new();

                    for attr in attrs {
                        attrs_string += &format!("{} ", attr.as_span().as_str());
                    }

                    // replace
                    out = out.replace(
                        &format!("e#{}#", block_string.replace("\"", "&quot;")),
                        &format!("<span class=\"{}\">", attrs_string),
                    );

                    continue;
                }

                // e#close#
                if btype == Rule::CLOSE {
                    // replace
                    out = out.replace(&format!("e#{}#", block_string), "</span>");
                    continue;
                }

                // e#animation (identifier) {attrs}#
                if btype == Rule::ANIMATION {
                    let tag = inner.get(0).unwrap();

                    let attrs = inner
                        .iter()
                        .skip(1)
                        .into_iter()
                        .collect::<Vec<&Pair<'_, Rule>>>();

                    // build attrs string
                    let mut attrs_string = String::new();

                    for attr in attrs {
                        attrs_string += &format!("{} ", attr.as_span().as_str());
                    }

                    // replace
                    out = out.replace(
                &format!("e#{}#", block_string.replace("\"", "&quot;")),
                &format!("<span role=\"animation\" style=\"animation: {} {} ease-in-out forwards running; display: inline-block;\">", tag.as_span().as_str(), attrs_string),
            );

                    continue;
                }
            }

            // ssm
            // essentially just ssm::parse_ssm_blocks, maybe clean this up later?
            let ssm_regex = RegexBuilder::new("(ssm\\#)(?<CONTENT>.*?)\\#")
                .multi_line(true)
                .dot_matches_new_line(true)
                .build()
                .unwrap();

            for capture in ssm_regex.captures_iter(&out.clone()) {
                let content = capture.name("CONTENT").unwrap().as_str().replace("$", "#");

                // compile
                let css = crate::ssm::parse_ssm_program(content.to_string());

                // replace
                out = out.replace(
                    capture.get(0).unwrap().as_str(),
                    &format!("<style>{css}</style>"),
                );
            }

            // text color (bundlrs style)
            let color_regex =
                RegexBuilder::new("(c\\#)\\s*(?<COLOR>.*?)\\s*\\#\\s*(?<CONTENT>.*?)\\s*\\#")
                    .multi_line(true)
                    .dot_matches_new_line(true)
                    .build()
                    .unwrap();

            for capture in color_regex.captures_iter(&out.clone()) {
                let content = capture.name("CONTENT").unwrap().as_str();
                let color = capture.name("COLOR").unwrap().as_str().replace("$", "#");

                // replace
                out = out.replacen(
                    capture.get(0).unwrap().as_str(),
                    &format!(
                        "<span style=\"color: {color}\" role=\"custom-color\">{content}</span>"
                    ),
                    1,
                );
            }

            // text color thing
            out = regex_replace_exp(
                &out,
                RegexBuilder::new(r"%(.*?)%\s*(.*?)\s*(%{2})")
                    .multi_line(true)
                    .dot_matches_new_line(true),
                "<span style=\"color: $1;\" role=\"custom-color\">$2</span>",
            );

            // return
            out
        }],
    )
}

pub fn parse_markdown(input: &str) -> String {
    let tree = super::parser::parse(input);
    from_tree(&tree.into_inner(), input.to_owned())
}

#[allow(dead_code)]
fn regex_replace_exp(input: &str, pattern: &mut RegexBuilder, replace_with: &str) -> String {
    pattern
        .build()
        .unwrap()
        .replace_all(input, replace_with)
        .to_string()
}
