#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

// グローバルな状態として単語リストを管理
static WORDS: Lazy<Mutex<Vec<Word>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Serialize, Deserialize, Clone)]
struct Word {
    id: u32,
    vocabulary: String,
    meaning: String,
    translate: String,
    category: String,
    example: Option<String>,
}

// JSON ファイルのパスを取得
fn get_words_file_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app dir: {}", e))
        .map(|dir| dir.join("words.json"))
}

// 新しいIDを生成
fn generate_id() -> Result<u32, String> {
    let words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    Ok(match words.last() {
        Some(word) => word.id + 1,
        None => 1,
    })
}

#[tauri::command]
fn get_words() -> Result<Vec<Word>, String> {
    let words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    Ok(words.clone())
}

// 単語をJSONファイルに保存するコマンド
#[tauri::command]
async fn save_words_to_file(app_handle: tauri::AppHandle) -> Result<(), String> {
    let words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    let json = serde_json::to_string_pretty(&*words).map_err(|e| e.to_string())?;

    let path = get_words_file_path(&app_handle)?;

    // 標準ライブラリを使ってディレクトリが存在することを確認
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 標準ライブラリを使ってファイルに書き込む
    fs::write(&path, json).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

// JSONファイルから単語を読み込む関数
async fn load_words_from_file(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let path = get_words_file_path(app_handle)?;

    // 標準ライブラリを使ってファイルが存在するか確認
    if !path.exists() {
        return Ok(());
    }

    // 標準ライブラリを使ってファイルを読み込む
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // JSONをパースする
    let words: Vec<Word> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // グローバル状態を更新
    let mut global_words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    *global_words = words;
    println!("Loaded {} words from storage", global_words.len());

    Ok(())
}

#[tauri::command]
async fn add_word(
    app_handle: tauri::AppHandle,
    vocabulary: String,
    meaning: String,
    translate: String,
    example: Option<String>,
    category: String,
) -> Result<(), String> {
    let new_word = Word {
        id: generate_id()?,
        vocabulary,
        meaning,
        translate,
        example,
        category,
    };

    {
        let mut words = WORDS
            .lock()
            .map_err(|e| format!("Failed to lock words: {:?}", e))?;
        words.push(new_word);
    }

    save_words_to_file(app_handle).await?;
    Ok(())
}

#[tauri::command]
async fn delete_word(_id: u32) -> Result<(), String> {
    let _words = get_words();

    Ok(())
}

fn main() {
    tauri::Builder::default()
        // tauri-plugin-fsはプラグインとしては必要なくなりました
        // .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // タスクを使用して非同期的にファイルを読み込む
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = load_words_from_file(&app_handle).await {
                    eprintln!("Error loading words: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_words,
            add_word,
            save_words_to_file,
            delete_word
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
