use crate::utils::logger::LogData;

#[derive(Debug, Clone)]
pub struct LogServiceData {
    pub log_data: LogData,
}

unsafe impl Send for LogServiceData {}
unsafe impl Sync for LogServiceData {}

#[allow(clippy::await_holding_lock)]
pub async fn apply(data: LogServiceData) {
    let _record = data.log_data;
    let _remove_flag = false;
    let mut remove_list = vec![];
    let poll = crate::LOG_WS_POLL.lock().unwrap();
    for session in poll.iter() {
        let mut _data = session.0.lock().unwrap();
        let _result = _data.text(serde_json::to_string(&_record).unwrap()).await;
        if _result.is_err() {
            remove_list.push(session.1);
        }
    }
    if _remove_flag {
        for remove in remove_list.iter() {
            let mut poll = crate::LOG_WS_POLL.lock().unwrap();
            poll.retain(|x| x.1 != *remove);
        }
    }
}