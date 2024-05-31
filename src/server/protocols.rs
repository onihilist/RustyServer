use colored::Colorize;
use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct protocolData {
    protocol: String,
    sender: String,
    receiver: String,
    data: String
}

pub fn checkProtocol(protocol_data: protocolData) -> String {

    let logs: UtilsData = utils::Logs::initLog(None, "This protocol action doesn't exist".to_string(), None);

    match protocol_data.protocol.as_str() {
        "INIT_CONNECTION"=>return "init_connection".to_string(),
        "REGISTER"=>return "register".to_string(),
        "LOGIN"=>return "connect".to_string(),
        "SEND"   =>return "send".to_string(),
        "RECEIVE"=>return "receive".to_string(),
        _=>return format!("{}{}{}", "[", "ERROR".red(), "] -> This protocol action doesn't exist"),
    }

}