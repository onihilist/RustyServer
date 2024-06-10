use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct databaseData {
    request: String,
    result: String
}

impl databaseData {
    pub(crate) fn decrypt() {
        let log: UtilsData = utils::Logs::initLog(None, format!("Database's response decryption..."), None);
        utils::Logs::info(log);
    }
}