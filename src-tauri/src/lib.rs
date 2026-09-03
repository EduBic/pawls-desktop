use std::path::PathBuf;

use crate::model::{
    Allocation, Annotation, Label, Page, PageTokens, PaperStatus, PdfAnnotations, RelationGroup,
};
use anyhow::Result;
use tauri::ipc::Response;
use tauri::Manager;

mod model;
mod preprocess;

#[tauri::command]
fn get_pdf(sha: String) -> Result<Response, String> {
    println!("get_pdf(sha={})", sha);

    // --- DEFAULT SETTINGS ---
    let output_directory = "allocated_pdfs";
    // --- END DEFAULT ---

    let path = PathBuf::from(output_directory).join(sha);
    let data = std::fs::read(&path).map_err(|e| format!("Failed to read PDF: {}", e))?;

    Ok(Response::new(data))
}

#[tauri::command]
async fn get_tokens(sha: String) -> Result<Vec<PageTokens>, String> {
    println!("get_tokens(sha={})", sha);

    // --- DEFAULT SETTINGS ---
    let output_directory = "allocated_pdfs";
    // --- END DEFAULT ---

    let path = PathBuf::from(output_directory).join(sha);
    let result = preprocess::process_pdfplumber(&path).unwrap();

    Ok(result
        .into_iter()
        .map(|p| PageTokens {
            tokens: p.tokens.clone(),
            page: p,
        })
        .collect())
}

#[tauri::command]
async fn get_labels() -> Result<Vec<Label>, String> {
    println!("get_labels()");

    // --- DEFAULT SETTINGS ---
    Ok(vec![
        Label {
            text: "Relation Success".into(),
            color: "#81f38a".into(),
        },
        Label {
            text: "Relation Warning".into(),
            color: "#ffd986".into(),
        },
        Label {
            text: "Relation Info".into(),
            color: "#97d2ff".into(),
        },
    ])
}

#[tauri::command]
async fn get_relations() -> Result<Vec<Label>, String> {
    println!("get_relations()");

    // --- DEFAULT SETTINGS ---
    Ok(vec![
        Label {
            text: "Success".into(),
            color: "#81f38a".into(),
        },
        Label {
            text: "Warning".into(),
            color: "#ffd986".into(),
        },
        Label {
            text: "Info".into(),
            color: "#97d2ff".into(),
        },
    ])
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

    Ok(PdfAnnotations {
        annotations: vec![],
        relations: vec![],
    })
}

#[tauri::command]
async fn get_allocated_paper_status() -> Result<Allocation, String> {
    println!("get_allocated_paper_status()");

    // --- DEFAULT SETTINGS ---
    let email = "local_user";
    let output_directory = "allocated_pdfs";
    // --- END DEFAULT ---

    let status_dir = PathBuf::from(&output_directory).join("status");
    let status_path = status_dir.join(format!("{}.json", email));

    if !status_path.exists() {
        // User doesn't have allocated papers.
        // They can see all PDFs but cannot save anything.
        let papers = all_pdf_shas(&output_directory)
            .into_iter()
            .map(|sha| PaperStatus::empty(&sha, &sha))
            .collect();

        Ok(Allocation {
            papers,
            has_allocated_papers: false,
        })
    } else {
        let contents = fs::read_to_string(&status_path)
            .map_err(|e| format!("Failed to read allocation status: {e}"))?;

        let status_json: std::collections::HashMap<String, PaperStatus> =
            serde_json::from_str(&contents)
                .map_err(|e| format!("Failed to parse allocation status: {e}"))?;

        let papers = status_json.into_values().collect();

        Ok(Allocation {
            papers,
            has_allocated_papers: true,
        })
    }
}

use std::fs;

fn all_pdf_shas(output_directory: &str) -> Vec<String> {
    let mut shas = Vec::new();

    println!("{}", output_directory);

    if let Ok(entries) = fs::read_dir(output_directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            println!("  - {:?}", path);

            if path.is_file() {
                if path.extension().is_some_and(|ext| ext == "pdf") {
                    if let Some(sha) = path.file_name() {
                        shas.push(sha.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    shas
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
            get_pdf,
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
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();

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
