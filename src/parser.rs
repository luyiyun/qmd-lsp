use regex::Regex;

use crate::document::{CodeBlock, Heading, LabelDef, LabelKind, RefKind, RefUse};

// pub fn parse_headings(text: &str) -> Vec<Heading> {
//     // let mut headings = Vec::new();
//     // for (line_no, line) in text.lines().enumerate() {
//     //     if let Some(heading) = parse_heading(line, line_no as u32) {
//     //         headings.push(heading);
//     //     }
//     // }
//     // headings
//
//     text.lines()
//         .enumerate()
//         .filter_map(|(line_no, line)| parse_heading(line, line_no as u32))
//         .collect()
// }
//
// pub fn parse_labels(line: &str, line_no: u32) -> Vec<LabelDef> {
//     let re = Regex::new(r"\{#([A-Za-z0-9_\-]+)\}").unwrap();
//
//     re.captures_iter(line)
//         .filter_map(|cap| {
//             let m = cap.get(1)?;
//             let label = m.as_str().to_string();
//             Some(LabelDef {
//                 kind: LabelKind::from_label(&label),
//                 label,
//                 line: line_no,
//                 character: m.start() as u32,
//             })
//         })
//         .collect()
//     // let mut labels = Vec::new();
//     // for cap in re.captures_iter(line) {
//     //     let m = cap.get(1).unwrap();
//     //     let label = m.as_str();
//     //     labels.push(LabelDef {
//     //         kind: LabelKind::from_label(label),
//     //         label: label.to_string(),
//     //         line: line_no,
//     //         character: m.start() as u32,
//     //     })
//     // let label = m.as_str().to_string();
//     // labels.push(LabelDef {
//     //     kind: LabelKind::from_label(&label),
//     //     label,
//     //     line: line_no,
//     //     character: m.start() as u32,
//     // })
//     // }
//     //
//     // labels
// }
//
// pub fn parse_code_block_labels(text: &str) -> Vec<LabelDef> {
//     let mut labels = Vec::new();
//     let mut inside_code_block = false;
//     let mut current_block_has_label = false;
//     for (line_no, line) in text.lines().enumerate() {
//         let line_no = line_no as u32;
//         let trimmed = line.trim_start();
//
//         if trimmed.starts_with("```") {
//             if inside_code_block {
//                 inside_code_block = false;
//                 current_block_has_label = false;
//             } else {
//                 inside_code_block = true;
//                 current_block_has_label = false;
//
//                 if let Some(label) = parse_chunk_header_label(line) {
//                     labels.push(LabelDef {
//                         kind: LabelKind::from_label(&label),
//                         character: label_character(line, &label),
//                         line: line_no,
//                         label,
//                     });
//                     current_block_has_label = true;
//                 }
//             }
//             continue;
//         }
//
//         if inside_code_block
//             && !current_block_has_label
//             && let Some(label) = parse_chunk_option_label(line)
//         {
//             labels.push(LabelDef {
//                 kind: LabelKind::from_label(&label),
//                 character: label_character(line, &label),
//                 line: line_no,
//                 label,
//             });
//
//             current_block_has_label = true;
//         }
//     }
//     labels
// }
//
// pub fn parse_all_labels(text: &str) -> Vec<LabelDef> {
//     let mut labels = Vec::new();
//     for (line_no, line) in text.lines().enumerate() {
//         labels.extend(parse_labels(line, line_no as u32));
//     }
//     labels.extend(parse_code_block_labels(text));
//     labels
// }
//
// pub fn parse_refs(line: &str, line_no: u32) -> Vec<RefUse> {
//     let re = Regex::new(r" @([A-Za-z0-9_\-]+) ").unwrap();
//
//     re.captures_iter(line)
//         .filter_map(|cap| {
//             let m = cap.get(1)?;
//             let key = m.as_str().to_string();
//             Some(RefUse {
//                 kind: RefKind::from_ref(&key),
//                 key: key.to_string(),
//                 line: line_no,
//                 character: m.start() as u32,
//             })
//         })
//         .collect()
//     // let mut refs = Vec::new();
//     // for cap in re.captures_iter(line) {
//     //     let m = cap.get(1).unwrap();
//     //     let key = m.as_str();
//     //     refs.push()
//     // }
//     // refs
// }
//
// pub fn parse_all_refs(text: &str) -> Vec<RefUse> {
//     text.lines()
//         .enumerate()
//         .flat_map(|(line_no, line)| parse_refs(line, line_no as u32))
//         .collect()
//     //
//     // let mut refs = Vec::new();
//     // for (line_no, line) in text.lines().enumerate() {
//     //     let line_refs = parse_refs(line, line_no as u32);
//     //     for ref_use in line_refs {
//     //         refs.push(ref_use);
//     //     }
//     // }
//     // refs
// }
//
// fn parse_code_block_language(line: &str) -> Option<String> {
//     let trimmed = line.trim_start();
//     let header = trimmed.strip_prefix("```")?.trim();
//
//     if header.is_empty() {
//         return None;
//     }
//
//     if header.starts_with('{') && header.ends_with('}') {
//         let inner = header.trim_start_matches('{').trim_end_matches('}').trim();
//         return inner.split_whitespace().next().map(|s| s.to_string());
//     }
//
//     Some(header.to_string())
// }
//
// fn parse_chunk_header_label(line: &str) -> Option<String> {
//     let trimmed = line.trim_start();
//     let re = Regex::new(r"^```\{[A-Za-z0-9_\-]+\s+([A-Za-z0-9_\-]+)\s*\}$").unwrap();
//     let cap = re.captures(trimmed)?;
//
//     cap.get(1).map(|m| m.as_str().to_string())
// }
//
// fn parse_chunk_option_label(line: &str) -> Option<String> {
//     let trimmed = line.trim_start();
//
//     let re = Regex::new(r#"^\#\|\s*label\s*:\s*["']?([A-Za-z0-9_\-]+)["']?\s*$"#).unwrap();
//     // #| label: fig-model
//     let cap = re.captures(trimmed)?;
//
//     cap.get(1).map(|m| m.as_str().to_string())
// }
//
// fn label_character(line: &str, label: &str) -> u32 {
//     line.find(label).unwrap_or(0) as u32
// }
//
// pub fn parse_code_blocks(text: &str) -> Vec<CodeBlock> {
//     let mut blocks = Vec::new();
//     let mut current_block: Option<CodeBlock> = None;
//
//     for (line_no, line) in text.lines().enumerate() {
//         let line_no = line_no as u32;
//         let trimmed = line.trim_start();
//
//         if trimmed.starts_with("```") {
//             if let Some(mut block) = current_block.take() {
//                 block.end_line = line_no;
//                 blocks.push(block);
//             } else {
//                 current_block = Some(CodeBlock {
//                     language: parse_code_block_language(line),
//                     label: parse_chunk_header_label(line),
//                     start_line: line_no,
//                     end_line: line_no,
//                 })
//             }
//
//             continue;
//         }
//
//         if let Some(block) = current_block.as_mut()
//             && block.label.is_none()
//         {
//             block.label = parse_chunk_option_label(line);
//         }
//     }
//
//     blocks
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn parse_basic_headings() {
//         let heading = parse_heading("# Introduction", 0).unwrap();
//         assert_eq!(heading.level, 1);
//         assert_eq!(heading.title, "Introduction");
//         assert_eq!(heading.line, 0);
//         assert_eq!(heading.character, 0);
//     }
//
//     #[test]
//     fn parse_level_two_heading() {
//         let heading = parse_heading("## Background", 3).unwrap();
//         assert_eq!(heading.level, 2);
//         assert_eq!(heading.title, "Background");
//         assert_eq!(heading.line, 3);
//     }
//
//     #[test]
//     fn ignore_plain_text() {
//         let heading = parse_heading("This is not a heading", 0);
//         assert!(heading.is_none());
//     }
//
//     #[test]
//     fn ignore_heading_without_space() {
//         let heading = parse_heading("#Introduction", 0);
//
//         assert!(heading.is_none());
//         // assert_eq!(heading, None);   // 也可以这样写，需要加 derive(PartialEq)
//     }
//
//     #[test]
//     fn ignore_too_deep_heading() {
//         let heading = parse_heading("####### Too deep", 0);
//
//         assert!(heading.is_none());
//     }
//
//     #[test]
//     fn parse_basic_label() {
//         let labels = parse_labels("![caption](fig.png){#fig-overview}", 10);
//
//         assert_eq!(labels.len(), 1);
//         assert_eq!(labels[0].label, "fig-overview");
//         assert_eq!(labels[0].kind, LabelKind::Figure);
//         assert_eq!(labels[0].line, 10);
//     }
//
//     #[test]
//     fn parse_multiple_labels_in_one_line() {
//         let labels = parse_labels("{#fig-a} and {#tbl-b}", 5);
//
//         assert_eq!(labels.len(), 2);
//         assert_eq!(labels[0].label, "fig-a");
//         assert_eq!(labels[1].label, "tbl-b");
//         assert_eq!(labels[0].kind, LabelKind::Figure);
//         assert_eq!(labels[1].kind, LabelKind::Table);
//     }
//
//     #[test]
//     fn parse_basic_code_block() {
//         let text = r#"# Title
//
//         ```{r}
//         summary(cars)
//         ```
//         some text
//         "#;
//         let blocks = parse_code_blocks(text);
//
//         assert_eq!(blocks.len(), 1);
//         assert_eq!(blocks[0].language, Some("r".to_string()));
//         assert_eq!(blocks[0].label, None);
//         assert_eq!(blocks[0].start_line, 2);
//         assert_eq!(blocks[0].end_line, 4);
//     }
//
//     #[test]
//     fn parse_code_block_without_language() {
//         let text = r#"```
//         plain text
//         ```"#;
//
//         let blocks = parse_code_blocks(text);
//
//         assert_eq!(blocks.len(), 1);
//         assert_eq!(blocks[0].language, None);
//         assert_eq!(blocks[0].start_line, 0);
//         assert_eq!(blocks[0].end_line, 2);
//     }
//
//     #[test]
//     fn parse_code_block_label_from_header() {
//         let text = r#"```{r fig-model}
// plot(cars)
// ```"#;
//
//         let blocks = parse_code_blocks(text);
//
//         assert_eq!(blocks.len(), 1);
//         assert_eq!(blocks[0].language.as_deref(), Some("r"));
//         assert_eq!(blocks[0].label.as_deref(), Some("fig-model"));
//     }
//
//     #[test]
//     fn parse_code_block_label_from_chunk_option() {
//         let text = r#"```{r}
// #| label: fig-model
// plot(cars)
// ```"#;
//
//         let blocks = parse_code_blocks(text);
//
//         assert_eq!(blocks.len(), 1);
//         assert_eq!(blocks[0].language.as_deref(), Some("r"));
//         assert_eq!(blocks[0].label.as_deref(), Some("fig-model"));
//     }
//
//     #[test]
//     fn code_block_label_is_added_to_label_index() {
//         let text = r#"```{r}
// #| label: fig-model
// plot(cars)
// ```"#;
//
//         let labels = parse_all_labels(text);
//
//         assert_eq!(labels.len(), 1);
//         assert_eq!(labels[0].label, "fig-model");
//         assert_eq!(labels[0].kind, LabelKind::Figure);
//     }
// }
