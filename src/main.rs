use std::thread;

use web_view::*;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

        web_view::builder()
            .title("FS Garbage Collector")
            .content(Content::Html(html_content))
            .size(320, 480)
            .resizable(false)
            .debug(true)
            .user_data(())
            .invoke_handler(|_webview, _arg| Ok(()))
            .run()
            .unwrap();
    });

        // Webview should run and pull read file
        // Should have option to delete

    // Spawn another thread that listens for specific keyboard shortcut to pull up again

    // Need config to know what listening for
    // Loop with listening for changes

    // Scaling to multiple checked directories
    let (tx, rx) = channel();
    
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // TODO: How to read from .env file?
    watcher.watch(check_dir, RecursiveMode::Recursive).unwrap();
    watcher.watch(check_dir_two, RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(path)) => println!("{}", path.display()),
            Ok(_e) => println!("Other event"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
