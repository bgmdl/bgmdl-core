#[derive(Debug, Clone)]
pub struct ChangeName {
    pub taskid: i32,
    pub path: String,
    pub name: String,
}

pub async fn apply(_task: &mut ChangeName) {}
