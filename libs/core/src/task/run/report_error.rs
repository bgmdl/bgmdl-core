#[derive(Debug, Clone)]
pub struct ReportError {
    pub taskid: i32,
    pub error: String,
}

pub async fn apply(task: &mut ReportError) {
    log::error!("{}", task.error);
}
