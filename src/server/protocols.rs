use colored::Colorize;
use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct protocolData {
    pub protocol: String,
    pub sender: String,
    pub receiver: String,
    pub data: String
}

impl protocolData {
    pub(crate) fn to_byte_slices(&self) -> (&[u8], &[u8], &[u8], &[u8]) {
        (
            self.protocol.as_bytes(),
            self.sender.as_bytes(),
            self.receiver.as_bytes(),
            self.data.as_bytes(),
        )
    }
}

pub fn initProtocolData(protocol: String, sender: String, receiver: String, data: String) -> protocolData {
    let data = protocolData {
        protocol,
        sender,
        receiver,
        data
    };
    return data;
}

pub fn checkProtocol(protocol_data: protocolData) -> protocolData {

    let logs: UtilsData = utils::Logs::initLog(None,
                                               format!("Packet from {} -> {}{}{}{}{}{}{}",
                                                       protocol_data.sender,
                                                       protocol_data.protocol,
                                                       "::",
                                                       protocol_data.sender,
                                                       "::",
                                                       protocol_data.receiver,
                                                       "::",
                                                       protocol_data.data
                                               ),
                                               None);
    let errLog: UtilsData = utils::Logs::initLog(None, "Unknown protocol".to_string(), None);

    match protocol_data.protocol.as_str() {
        "INIT_CONNECTION"=>{
            utils::Logs::debug(logs);
            return protocol_data
        },
        "REGISTER"=>{
            utils::Logs::debug(logs);
            return protocol_data
        },
        "LOGIN"=>{
            utils::Logs::debug(logs);
            return protocol_data
        },
        "SEND"   =>{
            utils::Logs::debug(logs);
            return protocol_data
        },
        "RECEIVE"=>{
            utils::Logs::debug(logs);
            return protocol_data
        },
        _=>{
            utils::Logs::warning(errLog);
            return initProtocolData(
                "PROTOCOL_NOT_EXIST".to_string(),
                "server".to_string(),
                "receiver".to_string(),
                "This protocol doesn't exist".to_string()
            )
        },
    }
}

pub fn createProtocol(request: String) -> protocolData {
    let parts: Vec<&str> = request.split("::").collect();

    let newProtocol: protocolData = protocolData{
        protocol: parts[0].to_string(),
        sender:   parts[1].to_string(),
        receiver: parts[2].to_string(),
        data:     parts[3].to_string(),
    };
    return newProtocol;
}

pub fn protocolParser(packet: protocolData) -> String {

    let splitter: String = "::".to_string();
    let request = format!("{}{}{}{}{}{}{}",
                          packet.protocol,
                          splitter,
                          packet.sender,
                          splitter,
                          packet.receiver,
                          splitter,
                          packet.data
    );
    return request;
}

pub fn concatenate_slices<'a>(separator: &[u8], slice1: &'a [u8], slice2: &'a [u8], slice3: &'a [u8], slice4: &'a [u8]) -> &'a [u8] {
    let total_length = slice1.len() + separator.len() + slice2.len()+ separator.len() + slice3.len()+ separator.len() + slice4.len();
    let mut result = Vec::with_capacity(total_length);
    result.extend_from_slice(slice1);
    result.extend_from_slice(separator);
    result.extend_from_slice(slice2);
    result.extend_from_slice(separator);
    result.extend_from_slice(slice3);
    result.extend_from_slice(separator);
    result.extend_from_slice(slice4);

    unsafe { std::slice::from_raw_parts(result.as_ptr(), total_length) }

}