#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;

// グローバルな状態として単語リストを管理
static WORDS: Lazy<Mutex<Vec<Word>>> = Lazy::new(|| Mutex::new(Vec::new()));
// グローバルな状態として日付リストを管理
static DATES: Lazy<Mutex<Vec<Date>>> = Lazy::new(|| Mutex::new(Vec::new()));

// 単語の型を定義
#[derive(Serialize, Deserialize, Clone)]
struct Word {
    id: u32,
    vocabulary: String,
    meaning: String,
    translate: String,
    category: String,
    example: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Date {
    date: String,
    add: u32,
    update: u32,
    quiz: Option<u32>,
}

// 単語のJSONファイルのパスを取得
fn get_words_file_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app dir: {}", e))
        .map(|dir| dir.join("words.json"))
}

// 追加・更新日時のJSONファイルのパスを取得
fn get_dates_file_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app dir: {}", e))
        .map(|dir| dir.join("date.json"))
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

// 全単語を取得する関数
#[tauri::command]
fn get_words() -> Result<Vec<Word>, String> {
    let words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    Ok(words.clone())
}

// IDに基づいて単語のデータを取得する関数
#[tauri::command]
fn get_words_by_id(id: u32) -> Result<Word, String> {
    let words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;

    words
        .iter()
        .find(|word| word.id == id)
        .cloned()
        .ok_or_else(|| format!("ID: {} の単語が見つかりませんでした", id))
}

// 単語をJSONファイルに保存するコマンド
#[tauri::command]
async fn save_words_to_file(app_handle: tauri::AppHandle) -> Result<(), String> {
    let words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    let json = serde_json::to_string_pretty(&*words).map_err(|e| e.to_string())?;

    let path = get_words_file_path(&app_handle)?;
    println!("Attempting to save to path: {:?}", path);  // パスを表示

    // ディレクトリが存在するか確認
    if let Some(parent) = path.parent() {
        println!("Creating directory: {:?}", parent);  // ディレクトリパスを表示
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // ファイルに書き込む
    println!("Writing file with content length: {}", json.len());
    fs::write(&path, &json).map_err(|e| format!("Failed to write file: {}", e))?;
    println!("File written successfully");

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

// 追加した単語を保存する関数
#[tauri::command]
async fn add_word(
    vocabulary: String,
    meaning: String,
    translate: String,
    example: Option<String>,
    category: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    println!("==== 単語追加開始 ====");
    println!("単語: {}", vocabulary);
    println!("意味: {}", meaning);
    println!("翻訳: {}", translate);
    println!("例文: {:?}", example);
    println!("カテゴリ: {}", category);

    let new_word = Word {
        id: generate_id()?,
        vocabulary: vocabulary.clone(),
        meaning,
        translate,
        example,
        category,
    };

    println!("単語ID: {}", new_word.id);

    {
        let mut words = WORDS
            .lock()
            .map_err(|e| format!("単語リストのロック失敗: {:?}", e))?;
        words.push(new_word);
        println!("単語をメモリに追加しました。現在の単語数: {}", words.len());
    }

    // ファイル保存を試みる
    println!("単語をファイルに保存します...");
    match save_words_to_file(app_handle.clone()).await {
        Ok(_) => println!("単語の保存に成功しました"),
        Err(e) => {
            println!("単語の保存に失敗しました: {}", e);
            return Err(format!("単語の保存に失敗: {}", e));
        }
    }

    // 日付統計の更新
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("今日の日付: {}", today);

    let date_to_update = Date {
        date: today,
        add: 1,
        update: 0,
        quiz: None,
    };

    println!("日付統計を更新します...");
    match add_date(date_to_update, "add".to_string(), app_handle).await {
        Ok(_) => println!("日付統計の更新に成功しました"),
        Err(e) => {
            println!("日付統計の更新に失敗しました: {}", e);
            return Err(format!("日付統計の更新に失敗: {}", e));
        }
    }

    println!("==== 単語追加完了 ====");
    Ok(())
}

// 指定した単語を削除する関数
#[tauri::command]
async fn delete_word(id: u32, app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut words = WORDS
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;

    let path = get_words_file_path(&app_handle)?;

    // 標準ライブラリを使ってディレクトリが存在することを確認
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 引数のidと一致する単語を削除する。
    if let Some(index) = words.iter().position(|word| word.id == id) {
        words.remove(index);
    }

    // 指定した単語を削除した後の残りの単語リストをjsonに書き込む
    let json = serde_json::to_string(&*words).map_err(|e| e.to_string())?;

    // 標準ライブラリを使ってファイルに書き込む
    fs::write(&path, json).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

// 指定した単語を更新する関数
#[tauri::command]
async fn update_word(
    id: u32,
    vocabulary: String,
    meaning: String,
    translate: String,
    example: Option<String>,
    category: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut words = WORDS
            .lock()
            .map_err(|e| format!("Failed to lock words: {:?}", e))?;

        if let Some(index) = words.iter().position(|word| word.id == id) {
            words[index] = Word {
                id,
                vocabulary,
                meaning,
                translate,
                example,
                category,
            };
        } else {
            return Err(format!("ID: {} が存在しません", id));
        };
    }
    save_words_to_file(app_handle).await?;
    Ok(())
}

// JSONファイルから日付を読み込む関数
async fn load_dates_from_file(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let path = get_dates_file_path(app_handle)?;

    // 標準ライブラリを使ってファイルが存在するか確認
    if !path.exists() {
        return Ok(());
    }

    // 標準ライブラリを使ってファイルを読み込む
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    // JSONをパースする
    let dates: Vec<Date> =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // グローバル状態を更新
    let mut global_dates = DATES
        .lock()
        .map_err(|e| format!("Failed to lock dates: {:?}", e))?;
    *global_dates = dates;
    println!("Loaded {} dates from storage", global_dates.len());

    Ok(())
}

// 単語をJSONファイルに保存するコマンド
#[tauri::command]
async fn save_dates_to_file(app_handle: tauri::AppHandle) -> Result<(), String> {
    let dates = DATES
        .lock()
        .map_err(|e| format!("Failed to lock dates: {:?}", e))?;
    let json = serde_json::to_string_pretty(&*dates).map_err(|e| e.to_string())?;

    let path = get_dates_file_path(&app_handle)?;

    // 標準ライブラリを使ってディレクトリが存在することを確認
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 標準ライブラリを使ってファイルに書き込む
    fs::write(&path, json).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

// 追加した日を記録する関数
#[tauri::command]
fn get_dates() -> Result<Vec<Date>, String> {
    let dates = DATES
        .lock()
        .map_err(|e| format!("Failed to lock words: {:?}", e))?;
    Ok(dates.clone())
}

#[tauri::command]
async fn add_date(date: Date, mode: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    {
        let mut dates = DATES
            .lock()
            .map_err(|e| format!("Failed to lock words: {:?}", e))?;

        // 既存の日時を探す
        if let Some(existing_date) = dates.iter_mut().find(|d| d.date == date.date) {
            // モードに応じて既存のエントリを更新
            match mode.as_str() {
                "add" => existing_date.add += 1,
                "update" => existing_date.update += 1,
                "quiz" => {
                    if let Some(quiz) = existing_date.quiz {
                        existing_date.quiz = Some(quiz + 1);
                    } else {
                        existing_date.quiz = Some(1);
                    }
                }
                _ => return Err(format!("不明なモード: {}", mode)),
            }
        } else {
            // 日付が存在しない場合は新規追加
            dates.push(date);
        }
    }
    // 変更をファイルに保存
    save_dates_to_file(app_handle).await?;

    Ok(())
}


// メイン関数：Tauriで実行する
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
                if let Err(e) = load_dates_from_file(&app_handle).await {
                    eprintln!("Error loading dates: {}", e);
                }
            });
            Ok(())
        })
        // FEで関数を実行できるように設定
        .invoke_handler(tauri::generate_handler![
            get_words,
            get_words_by_id,
            add_word,
            save_words_to_file,
            delete_word,
            update_word,
            get_dates,
            add_date,
            save_dates_to_file,
        ])
        // 実行
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
