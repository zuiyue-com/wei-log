use chrono::prelude::*;

use std::fs::{OpenOptions};
use std::io::{BufReader, BufRead, BufWriter, Write, Seek, SeekFrom};
use std::env;
pub fn log(s: &str) {
    let path = std::env::current_exe().unwrap();
    let filename = std::path::Path::new(&path).file_name().unwrap().to_str().unwrap();

    let home_dir = get_home_dir().unwrap_or_else(|| String::from("."));
    let path;
    if cfg!(target_os = "windows") {
        path = format!("{}/AppData/Local/wei/{}.log.txt", home_dir, filename);
    } else {
        path = format!("{}/.wei/{}.log.txt", home_dir, filename);
    }
    let local: DateTime<Local> = Local::now();
    let data = format!("{} {}",local.format("%Y-%m-%d %H:%M"), s);

    // let _ = write_and_prune_file(&path, &data, 100);

    let mut file = OpenOptions::new().read(true).write(true).create(true).open(&path).unwrap();
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    lines.push(data);
    if lines.len() > 100 {
        lines.remove(0);
    }
    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();  // Truncate the file
    let mut writer = BufWriter::new(&file);
    for line in &lines {
        writeln!(writer, "{}", line).unwrap();
    }

}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        crate::wei_log::log(&message); // 将格式化后的字符串传递给函数
        // println!("{}", message);
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        let message = format!("错误: {}", message);
        crate::wei_log::log(&message); // 将格式化后的字符串传递给函数
        // println!("{}", message);
    }}
}

#[macro_export]
macro_rules! info_println {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        crate::wei_log::log(&message); // 将格式化后的字符串传递给函数
        // println!("{}", message);
    }}
}

#[macro_export]
macro_rules! info_print {
    ($($arg:tt)*) => {{
        let message = format!($($arg)*);
        crate::wei_log::log(&message); // 将格式化后的字符串传递给函数
        // print!("{}", message);
    }}
}


fn get_home_dir() -> Option<String> {
    if cfg!(target_os = "windows") {
        env::var("USERPROFILE").ok()
    } else {
        env::var("HOME").ok()
    }
}



// fn write_and_prune_file(path: &str, content: &str, max_lines: usize) -> std::io::Result<()> {
//     let mut file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

//     // Step 1: Read all lines
//     let reader = BufReader::new(&file);
//     let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

//     // Step 2: Prune lines if necessary
//     if lines.len() >= max_lines {
//         lines.remove(lines.len() - 1);
//     }

//     // Step 3: Insert new line at the top
//     lines.insert(0, content.to_string());

//     // Step 4: Seek to the start of the file and write all lines
//     file.seek(SeekFrom::Start(0))?;
//     file.set_len(0)?;  // Truncate the file
//     for line in &lines {
//         writeln!(file, "{}", line)?;
//     }
//     Ok(())
// }



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
