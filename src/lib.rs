use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

///
/// ファイルを読取り、SQL文を返します
///
#[no_mangle]
pub extern "C" fn read_sql_file(file_name: *const c_char) -> *mut c_char {
    let f_name = unsafe {CStr::from_ptr(file_name)}.to_str().unwrap();
    let file = File::open(f_name).expect("file not found");

    // ファイルの読取
    let buf = BufReader::new(file);
    // SQL文の初期化
    let mut sql_string = String::from("");
    for line in buf.lines() {
        // ファイルから一行ずつ読取る
        let sql= &line.unwrap();
        // //で始まる行はコメントとして無視。空白行も無視
        if substring(&sql, 0, 2) != "//" && !trim(sql).is_empty() {
            if sql.contains(";") {
                sql_string.push_str(sql);
                println!("SQL:{}", sql_string.as_str());
                break;

            }
            else {
                // ;がない場合は改行を追加し、SQL文に追加
                let return_code = "\n";
                sql_string.push_str(sql);
                sql_string.push_str(&return_code);
            }
        }
    }

    return  CString::new(sql_string).unwrap().into_raw();
}

///
///  文字列を解放します
///
pub extern "C" fn free_string(ptr: *mut c_char) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        let _ = CString::from_raw(ptr);
    }
}

/// * `s` - 対象の文字列
/// * `start` - 開始位置
/// * `length` - 長さ
fn substring(s: &str, start: usize, length: usize) -> &str {
    if length == 0 {
        return "";
    }

    let mut ci = s.char_indices();
    let start_byte = match ci.nth(start) {
        Some(i) => i.0,
        None => return "",
    };

    match ci.nth(length - 1) {
        Some(j) => &s[start_byte..j.0],
        None => &s[start_byte..],
    }
}

/// 半角/全角の文字列をトリムします
///
/// * `s` - 対象の文字列
fn trim(s: &str) -> &str {
    let text = s.trim();
    text.trim_end_matches(|c: char| c.is_whitespace())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let name = CString::new("E:\\projects\\ShareSqlLib\\select_history.sql").unwrap().into_raw();
        let sql = read_sql_file(name);
        let sql_line = unsafe {CStr::from_ptr(sql)}.to_str().unwrap();
        println!("SQL:{}", sql_line);

        free_string(sql);
    }
}
