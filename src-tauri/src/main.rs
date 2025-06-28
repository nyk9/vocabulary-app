#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::Manager;
use dotenvy::dotenv;
use std::env;

// --- AI Grading Structures ---
#[derive(Serialize, Deserialize, Debug)]
struct GradingResponse {
    score: u32,
    feedback: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    content: Content,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Part {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiApiResponse {
    candidates: Vec<Candidate>,
}

// --- End AI Grading Structures ---

// グローバルな状態として単語リストを管理
static WORDS: Lazy<Mutex<Vec<Word>>> = Lazy::new(|| Mutex::new(Vec::new()));
// グローバルな状態として日付リストを管理
static DATES: Lazy<Mutex<Vec<Date>>> = Lazy::new(|| Mutex::new(Vec::new()));

// 品詞の型を定義（Enum）
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
enum PartOfSpeech {
    Noun, // 名詞
    Verb, // 動詞
    Adjective, // 形容詞
    Adverb, // 副詞
    Pronoun, // 代名詞
    AuxiliaryVerb, // 助動詞
    Article, // 冠詞
    Conjunction, // 接続詞
    Preposition, // 前置詞
    Interjection, // 感嘆詞
    Other, // その他
}

// 単語の型を定義
#[derive(Serialize, Deserialize, Clone)]
struct Word {
    id: u32,
    vocabulary: String,
    meaning: String,
    translate: String,
    category: String,
    part_of_speech: Vec<PartOfSpeech>,
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
    part_of_speech: Vec<PartOfSpeech>,
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
        category,
        part_of_speech,
        example,
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
    part_of_speech: Vec<PartOfSpeech>,
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
                part_of_speech,
            };
        } else {
            return Err(format!("ID: {} が存在しません", id));
        };
    }
    save_words_to_file(app_handle.clone()).await?;

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    println!("今日の日付: {}", today);

    let date_to_update = Date {
        date: today,
        add: 0,
        update: 1,
        quiz: None,
    };

    println!("日付統計を更新します...");
    match add_date(date_to_update, "update".to_string(), app_handle).await {
        Ok(_) => println!("日付統計の更新に成功しました"),
        Err(e) => {
            println!("日付統計の更新に失敗しました: {}", e);
            return Err(format!("日付統計の更新に失敗: {}", e));
        }
    }
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

// 日付データが存在しない日を補完する関数
async fn ensure_date_records_exist(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let mut missing_dates_added = false;
    {
        let mut dates = DATES
            .lock()
            .map_err(|e| format!("Failed to lock dates: {:?}", e))?;

        if dates.is_empty() {
            return Ok(());
        }

        dates.sort_by_key(|d| d.date.clone());

        if let Some(last_date_entry) = dates.last() {
            let last_date_str = &last_date_entry.date;
            let today = chrono::Local::now().date_naive();
            let mut current_date = chrono::NaiveDate::parse_from_str(last_date_str, "%Y-%m-%d")
                .map_err(|e| format!("Failed to parse last date: {}", e))?
                + chrono::Duration::days(1);

            let mut dates_to_add = Vec::new();
            while current_date <= today {
                let date_str = current_date.format("%Y-%m-%d").to_string();
                dates_to_add.push(Date {
                    date: date_str,
                    add: 0,
                    update: 0,
                    quiz: Some(0),
                });
                current_date += chrono::Duration::days(1);
            }

            if !dates_to_add.is_empty() {
                println!("Adding {} missing date records.", dates_to_add.len());
                dates.extend(dates_to_add);
                missing_dates_added = true;
            }
        }
    }

    if missing_dates_added {
        save_dates_to_file(app_handle.clone()).await?;
        println!("Successfully saved missing date records.");
    }

    Ok(())
}

fn build_prompt(vocabulary: &str, meaning: &str, user_answer: &str) -> String {
    format!(
        r#"
あなたは英語学習の専門家です。以下の情報に基づいて、ユーザーの解答を採点し、フィードバックを提供してください。

### 指示
1.  **役割**: あなたは、ユーザーの英語の理解度を評価する親切な家庭教師です。
2.  **採点基準**:
    *   **正確性**: ユーザーの解答が、提示された「正しい意味」と合致しているか。
    *   **自然さ**: 英語の表現として自然か、不自然な点はないか。
    *   **具体性**: 例文として適切か、具体的に状況を説明できているか。
3.  **評価**: 上記の基準に基づき、ユーザーの解答を0点から100点の範囲で採点してください。
4.  **フィードバック**:
    *   必ず良かった点を1つ以上挙げてください。
    *   改善できる点があれば、具体的に指摘し、より良い表現の例を提示してください。
    *   全体を通して、ユーザーの学習意欲を高めるような、ポジティブで丁寧な言葉遣いを心がけてください。
5.  **出力形式**: 採点結果は、必ず以下のJSON形式で返してください。他のテキストは一切含めないでください。
    ```json
    {{
      "score": <0-100の整数>,
      "feedback": "<フィードバックの文字列>"
    }}
    ```

### 問題
*   **単語**: {}
*   **正しい意味**: {}

### ユーザーの解答
{}
"#,
        vocabulary, meaning, user_answer
    )
}

async fn grade_with_gemini(
    api_key: &str,
    client: &reqwest::Client,
    vocabulary: &str,
    meaning: &str,
    user_answer: &str,
) -> Result<GradingResponse, String> {
    let prompt = build_prompt(vocabulary, meaning, user_answer);
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}", api_key);

    let payload = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }]
    });

    let res = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Gemini request failed: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Gemini API error: {}", res.status()));
    }

    let gemini_response: GeminiApiResponse = res
        .json()
        .await
        .map_err(|e| format!("Failed to parse Gemini response: {}", e))?;

    if let Some(candidate) = gemini_response.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            // AIの出力からJSON部分だけを抽出する
            let json_text = if let (Some(start), Some(end)) = (part.text.find('{'), part.text.rfind('}')) {
                &part.text[start..=end]
            } else {
                &part.text
            };

            return serde_json::from_str(json_text)
                .map_err(|e| format!("Failed to deserialize Gemini JSON: {}. Raw text: {}", e, part.text));
        }
    }

    Err("No content found in Gemini response".to_string())
}


#[tauri::command]
async fn grade_answer(
    vocabulary: String,
    meaning: String,
    user_answer: String,
) -> Result<GradingResponse, String> {
    dotenv().ok(); // .envファイルを読み込む

    let client = reqwest::Client::new();

    // 1. Gemini APIを試す
    if let Ok(api_key) = env::var("GEMINI_API_KEY") {
        println!("Attempting to grade with Gemini...");
        match grade_with_gemini(&api_key, &client, &vocabulary, &meaning, &user_answer).await {
            Ok(response) => {
                println!("Successfully graded with Gemini.");
                return Ok(response);
            }
            Err(e) => {
                eprintln!("Gemini API failed: {}. Falling back to Claude.", e);
            }
        }
    } else {
        eprintln!("GEMINI_API_KEY not found. Skipping Gemini.");
    }

    // 2. Geminiが失敗した場合、Claude APIを試す (フォールバック)
    // Note: Claudeの実装は、APIの仕様に合わせて別途追加する必要があります。
    // ここでは、フォールバックのロジックを示すためのプレースホルダーです。
    if let Ok(_api_key) = env::var("CLAUDE_API_KEY") {
         eprintln!("Claude fallback is not yet implemented.");
        // match grade_with_claude(&api_key, &client, &vocabulary, &meaning, &user_answer).await {
        //     Ok(response) => {
        //         println!("Successfully graded with Claude.");
        //         return Ok(response);
        //     }
        //     Err(e) => {
        //         eprintln!("Claude API failed: {}", e);
        //     }
        // }
    } else {
        eprintln!("CLAUDE_API_KEY not found. Skipping Claude.");
    }


    Err("All AI grading services failed.".to_string())
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
                if let Err(e) = load_dates_from_file(&app_handle).await {
                    eprintln!("Error loading dates: {}", e);
                }
                if let Err(e) = ensure_date_records_exist(&app_handle).await {
                    eprintln!("Error ensuring date records exist: {}", e);
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
            grade_answer,
        ])
        // 実行
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
