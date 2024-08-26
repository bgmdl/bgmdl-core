use std::thread;

#[derive(Debug, Clone)]
pub struct DownloadData {
    pub name: String,
    pub status: String,
    pub progress: f64,
    pub speed: i64,
    pub eta: i64,
}


pub trait DownloadTools {
    // This will run in the open cli.
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    // download by link
    fn download_by_link(&mut self, url: &str, savepath: &str, rename: &str) -> Result<(), Box<dyn std::error::Error>>;
    // such as init.
    fn login(&mut self, username: &str, password: &str, link: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn progress_update_run(&mut self, callback: Box<dyn FnMut(DownloadData) -> () + Send>) -> ();
}

// pub struct TestTools {

// }

// pub struct TestManager {
// }

// impl DownloadManager for TestManager {
//     fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(())
//     }

//     fn progress_update_run(&mut self, mut callback: Box<dyn FnMut(DownloadData) -> () + Send>) -> () {
//         let mut data = DownloadData {
//             status: "Downloading".to_string(),
//             progress: 0.0,
//             speed: 0.0,
//             eta: 0.0,
//         };
//         thread::spawn(move || {
//             for i in 0..100 {
//                 data.progress = i as f32;
//                 data.speed = i as f32;
//                 data.eta = i as f32;
//                 callback(data.clone());
//                 thread::sleep(std::time::Duration::from_secs(1));
//             }
//         });
        
//     }
// }