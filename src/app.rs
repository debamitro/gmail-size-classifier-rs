use handlebars::Handlebars;
use rocket::{routes, Config};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio;

use crate::server::{
    appjs, chart_sectionjs, error, headerjs, home, index, login, logo, mainjs, oauth2_callback,
    profile, searchjs, summary, tab_sectionjs, utilsjs,
};

fn init_handlebars() -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("index", include_str!("../templates/index.html.hbs"))
        .expect("Failed to register index template");
    handlebars
        .register_template_string("error", include_str!("../templates/error.html.hbs"))
        .expect("Failed to register error template");
    handlebars
}

pub struct AppState {
    pub status: Arc<Mutex<String>>,
    pub server_handle: Arc<Mutex<Option<rocket::Shutdown>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new("Idle".to_string())),
            server_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&mut self) {
        let server_handle = self.server_handle.clone();
        let status = self.status.clone();
        // Start Rocket in a separate thread
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let handlebars = init_handlebars();
                let figment = Config::figment()
                    .merge(("port", 5000))
                    .merge(("secret_key", include_str!("../rocket_secret_key")));
                let rocket = rocket::custom(figment)
                    .manage(handlebars)
                    .mount(
                        "/",
                        routes![
                            index,
                            summary,
                            oauth2_callback,
                            login,
                            error,
                            home,
                            profile,
                            mainjs,
                            appjs,
                            headerjs,
                            chart_sectionjs,
                            tab_sectionjs,
                            searchjs,
                            utilsjs,
                            logo
                        ],
                    )
                    .ignite();
                match rocket.await {
                    Ok(ignited_rocket) => {
                        let shutdown = ignited_rocket.shutdown();
                        *server_handle.lock().unwrap() = Some(shutdown);
                        let _ = ignited_rocket.launch().await;
                        *status.lock().unwrap() =
                            "Open http://127.0.0.1:5000 in your web browser".to_string();
                    }
                    Err(e) => {
                        println!("Rocket error: {}", e);
                    }
                }
            });
        });
    }

    pub fn stop(&mut self) {
        if let Some(shutdown) = self.server_handle.lock().unwrap().take() {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                shutdown.notify();
            });
        }
        *self.status.lock().unwrap() = "Server stopped".to_string();
    }

    pub fn get_status(&self) -> String {
        self.status.lock().unwrap().clone()
    }

    pub fn is_running(&self) -> bool {
        self.server_handle.lock().unwrap().is_some()
    }
}
