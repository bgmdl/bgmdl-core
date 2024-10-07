
#[derive(Clone)]
pub struct ReportError {
    error: String,
}

pub async fn apply(task: &mut ReportError) {
    log::error!("{}", task.error);
}