use std::borrow::Cow;

use crate::PROGRAM_NAME;
use tracing::error;

pub struct StartHelper {
    exe_name: String,
}

impl StartHelper {
    pub fn new(exe_name: String) -> Self {
        Self { exe_name }
    }

    pub fn set_auto_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        if cfg!(windows) {
            return self.set_win_auto_start();
        }
        #[cfg(target_os = "macos")]
        if cfg!(macos) {
            return self.set_mac_auto_start();
        }
        return Err("不支持的操作系统".into());
    }

    #[cfg(target_os = "windows")]
    fn set_win_auto_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // C:\Users\*\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup
        // 获取当前Windows用户的home directory.
        let win_user_home_dir =
            home::home_dir().ok_or_else(|| "获取当前Windows用户的home directory失败")?;
        let start_file = format!(
            r#"{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\{}_start.vbs"#,
            win_user_home_dir.to_str().unwrap(),
            self.exe_name
        );

        let path = std::env::current_dir().map_err(|err| {
            format!(
                "获取当前文件目录失败: {}",
                err.to_string().replace("\\", "\\\\")
            )
        })?;
        let path = path.to_str().unwrap().replace("\\", "\\\\");

        let content = String::from(r#"Set objShell = CreateObject("WScript.Shell")"#)
            + "\r\n"
            + &format!(r#"objShell.CurrentDirectory = "{}""#, path)
            + "\r\n"
            + &format!(r#"objShell.Run "powershell /c .\\{}""#, self.exe_name)
            + ",0";
        let content_bytes = utf8_to_gbk(&content)?;
        let old_content = std::fs::read(&start_file).unwrap_or_default();
        if old_content == content_bytes {
            tracing::debug!("start_file content not changed, skip");
            return Ok(());
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&start_file)?;
        use std::io::Write;
        file.write_all(&content_bytes)?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    fn set_mac_auto_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home_dir = home::home_dir().ok_or_else(|| "获取当前用户的home directory失败")?;
        let start_file = format!(
            "{}/Library/LaunchAgents/{}_start.plist",
            home_dir.to_str().unwrap(),
            self.exe_name
        );
        let cur_path = std::env::current_dir().map_err(|err| {
            format!(
                "获取当前文件目录失败: {}",
                err.to_string().replace("\\", "\\\\")
            )
        })?;
        let mac_list_file = format!(
            r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
            <plist version="1.0">
            <dict>
                <key>Label</key>
                <string>{0}_start</string>
                <key>ProgramArguments</key>
                    <array>
                        <string>{1}/{0}</string>
                    </array>
                <key>RunAtLoad</key>
                <true/>
                <key>WorkingDirectory</key>
                <string>/Applications/DownTip.app/Contents/MacOS</string>
                <key>StandardErrorPath</key>
                <string>/tmp/{0}_start.err</string>
                <key>StandardOutPath</key>
                <string>/tmp/{0}_start.out</string>
            </dict>
            </plist>
            "#,
            self.exe_name,
            cur_path.to_str().unwrap()
        );
        let old_content = std::fs::read(&start_file).unwrap_or_default();
        if old_content == mac_list_file.as_bytes() {
            tracing::debug!("start_file content not changed, skip");
            return Ok(());
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&start_file)?;
        use std::io::Write;
        file.write_all(mac_list_file.as_bytes())?;
        Ok(())
    }

    pub fn unset_auto_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        if cfg!(windows) {
            return self.unset_win_auto_start();
        }
        #[cfg(target_os = "macos")]
        if cfg!(macos) {
            return self.unset_mac_auto_start();
        }
        return Err("不支持的操作系统".into());
    }

    fn unset_win_auto_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let win_user_home_dir =
            home::home_dir().ok_or_else(|| "获取当前Windows用户的home directory失败")?;
        let start_file = format!(
            r#"{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\{}_start.vbs"#,
            win_user_home_dir.to_str().unwrap(),
            self.exe_name
        );

        if !std::path::Path::new(&start_file).exists() {
            return Ok(());
        }

        std::fs::remove_file(&start_file)?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn unset_mac_auto_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home_dir = home::home_dir().ok_or_else(|| "获取当前用户的home directory失败")?;
        let start_file = format!(
            "{}/Library/LaunchAgents/{}_start.plist",
            home_dir.to_str().unwrap(),
            self.exe_name
        );
        if !std::path::Path::new(&start_file).exists() {
            return Ok(());
        }
        std::fs::remove_file(&start_file)?;
        Ok(())
    }
}

fn utf8_to_gbk(b: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use encoding::all::GBK;
    use encoding::{EncoderTrap, Encoding};
    let content_bytes = GBK.encode(b, EncoderTrap::Strict)?;
    Ok(content_bytes)
}

/// 获取系统默认桌面路径
pub fn get_desktop_path() -> Result<String, String> {
    let home_dir = home::home_dir().ok_or_else(|| "get home dir error".to_string())?;
    let desktop_path = home_dir.join("Desktop");
    if !desktop_path.exists() {
        return Err("desktop path not exist in home dir".to_string());
    }
    Ok(desktop_path.to_str().unwrap().to_string())
}

/// 去除颜色
pub fn eliminate_color<'a>(line: &'a [u8]) -> Cow<'a, [u8]> {
    //"\033[31m 红色 \033[0m"
    if subslice::bmh::find(line, b"\x1b[0m").is_some() {
        let mut buf = Vec::with_capacity(line.len());
        let mut start = 0;
        let mut end;
        loop {
            if let Some(index) = subslice::bmh::find(&line[start..], b"\x1b[") {
                end = start + index;
                buf.extend_from_slice(&line[start..end]);
                // end的位置是\x1b的位置，end + 3 与 end + 4 一个是\x1b[0m，一个是\x1b[31m，以此类推，
                // 如果 end + 4 <= line.len()或者end + 5 <= line.len() 都不成立，
                // 说明字符串含有\x1b，但是\x1b[0m或者\x1b[31m不完整，或许不是颜色字符串。
                // if end + 3 < line.len() && line[end + 3] == 'm' as u8 {
                //     start = end + 4;
                // } else if end + 4 < line.len() && line[end + 4] == 'm' as u8 {
                //     start = end + 5;
                // } else if end + 5 < line.len() && line[end + 5] == 'm' as u8 {
                //     start = end + 6;
                // } else if end + 6 < line.len() && line[end + 6] == 'm' as u8 {
                //     start = end + 7;
                // } else {
                //     println!("WARN: line[end + 3] != 'm' as u8 && line[end + 4] != 'm' as u8");
                //     return line.to_vec();
                // }
                let mut temp_index = end + 3;
                while temp_index < line.len() && temp_index <= end + 6 {
                    if line[temp_index] == 'm' as u8 {
                        start = temp_index + 1;
                        break;
                    }
                    temp_index += 1;
                }
                if temp_index == line.len() || temp_index > end + 6 {
                    println!("WARN: 'm' not found in line[{}..{}]", end + 3, end + 6);
                    return Cow::Owned(line.to_vec());
                }
                if start == line.len() {
                    break;
                }
                if start > line.len() {
                    println!("WARN: start: {} > line.len(): {}", start, line.len());
                    return Cow::Owned(line.to_vec());
                }
            } else {
                buf.extend_from_slice(&line[start..]);
                break;
            }
        }
        return Cow::Owned(buf);
    }
    Cow::Borrowed(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eliminate_color() {
        let line = "\x1b[31m 红色 \x1b[0m";
        println!("origin line: {}", line);
        let line = eliminate_color(line.as_bytes());
        println!("eliminate_color: {}", String::from_utf8_lossy(&line));
        assert_eq!(line, " 红色 ".as_bytes());

        struct TestCase {
            name: &'static str,
            origin: &'static str,
            expected: &'static str,
        }
        let test_cases = vec![
            TestCase {
                name: "test1",
                origin: "\x1b[31m 红色 \x1b[0m",
                expected: " 红色 ",
            },
            TestCase {
                name: "test2",
                origin: "\x1b[31m 红色 \x1b[0m\x1b[31m 红色 \x1b[0m",
                expected: " 红色  红色 ",
            },
            TestCase {
                name: "test3",
                origin: "\x1b[31m 红色 \x1b[0m\x1b[31m 红色 \x1b[0m\x1b[31m 红色 \x1b[0m",
                expected: " 红色  红色  红色 ",
            },
            TestCase {
                name: "test4",
                origin: "你好\x1b[31m 红色 \x1b[0m",
                expected: "你好 红色 ",
            },
            TestCase {
                name: "test5",
                origin: "你好\x1b[2m 不知道啥色 \x1b[0m",
                expected: "你好 不知道啥色 ",
            },
            TestCase {
                name: "test6",
                origin: "你好\x1b[2m 不知道啥色 \x1b[0m 世界！！！",
                expected: "你好 不知道啥色  世界！！！",
            },
            TestCase {
                name: "test7",
                origin: "你好\x1b[101m 不知道啥色 \x1b[0m 世界！！！",
                expected: "你好 不知道啥色  世界！！！",
            },
            TestCase {
                name: "test8",
                origin: "你好\x1b[1001m 不知道啥色 \x1b[0m 世界！！！",
                expected: "你好 不知道啥色  世界！！！",
            },
            TestCase {
                name: "test9",
                origin: "你好 红色，hello world",
                expected: "你好 红色，hello world",
            },
        ];
        for test_case in test_cases {
            let line = eliminate_color(test_case.origin.as_bytes());
            println!(
                "origin: {}, eliminate_color: {}",
                test_case.origin,
                String::from_utf8_lossy(&line)
            );
            assert_eq!(line, test_case.expected.as_bytes(), "{}", test_case.name);
        }
    }
}

pub fn open_url(uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    match std::env::consts::OS {
        "windows" => {
            let mut cmd = std::process::Command::new("cmd")
                .args(&["/c", "start", uri])
                .spawn()?;
            cmd.wait()?;
        }
        "macos" => {
            let mut cmd = std::process::Command::new("open").arg(uri).spawn()?;
            cmd.wait()?;
        }
        "linux" => {
            let mut cmd = std::process::Command::new("xdg-open").arg(uri).spawn()?;
            cmd.wait()?;
        }
        _ => {
            return Err(format!(
                "don't know how to open things on {} platform",
                std::env::consts::OS
            )
            .into());
        }
    }
    Ok(())
}

pub fn inform<T: AsRef<str>>(content: T, title: &str) {
    use notify_rust::Notification;
    let show_len = 80;
    let mut content_runes = content
        .as_ref()
        .char_indices()
        .filter_map(|ic| match ic.1 {
            c if !c.is_control() => Some(ic.1),
            _ => None,
        })
        .collect::<Vec<char>>();
    if content_runes.len() >= show_len {
        content_runes.truncate(show_len);
        content_runes.append(&mut vec!['.'; 3])
    }
    let body = content_runes.into_iter().collect::<String>();
    Notification::new()
        .summary(title)
        .appname(PROGRAM_NAME)
        .body(&body)
        .icon(crate::config::APP_ICON_PATH.get().unwrap())
        .show()
        .map_err(|err| error!("show notification error: {}", err))
        .ok();
}

pub fn has_img_ext(name: &str) -> bool {
    let ext = name.split(".").last().unwrap_or("");
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "ico" => true,
        _ => false,
    }
}

// e.g. zh_CN.UTF-8 => zh_CN
pub fn get_system_lang() -> String {
    let env_keys = vec!["LANG", "LC_ALL", "LC_MESSAGES", "LANGUAGE"];
    let mut lang = String::new();
    for key in env_keys {
        lang = std::env::var(key).unwrap_or_default();
        if !lang.is_empty() {
            break;
        }
    }
    lang
}
