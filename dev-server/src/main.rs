use std::env;
use std::fmt::Error;
use std::net::ToSocketAddrs;
// use std::net::ToSocketAddrs;
use std::path::PathBuf;
use std::process::{ExitCode, Stdio};
use tokio::process::Command;
use framework::dd;
// use std::time::Duration;
// use watchexec::Watchexec;
// use watchexec::action::ActionHandler;
// use watchexec_events::Tag;
// use watchexec_signals::Signal;
// use watchexec_supervisor::command::{Program};
// use watchexec_supervisor::job::start_job;

#[tokio::main]
async fn main() {
    let mut addrs_iter = "node:80".to_socket_addrs().unwrap();
    let vite_ip = addrs_iter.next().unwrap().ip();
    let dev_server_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let app_root = dev_server_root.join("..").canonicalize().unwrap();
    let app_src = dev_server_root.join("../src").canonicalize().unwrap();

    let app_root_clone = app_root.clone();
    let join_handle = tokio::spawn(async move {
        // call cargo run in root dir
        Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--host=0.0.0.0")
            .arg(format!("--vite-url={}:5173", vite_ip))
            .current_dir(app_root_clone)
            .stdout(Stdio::piped())
            .spawn()
            .expect("command failed to start")
            .wait()
            .await
            .unwrap()
    });

    let t = join_handle.await.unwrap();

    // println!("Pretend started cargo run in project root...");
    //
    // let wx = Watchexec::new_async(move |mut action: ActionHandler| {
    //     Box::new(async move {
    //         for event in action.events.iter() {
    //             // Listen for change to .rs files in ../src/
    //
    //             let path_result = event.tags.iter().find(|tag: &&Tag| {
    //                 if let Tag::Path { .. } = tag {
    //                     return true;
    //                 };
    //                 false
    //             });
    //             let kind_result = event.tags.iter().find(|tag: &&Tag| {
    //                 if let Tag::FileEventKind(kind) = tag {
    //                     return match kind {
    //                         EventKind::Create(_)
    //                         | EventKind::Modify(_)
    //                         | EventKind::Remove(_)
    //                         | EventKind::Other => true,
    //                         _ => false,
    //                     };
    //                 };
    //                 false
    //             });
    //             if let Some(path_outer) = path_result
    //                 && let Tag::Path { path, .. } = path_outer
    //                 && let Some(kind_outer) = kind_result
    //                 && let Tag::FileEventKind(kind) = kind_outer
    //             {
    //                 if path.to_str().unwrap().ends_with(".rs") {
    //                     let r#type = match kind {
    //                         EventKind::Create(_) => "Create",
    //                         EventKind::Modify(_) => "Modify",
    //                         EventKind::Remove(_) => "Remove",
    //                         EventKind::Other => "Other",
    //                         _ => unimplemented!("Should not hit."),
    //                     };
    //
    //                     // Should restart app server
    //                     println!("Event occurred: {type}: {path:?}");
    //                     println!("Pretend stopped cargo run in project root...");
    //                     println!("Pretend restarted cargo run in project root...");
    //                 }
    //             }
    //         }
    //
    //         // If Ctrl-C is received, quit.
    //         // Important: do not remove otherwise you will not be able to quit
    //         let stop_signal = action.signals().find(|sig| match sig {
    //             Signal::ForceStop
    //             | Signal::Interrupt
    //             | Signal::Quit
    //             | Signal::Terminate
    //             | Signal::Custom(_) => true,
    //             _ => false,
    //         });
    //         if stop_signal.is_some() {
    //             println!("Gracefully shutting down...");
    //             action.quit_gracefully(stop_signal.unwrap(), Duration::from_millis(250));
    //         }
    //
    //         action
    //     })
    // })
    // .unwrap();
    //
    // wx.config.pathset([app_src]);
    //
    // wx.main().await.unwrap().unwrap();
    //
    // println!("loop ended...");
    // println!("exiting...");
}
