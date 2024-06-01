use crate::server::protocols::{
    protocolData,
    initProtocolData,
    checkProtocol
};
use chrono::{
    DateTime,
    Utc
};
use std::time::{Instant};


pub struct responseData{
    protocolData: protocolData,
    timeUTC: DateTime<Utc>,
    responseTime: u16
}

impl responseData {
    pub(crate) fn to_byte_slices(&self) -> (&[u8], &[u8], &[u8], &[u8]) {
        (
            self.protocolData.protocol.as_bytes(),
            self.protocolData.sender.as_bytes(),
            self.protocolData.receiver.as_bytes(),
            self.protocolData.data.as_bytes(),
        )
    }
}

pub fn initResponseData(protocol_data: protocolData) -> responseData {

    let t0: Instant = Instant::now();
    let protocol: protocolData= checkProtocol(protocol_data);

    // CHECK TO-DO 1
    if protocol.protocol == "INIT_CONNECTION" {

        let response = responseData {
            protocolData: protocol,
            timeUTC: Utc::now(),
            responseTime: t0.elapsed().as_millis() as u16
        };

        return response;

    } else {

        let payload: protocolData = initProtocolData(
            "PROTOCOL_NOT_EXIST".to_string(),
            "server".to_string(),
            "receiver".to_string(),
            "This protocol doesn't exist".to_string()
        );

        let response = responseData {
            protocolData: payload,
            timeUTC: Utc::now(),
            responseTime: t0.elapsed().as_millis() as u16
        };

        return response;

    }
}