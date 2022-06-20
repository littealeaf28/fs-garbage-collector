// use web_view::*;

extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

fn main() {
    // let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/10 * * * * *".parse().unwrap(), || {
        println!("Hello!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }

    // web_view::builder()
    //     .title("My Project")
    //     .content(Content::Html(html_content))
    //     .size(320, 480)
    //     .resizable(false)
    //     .debug(true)
    //     .user_data(())
    //     .invoke_handler(|_webview, _arg| Ok(()))
    //     .run()
    //     .unwrap();
}
