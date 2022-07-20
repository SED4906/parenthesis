mod common;

use common::*;
use imgui::*;
use simple_logger::SimpleLogger;
use std::error::Error;
use rfd::FileDialog;
use std::fs;

const APP_NAME: &str = "parenthesis";

struct WindowData {
    text: String,
    path: String
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut windows: Vec::<WindowData> = Vec::new();
    SimpleLogger::new().init()?;
    System::new(APP_NAME)?.run((), move |_, ui, _| {
        if let Some(menu_bar) = ui.begin_main_menu_bar() {
            if let Some(menu) = ui.begin_menu("Window") {
                if MenuItem::new("New").shortcut("CTRL+N").build(ui) {
                    windows.push(WindowData {text:"".into(), path:"".into()});
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
            menu_bar.end();
        }
        let mut windownum = 1;
        for window in &mut windows {
            let title=format!("[{}] {}",windownum, window.path);
            Window::new(title)
                .size([300.0, 256.0], Condition::Always)
                .menu_bar(true)
                .resizable(false)
                .build(ui, || {
                    if let Some(menu_bar) = ui.begin_menu_bar() {
                        if let Some(menu) = ui.begin_menu("File") {
                            if MenuItem::new("New").build(ui) {
                                window.text = "".into();
                                window.path = "".into();
                            }
                            if MenuItem::new("Open").build(ui) {
                                let files = FileDialog::new().pick_file();
                                if files.is_some() {
                                    window.text = fs::read_to_string(files.as_ref().unwrap()).unwrap();
                                    window.path = files.unwrap().to_str().unwrap().into();
                                }
                            }
                            if MenuItem::new("Save As").build(ui) {
                                let res = rfd::FileDialog::new().save_file();
                                if res.is_some() {
                                    window.path = res.unwrap().to_str().unwrap().into();
                                    fs::write(&window.path,&window.text);
                                }
                            }
                            if MenuItem::new("Save").build(ui) {
                                if window.path.len() > 0 {
                                    fs::write(&window.path,&window.text);
                                } else {
                                    let res = rfd::FileDialog::new().save_file();
                                    if res.is_some() {
                                        window.path = res.unwrap().to_str().unwrap().into();
                                        fs::write(&window.path,&window.text);
                                    }
                                }
                            }
                            menu.end();
                        }
                        menu_bar.end();
                    }
                    ui.input_text_multiline("", &mut window.text,[300.0,200.0]).build();
                });
            windownum+=1;
        }
    })?;

    Ok(())
}