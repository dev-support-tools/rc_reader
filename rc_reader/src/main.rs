mod models;
mod resource_reader;
mod file_reader;

use std::{default, fs, rc};

use glob::glob;
use models::resource_file::{ClassInfo, CodeInfo, ResourceFile};



// ①リソースファイル読み込み
//  Dialog、StringTable、Controlを作成
// ②ヘッダーファイル読み込み
// ③Dialogにヘッダーファイルの情報を追加
//  コード解析
//  コードにIDDが含まれているか確認
//  コードにIDDが含まれている場合、Dialogにコード情報を追加
//  DialogとStringTableの紐づけ



// rootパスの定数
const ROOT_PATH: &str = r"../mfc_app/";

// ファイル一覧
const RC_FILES : &str = r"rc_files.json";

// リソース情報
const RESOURCE_INFOS : &str = r"resource_infos.json";

// コード情報
const CODE_INFOS : &str = r"code_infos.json";

// コードとリソース情報
const CODE_DIALOG_INFOS : &str = r"code_dialog_infos.json";

fn main() {

    let mut is_loop = true;
    while is_loop{
        println!("1:リソースファイル列挙する。.rcファイルの一覧を作成。 {}", RC_FILES);
        println!("2:リソースファイル読み込み .rcファイルの中身を横こむ。{}", RESOURCE_INFOS);
        println!("3:ヘッダー、実装ファイル読み込み。.h/.cppファイルを列挙して中身を読み込む。クラス、基底クラス、Dialogを出力 {}", CODE_INFOS);
        println!("4:Dialogに.rcファイルの情報を追加 {}", CODE_DIALOG_INFOS);
        // キーボードの入力を待つ
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => list_resource_files(),
            "2" => read_resource_files(),
            "3" => create_code_info(),
            "4" => create_dialog_info(),
            "5" => create_dialog_svg_files(),
            _ => {
                is_loop = false;
            }
        }
    }
}

fn create_dialog_svg_files() {
    // resouce_infos.jsonファイルの読み込み
    let resource_infos_json = fs::read_to_string(RESOURCE_INFOS).unwrap();
    let mut resource_files: Vec<ResourceFile> = serde_json::from_str(&resource_infos_json).unwrap();

    for resource_file in &mut resource_files {
        for dialog in &mut resource_file.dialogs {
            let svg = dialog.create_svg();
            let svg_file_path = format!(r"{}/{}.svg", ".", dialog.id);
            std::fs::write(svg_file_path, svg).unwrap();
        }
    }
}

// リソースファイル列挙
fn list_resource_files() {
    // リソースファイル一覧
    let sample_rc_path = format!("{}{}", ROOT_PATH, r"**/*.rc") ;
    let rc_files = glob(&sample_rc_path).unwrap();

    let mut resource_files: Vec<String> = Vec::new();
    for rc_file in rc_files {
        // rc_fileのパスを取得
        let rc_file = rc_file.unwrap();
        let rc_file_path = rc_file.to_str().unwrap();
        resource_files.push(rc_file_path.to_string());
    }

    // jsonファイルとしてファイルに出力
    let json = serde_json::to_string_pretty(&resource_files).unwrap();
    let json_file_path = format!(r"{}/{}", ".", RC_FILES);
    std::fs::write(json_file_path, json).unwrap();
}   

fn read_resource_files() {
    // rc_files.jsonファイルの読み込み
    let rc_files_json = fs::read_to_string(RC_FILES).unwrap();
    let mut resource_files: Vec<String> = serde_json::from_str(&rc_files_json).unwrap();

    let mut resource_infos = resource_reader::read_resource_files(resource_files);
    // jsonファイルとしてファイルに出力
    let json = serde_json::to_string_pretty(&resource_infos).unwrap();
    let json_file_path = format!(r"{}/{}", ".", RESOURCE_INFOS);
    std::fs::write(json_file_path, json).unwrap();
}



fn create_code_info() {
    let sample_h_path = format!("{}{}", ROOT_PATH, r"**/*.h") ;
    let header_files = glob(&sample_h_path).unwrap();
    let mut code_infos:Vec<CodeInfo> = Vec::new();
    for header_file in header_files {
        let header_file = header_file.unwrap();
        let header_file_path = header_file.to_str().unwrap();
        // ファイル読み込み
        let text = fs::read_to_string(header_file_path).unwrap();
        let text_lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();

        let mut class_info: ClassInfo = ClassInfo::default();
        // classが含まれている行はクラス定義とみなす
        let class_lines = text_lines.iter().filter(|line| line.contains("class")).collect::<Vec<&String>>();
        if class_lines.len() > 0 {
            // クラス名
            let class_line = class_lines[0].clone();
            let class_name = class_line.split_whitespace().collect::<Vec<&str>>()[1];
            // ベースクラス名
            let base_class_name = class_lines[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().collect::<Vec<&str>>()[1];
            class_info = ClassInfo {
                class_name: class_name.to_string(),
                base_class_name: base_class_name.to_string(),
                dialog: None,
            };
        }


        let mut code_text = String::new();
        let code_file_path = &header_file_path.replace(".h", ".cpp");
        // ファイルがしている場合は読み込み
        if std::path::Path::new(code_file_path).exists() {
            code_text = fs::read_to_string(code_file_path).unwrap();
        }
        let code_info = CodeInfo {
            header_file_path: String::from(header_file_path),
            header_file_lines: text_lines,
            code_file_path: String::from(code_file_path),
            code_file_lines: code_text.lines().map(|s| s.to_string()).collect(),
            reference_string_table_ids: vec![],
            class_info,
        };
        code_infos.push(code_info);
    }
    // jsonファイルとしてファイルに出力
    let json = serde_json::to_string_pretty(&code_infos).unwrap();
    let json_file_path = format!(r"{}/{}", ".", CODE_INFOS);
    std::fs::write(json_file_path, json).unwrap();
}




fn create_dialog_info() {
    // resouce_infos.jsonファイルの読み込み
    let resource_infos_json = fs::read_to_string(RESOURCE_INFOS).unwrap();
    let mut resource_files: Vec<ResourceFile> = serde_json::from_str(&resource_infos_json).unwrap();
    
    // code_infos.jsonファイルの読み込み
    let code_infos_json = fs::read_to_string(CODE_INFOS).unwrap();
    let mut code_infos: Vec<CodeInfo> = serde_json::from_str(&code_infos_json).unwrap();


    // ③Dialogにヘッダーファイルの情報を追加
    for resource_file in &mut resource_files {
        for code_info in &mut code_infos {
            for dialog in &mut resource_file.dialogs {
                // .h or .cppに含まれているIDD = IDD_の行にDialogのIDDが含まれているか確認
                let search_text = format!("IDD = {} ", dialog.id);
                // .hにIDDが含まれているか確認
                let mut idd_line = code_info.header_file_lines.iter().find(|line| line.contains(&search_text)).unwrap_or(&String::new()).clone();
                if idd_line.len() == 0 {
                    // .cppにIDDが含まれているか確認
                    idd_line = code_info.code_file_lines.iter().find(|line| line.contains(&search_text)).unwrap_or(&String::new()).clone();
                }
                if idd_line.len() == 0 {
                    // DialogのIDDが含まれていない場合は次のDialogへ
                    continue;
                }
                // Dialogにコード情報を追加
                dialog.code_infos.push(code_info.clone());

                code_info.class_info.dialog = Some(dialog.clone());
                // コードに参照しているStringTableのIDを追加
                let mut string_table_ids = code_info.reference_string_table_ids.clone();
                // string_table_idsを長い順位にソート
                string_table_ids.sort_by(|a, b| b.id.len().cmp(&a.id.len()));
                for cpp_line in &code_info.code_file_lines {
                    let mut cpp_line = cpp_line.clone();
                    for string_table in &string_table_ids {
                        if !cpp_line.contains(&string_table.id) {
                            continue;
                        }
                        dialog.reference_string_table_ids.push(string_table.clone());
                        cpp_line = cpp_line.replace(&string_table.id, "");
                    }
                }
            }
        }
        break;
    }
    // jsonファイルとしてファイルに出力
    let json = serde_json::to_string_pretty(&code_infos).unwrap();
    let json_file_path = format!(r"{}/{}", ".", CODE_DIALOG_INFOS);
    std::fs::write(json_file_path, json).unwrap();
    
}