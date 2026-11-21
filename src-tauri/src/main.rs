
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    utils::config::WebviewUrl,
    webview::{NewWindowResponse, WebviewWindowBuilder},
};

const START_URL: &str = "https://script.google.com/macros/s/AKfycbxHpLGmHxCzLoX1Qq5SGTxUDR5V_dTVVmdGUXAPyHzJkL0BtXLYkCh9NfJ0J5Septg0/exec";

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let start_url = START_URL
                .parse()
                .expect("Invalid Zanobia start URL");

            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(start_url),
            )
            .title("جرد زنوبيا")
            .inner_size(1280.0, 800.0)
            .resizable(true)

            // السماح فقط بخدمة تطبيق زنوبيا
            .on_navigation(|url| {
                url.scheme() == "https"
                    && matches!(
                        url.host_str(),
                        Some(
                            "script.google.com"
                                | "script.googleusercontent.com"
                        )
                    )
            })

            // منع فتح نوافذ وصفحات جديدة
            .on_new_window(|_, _| NewWindowResponse::Deny)

            .build()?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Failed to run Zanobia Inventory");
}
