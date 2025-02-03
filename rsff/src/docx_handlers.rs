use docx_rs::{Docx as DocxBuilder, Paragraph, Run};
use docx_rust::{
    document::{BodyContent, ParagraphContent, RunContent},
    DocxFile as DocxReader,
};
use std::path::Path;

pub fn parse_docx_to_string(p: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let docx = DocxReader::from_file(p)?;
    let docx = docx.parse()?;
    let mut doc_strings: Vec<String> = Vec::with_capacity(200);
    for element in docx.document.body.content {
        let mut text = String::with_capacity(200);
        match element {
            BodyContent::Paragraph(p) => {
                let p_contents = p.content;
                for pc in p_contents {
                    match pc {
                        ParagraphContent::Run(r) => {
                            let run_cont = handle_docx_run(r);
                            text.push_str(&run_cont);
                        }
                        ParagraphContent::Link(hyperlink) => {
                            if let Some(r) = hyperlink.content {
                                let run_cont = handle_docx_run(r);
                                text.push_str(&run_cont);
                            }
                        }
                        _ => {}
                    }
                }
            }
            BodyContent::Run(r) => {
                let run_cont = handle_docx_run(r);
                text.push_str(&run_cont);
            }
            _ => {}
        }

        if !text.is_empty() {
            doc_strings.push(text);
        }
    }

    Ok(doc_strings.join("\n//\n"))
}

fn handle_docx_run(run: docx_rust::document::Run) -> String {
    let mut runstring = String::new();

    for element in run.content {
        match element {
            RunContent::Text(t) => {
                runstring.push_str(&t.text);
            }
            RunContent::SoftHyphen(_) => {
                runstring.push('\u{00AD}');
            }
            RunContent::NoBreakHyphen(_) => {
                runstring.push('\u{2011}');
            }
            RunContent::Separator(_) => {
                runstring.push('\u{2028}');
            }
            RunContent::Tab(_) => {
                runstring.push('\t');
            }
            RunContent::CarriageReturn(_) => {
                runstring.push(' ');
            }
            _ => {}
        }
    }
    runstring
}

pub fn string_to_docx(s: &str) -> DocxBuilder {
    let mut docx = DocxBuilder::new();
    for line in s.lines() {
        let paragraph = Paragraph::new().add_run(Run::new().add_text(line));
        docx = docx.add_paragraph(paragraph);
    }

    docx
}
