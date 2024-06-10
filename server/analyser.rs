use crate::server::protocols::protocolData;
use crate::database::database;

pub struct analyserData {
    packet : protocolData,
    is_valid: bool,
    response: String
}

pub fn analysePacket(packet: protocolData) {

    if packet.protocol == "INIT_CONNEXION" {

    }

}