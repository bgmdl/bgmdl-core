use download_link::DownloadData;

pub struct TaskDownload {
    pub update_func: Box<dyn FnMut(DownloadData) -> () + Send + Sync>,
    pub url: String,
    pub savepath: String,
    pub save_name: String,
    pub tool: String,
}

pub async fn apply(task: &mut TaskDownload) {
    log::info!("start to download {}", task.url);
    // let update_func = &mut task.update_func;
    // let task_tool = TOOLS.read().unwrap().get(&task.tool).unwrap();
    // let TASK_
    // task_tool.progress_update_run(Box::new(move |data| {
    //     update_func(data);
    // }));
    // log::info!("download {} finished", task.url);
}