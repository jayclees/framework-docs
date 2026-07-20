use notify::{recommended_watcher, Error, Event, RecursiveMode, Watcher};
use notify_types::event::EventKind;
use std::env;
use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::mpsc;
use tokio::process::{Child, Command};
use tokio::task::JoinHandle;

// struct DropGuard(Child);
//
// impl Drop for DropGuard {
//     async fn drop(&mut self) {
//         let mut child = &mut self.0;
//         match child.kill().await {
//             Ok(_) => {}
//             Err(_) => {}
//         };
//     }
// }

#[tokio::main]
async fn main() {
    let mut addrs_iter = "node:5173".to_socket_addrs().unwrap();
    let vite_url = addrs_iter.next().unwrap().to_string();
    let dev_server_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let app_root = dev_server_root.join("..").canonicalize().unwrap();
    let app_src = dev_server_root.join("../src").canonicalize().unwrap();
    let mut join_handle = cargo_run(vite_url.clone(), app_root.clone());
    let mut child = join_handle.await.unwrap();

    // let mut started =
    let (tx, rx) = mpsc::channel::<Result<Event, Error>>();
    let mut watcher = recommended_watcher(tx).unwrap();

    // start process
    // start loop & wait for fs event
    // when event received and is updating an rs file
    // kill original process and start a new one

    watcher
        .watch(Path::new("../src"), RecursiveMode::Recursive)
        .unwrap();
    // loop {
    //     let event = listen_for_rs_update().await;
    //     join_handle.await.unwrap().kill().await.unwrap();
    //     join_handle = cargo_run(vite_url.clone(), app_root.clone());
    // }
    for res in rx {
        match res {
            Ok(event) => match event.kind {
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
                    dbg!("Modify event received");
                    Command::new("kill")
                        .args(["-s", "TERM", &child.id().unwrap().to_string()])
                        .spawn()
                        .unwrap()
                        .wait()
                        .await
                        .unwrap();
                    dbg!("kill command sent");
                    let t = child.wait().await.unwrap();
                    dbg!("Child exit status:");
                    dbg!(t);
                    dbg!("End child exit status:");
                    join_handle = cargo_run(vite_url.clone(), app_root.clone());
                    child = join_handle.await.unwrap();

                    let path_or_error = match PathBuf::from(&event.paths[0]).canonicalize() {
                        Ok(pathbuf) => pathbuf,
                        Err(err) => event.paths[0].clone()
                    };
                    dbg!(path_or_error);
                }
                _ => {
                    dbg!("Other event");
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    // let t2 = t.kill().await;

    // let t = join_handle.await.unwrap();

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
    // println!("closing...");
    // join_handle.await.unwrap().kill().await.unwrap();
}

fn cargo_run(vite_url: String, working_dir: PathBuf) -> JoinHandle<Child> {
    tokio::spawn(async move {
        Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--host=0.0.0.0")
            .arg(format!("--vite-url={vite_url}"))
            .current_dir(working_dir)
            .stdout(Stdio::piped())
            .spawn()
            .expect("command failed to start")
    })
}
