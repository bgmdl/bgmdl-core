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
    // when updated
    fn progress_update_run(&mut self, callback: Box<dyn FnMut(DownloadData) -> () + Send>) -> ();
}