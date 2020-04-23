extern crate latex;
extern crate tectonic;
use latex::{print, Document, DocumentClass, Element, Section};
use tectonic::latex_to_pdf;
/*use astrology::svg_draw::{DataChartNatal, DataObjectSvg, DataObjectType};
use base64::encode;
use libswe_sys::sweconst::Language;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
*/
fn create_document() -> Document {
    let mut doc = Document::new(DocumentClass::Article);

    // Set some metadata for the document
    doc.preamble.title("My Fancy Document");
    doc.preamble.author("Michael-F-Bryan");

    doc.push(Element::TitlePage)
        .push(Element::ClearPage)
        .push(Element::TableOfContents)
        .push(Element::ClearPage);

    let mut section_1 = Section::new("Section 1");
    section_1
        .push("Here is some text which will be put in paragraph 1.")
        .push("And here is some more text for paragraph 2.");
    doc.push(section_1);

    let mut section_2 = Section::new("Section 2");
    section_2.push("More text...");
    doc.push(section_2);

    doc
}
fn main() {
    let doc = create_document();
    let rendered = print(&doc).unwrap();
    println!("{}", rendered);
    let latex_hello_world = r#"
\documentclass{article}
\begin{document}
Hello, world!
\end{document}
"#;
    /*tectonic::test_util::activate_test_mode_augmented(env!(
        "CARGO_MANIFEST_DIR"
    ));*/
    let pdf_data: Vec<u8> =
        latex_to_pdf(latex_hello_world).expect("pocessing failed");
    println!("ok");
    println!("Output PDF size is {} bytes", pdf_data.len());
}
