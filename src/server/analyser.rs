use crate::server::protocols::{protocolData, PROTOCOL_DATA_SEP, INIT_CONNECTION, SEND, RECEIVE, REGISTER, LOGIN, initProtocolData};
use crate::database::database;
use crate::server::response;
use crate::server::response::{
    responseData,
    initResponseData
};
use crate::utils;
use crate::utils::Logs::UtilsData;

pub struct analyserData {
    packet : protocolData,
    is_valid: bool,
    response: String
}

pub fn analysePacket(packet: protocolData)-> responseData {

    let logs: UtilsData = utils::Logs::initLog(
        None,
        format!("Packet from {} -> {}{}{}{}{}{}{}",
                packet.sender,
                packet.protocol, PROTOCOL_DATA_SEP,
                packet.sender, PROTOCOL_DATA_SEP,
                packet.receiver, PROTOCOL_DATA_SEP,
                packet.data),
        None);

    return match packet.protocol.as_str() {
        INIT_CONNECTION => {
            utils::Logs::debug(logs);
            initResponseData(
                INIT_CONNECTION.to_string(),
                "server".to_string(),
                packet.sender,
                "CONNECTION OK.".to_string()
            ) //
        },
        REGISTER => {
            utils::Logs::debug(logs);
            initResponseData(
                REGISTER.to_string(),
                "server".to_string(),
                packet.sender,
                "REGISTER OK.".to_string()
            )
        },
        LOGIN => {
            utils::Logs::debug(logs);
            initResponseData(
                LOGIN.to_string(),
                "server".to_string(),
                packet.sender,
                "LOGIN OK.".to_string()
            )
        },
        SEND => {
            utils::Logs::debug(logs);
            initResponseData(
                SEND.to_string(),
                "server".to_string(),
                packet.sender,
                "SEND OK.".to_string()
            )
        },
        RECEIVE => {
            utils::Logs::debug(logs);
            initResponseData(
                RECEIVE.to_string(),
                "server".to_string(),
                packet.sender,
                "RECEIVE OK.".to_string()
            )
        },
        _ => {
            utils::Logs::warning(utils::Logs::initLog(None, "Unknown protocol".to_string(), None));
            initResponseData(
                "PROTOCOL_NOT_EXIST".to_string(),
                "server".to_string(),
                "receiver".to_string(),
                "This protocol doesn't exist".to_string()
            )
        },
    }

}