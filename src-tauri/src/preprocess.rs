use std::path::Path;

use anyhow::{Context, Result};
use pdfplumber::{Pdf, WordOptions};

use crate::model::{Page, Token};

pub fn process_pdfplumber(pdf_file: impl AsRef<Path>) -> Result<Vec<Page>> {
    let pdf = Pdf::open_file(pdf_file, None).with_context(|| format!("failed to open PDF"))?;

    let mut pages = Vec::new();

    for page_result in pdf.pages_iter() {
        let page = page_result.context("failed to parse PDF page")?;

        let width = page.width();
        let height = page.height();
        let index = page.page_number();

        let options = WordOptions {
            x_tolerance: 1.5,
            y_tolerance: 3.0,
            keep_blank_chars: false,
            use_text_flow: true,
            ..Default::default()
        };

        let words = page.extract_words(&options);

        let tokens = words
            .into_iter()
            .map(|word| {
                // pdfplumber uses:
                //
                // x0    -> left
                // x1    -> right
                // top   -> top
                // bottom -> bottom
                //
                // Clip the word bounding box to the page,
                // equivalent to the Python implementation.

                let x0 = word.bbox.x0.clamp(0.0, width);
                let x1 = word.bbox.x1.clamp(0.0, width);
                let top = word.bbox.top.clamp(0.0, height);
                let bottom = word.bbox.bottom.clamp(0.0, height);

                Token {
                    text: word.text,
                    x: x0,
                    width: x1 - x0,
                    y: top,
                    height: bottom - top,
                }
            })
            .collect();

        pages.push(Page {
            width,
            height,
            index,
            tokens,
        });
    }

    Ok(pages)
}
