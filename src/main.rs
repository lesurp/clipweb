 #![warn(
     clippy::all,
     clippy::complexity,
     clippy::perf,
     clippy::correctness
 )]

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::sync::Mutex;

#[derive(Serialize, Clone)]
struct Clipboard {
    content: String,
}
impl Clipboard {
    fn new() -> Clipboard {
        Clipboard {
            content: "This is a shared clipboard! Erase me, Daddy!".to_owned(),
        }
    }
}

#[derive(Serialize)]
enum ClipboardStatus {
    Ok,
    TooLong(u32),
}

struct ManagedClipboard(Mutex<Clipboard>);
impl ManagedClipboard {
    fn new() -> ManagedClipboard {
        ManagedClipboard(Mutex::new(Clipboard::new()))
    }
}

#[get("/")]
fn get_clipboard(clipboard: State<ManagedClipboard>) -> Json<Clipboard> {
    Json(clipboard.0.lock().unwrap().clone())
}

#[post("/", format = "json", data = "<new_clipboard>")]
fn post_clipboard(
    clipboard: State<ManagedClipboard>,
    new_clipboard: Json<String>,
) -> Json<ClipboardStatus> {
    if new_clipboard.0.len() > 1024 {
        Json(ClipboardStatus::TooLong(1024))
    } else {
        clipboard.0.lock().unwrap().content = new_clipboard.0;
        Json(ClipboardStatus::Ok)
    }
}

fn main() {
    rocket::ignite()
        .manage(ManagedClipboard::new())
        .mount("/api", routes![get_clipboard, post_clipboard])
        .mount("/", StaticFiles::from("./html"))
        .launch();
}
