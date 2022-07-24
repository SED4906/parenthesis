mod common;

use common::*;
use imgui::*;
use chrono::prelude::*;
use simple_logger::SimpleLogger;
use std::error::Error;
use rfd::FileDialog;
use std::fs;
use std::process::Command;
use std::process::Stdio;
use std::process::Child;
use std::io::Read;

const APP_NAME: &str = "Parenthesis";

struct WindowData {
    text: String,
    path: String,
    pos: [f32; 2],
    size: [f32; 2],
    form: String,
    cmd: Option<Child>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut windows: Vec::<WindowData> = Vec::new();
    SimpleLogger::new().init()?;
    System::new(APP_NAME)?.run((), move |_, ui, _| {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("Window") {
                if MenuItem::new("New editor").build(ui) {
                    windows.push(WindowData {text:"".into(), path:"".into(), size:[300.0,256.0], pos:[60.0,60.0],form:"editor".into(),cmd:None});
                }
                if MenuItem::new("New shell (unimplemented)").build(ui) {
                    windows.push(WindowData {text:"".into(), path:"".into(), size:[300.0,256.0], pos:[60.0,60.0],form:"shell".into(),cmd:/*Some(
                        Command::new("pwsh").stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("could not spawn")
                    )*/None});
                }
                for windowindex in 0..windows.len() {
                    let closename=format!("Close [{}]",windowindex+1);
                    if MenuItem::new(closename).build(ui) {
                        windows.remove(windowindex);
                    }
                }
                menu.end();
            }
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
            let dt = Local::now();
            ui.text(format!(
                "System Time: ({})",
                dt.format("%a, %Y-%m-%d %H:%M:%S")
            ));
            menu_bar.end();
        }
        let mut windownum = 1;
        for window in &mut windows {
            let title=format!("[{}] {} ({})",windownum, window.path, window.form);
            Window::new(title)
                .size(window.size, Condition::FirstUseEver)
                .position(window.pos, Condition::FirstUseEver)
                .menu_bar(true)
                .build(ui, || {
                    if window.form == "editor" {
                        if let Some(menu_bar) = ui.begin_menu_bar() {
                            if let Some(menu) = ui.begin_menu("File") {
                                if MenuItem::new("New").build(ui) {
                                    window.text = "".into();
                                    window.path = "".into();
                                }
                                if MenuItem::new("Open").build(ui) {
                                    let files = FileDialog::new().pick_file();
                                    if let Some(ref pathbuf) = files {
                                        window.text = fs::read_to_string(files.as_ref().unwrap()).unwrap();
                                        window.path = pathbuf.to_str().unwrap().into();
                                    }
                                }
                                if MenuItem::new("Save As").build(ui) {
                                    let res = rfd::FileDialog::new().save_file();
                                    if let Some(pathbuf) = res {
                                        window.path = pathbuf.to_str().unwrap().into();
                                        fs::write(&window.path,&window.text).expect("Failed to write to file");
                                    }
                                }
                                if MenuItem::new("Save").build(ui) {
                                    if !window.path.is_empty() {
                                        fs::write(&window.path,&window.text).expect("Failed to write to file");
                                    } else {
                                        let res = rfd::FileDialog::new().save_file();
                                        if let Some(pathbuf) = res {
                                            window.path = pathbuf.to_str().unwrap().into();
                                            fs::write(&window.path,&window.text).expect("Failed to write to file");
                                        }
                                    }
                                }
                                menu.end();
                            }
                            menu_bar.end();
                        }
                    }
                    if window.form == "shell" {
                        window.text = "Shells are currently unimplemented.\nHelp with this would be appreciated.\n".into();
                        /*let mut buf: [u8;1] = [0;1];
                        let mut handle = window.cmd.as_mut().unwrap().stdout.as_mut().unwrap();
                        match handle.read(&mut buf) {
                            Ok(0) => {},
                            _ => window.text.push_str(&String::from_utf8_lossy(&buf)),
                        }*/
                    }
                    window.size = ui.window_size();
                    window.pos = ui.window_pos();
                    ui.input_text_multiline("", &mut window.text,[window.size[0],window.size[1]-60.0]).build();
                });
            windownum+=1;
        }
    })?;

    Ok(())
}