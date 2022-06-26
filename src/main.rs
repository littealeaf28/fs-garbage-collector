use std::thread;

use web_view::*;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

use redis::Commands;

fn main() {
    thread::spawn(|| {
        let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

        web_view::builder()
            .title("FS Garbage Collector")
            .content(Content::Html(html_content))
            // .size(320, 480)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|_webview, _arg| Ok(()))
            .run()
            .unwrap();

        // Webview should run and pull read file
        // Should have option to delete
    });

    // Spawn another thread that listens for specific keyboard shortcut to pull up again

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // lpush
    // ldelete

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    let check_dirs: Vec<String> = con.lrange("check_dirs", 0, -1).unwrap();
    for dir in check_dirs.iter() {
        watcher.watch(dir, RecursiveMode::Recursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(path)) => {
                println!("{}", path.into_os_string().into_string().unwrap())
                // Add to tree
                // TODO: Figure out how to use redis.json
            },
            // Ok(DebouncedEvent::R) - Delete
            Ok(_e) => println!("Other event"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
