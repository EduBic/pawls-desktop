use crate::model::{Allocation, Annotation, Label, PdfAnnotations, RelationGroup};
use anyhow::Result;

mod model;

#[tauri::command]
async fn get_tokens(sha: String) -> Result<Vec<String>, String> {
    println!("get_tokens(sha={})", sha);
    // ...

    todo!()
}

#[tauri::command]
async fn get_labels() -> Result<Vec<Label>, String> {
    println!("get_labels()");
    // ...
    todo!()
}

#[tauri::command]
async fn get_relations() -> Result<Vec<Label>, String> {
    println!("get_relations()");
    // ...
    todo!()
}

#[tauri::command]
async fn set_pdf_comment(sha: String, comments: String) -> Result<(), String> {
    println!("set_pdf_comment(sha={}, comments={})", sha, comments);
    // ...
    todo!()
}

#[tauri::command]
async fn set_pdf_junk(sha: String, junk: bool) -> Result<(), String> {
    println!("set_pdf_junk(sha={}, junk={})", sha, junk);
    // ...
    todo!()
}

#[tauri::command]
async fn set_pdf_finished(sha: String, finished: bool) -> Result<(), String> {
    println!("set_pdf_finished(sha={}, finished={})", sha, finished);
    // ...
    todo!()
}

#[tauri::command]
async fn get_annotations(sha: String) -> Result<PdfAnnotations, String> {
    println!("get_annotations(sha={})", sha);
    // ...

    todo!()
}

#[tauri::command]
async fn get_allocated_paper_status() -> Result<Allocation, String> {
    println!("get_allocated_paper_status()");
    // ...
    todo!()
}

#[tauri::command]
async fn save_annotations(
    sha: String,
    annotations: Vec<Annotation>,
    relations: Vec<RelationGroup>,
) -> Result<(), String> {
    println!(
        "save_annotations(sha={}, annotations={}, relations={})",
        sha,
        annotations.len(),
        relations.len()
    );
    // ...
    todo!()
}

// ----------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tokens,
            get_labels,
            get_relations,
            set_pdf_comment,
            set_pdf_finished,
            set_pdf_junk,
            get_allocated_paper_status,
            save_annotations,
            get_annotations,
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
