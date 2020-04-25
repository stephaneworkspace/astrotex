#![feature(vec_into_raw_parts)]
extern crate latex;
extern crate nsvg;
extern crate tectonic;
use astrology::svg_draw::{
    chart, DataChartNatal, DataObjectSvg, DataObjectType,
};
use base64::encode;
use latex::{print, Document, DocumentClass, Element, Section};
use libswe_sys::sweconst::Language;
use std::ffi::{CStr, CString};
use tectonic::latex_to_pdf;
/*
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
*/
fn convert_using_into_raw_parts(v: Vec<u8>) -> Vec<u32> {
    let (ptr, len, cap) = v.into_raw_parts();
    unsafe { Vec::from_raw_parts(ptr as *mut u32, len, cap) }
}

fn svg() -> String {
    let path = CString::new(
        "/Users/stephanebressani/Code/Flutter/astro/ios/EphemFiles/",
    )
    .expect("CString::new failled");
    let d = DataChartNatal {
        year: 2020,
        month: 4,
        day: 25,
        hour: 18,
        min: 0,
        sec: 0.0 as f32,
        lat: 40.0 as f32,
        lng: 19.0 as f32,
    };
    let path_c_str = unsafe { CStr::from_ptr(path.as_ptr()) };
    let path_str: &str = path_c_str.to_str().unwrap();
    let res: Vec<DataObjectSvg> =
        chart(1000.0, d, &path_str, Language::English);
    let mut svg_res: String = "".to_string();
    for r in res.clone() {
        if r.object_type == DataObjectType::Chart {
            svg_res = r.svg;
        }
    }
    if svg_res != "" {
        svg_res = svg_res.replace("</svg>", "");
        for r in res {
            if r.object_type != DataObjectType::Chart {
                // to do better inside after for real use
                svg_res = format!("{}<image width=\"{}\" height=\"{}\" x=\"{}\" y=\"{}\" href=\"data:image/svg+xml;base64,{}\"/>", svg_res, r.size_x, r.size_y, r.pos_x, r.pos_y, encode(r.svg.as_str()));
            }
        }
    } else {
        svg_res = "<svg>".to_string();
    }
    svg_res = format!("{}</svg>", svg_res);
    let n = nsvg::parse_str(&svg_res, nsvg::Units::Pixel, 96.0).unwrap();
    let scale = 2.0;
    let image = n.rasterize(scale).unwrap();
    let (w_r, h_r, raw_rgba) = n.rasterize_to_raw_rgba(2.0).unwrap();
    let raw_final: Vec<u32> = convert_using_into_raw_parts(raw_rgba);
    const OUTPUT: &str = "/Users/stephanebressani/Code/Rust/astrotex/chart.png";

    let save_path = OUTPUT;
    let (width, height) = image.clone().dimensions();

    // Write the image to disk as a PNG
    image::save_buffer(
        save_path.clone(),
        &image.into_raw(),
        500, // width
        500, // height
        image::ColorType::RGBA(8),
    )
    .expect("Failed to save png.");

    //println!("File exported to: {:?}", image);
    svg_res
}

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
    let svg_str = svg();
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
