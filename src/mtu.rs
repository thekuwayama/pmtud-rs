extern crate pnet;

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::time::Duration;

use pnet::packet::icmp;
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Flags;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::packet::Packet;
use pnet::transport;
use pnet::transport::TransportChannelType;

const ICMP_RECEIVE_TIMEOUT: u64 = 2;
const LOOP_LIMIT: usize = 16;
const MAX_ICMP_PAYLOAD_SIZE: usize = 1500;

pub fn discover(target: Ipv4Addr) -> Result<u16, String> {
    let (mut us, mut ur) = transport::transport_channel(
        65535,
        TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp),
    )
    .map_err(|e| e.to_string())?;
    let mut ur = transport::ipv4_packet_iter(&mut ur);

    let mut max: usize = MAX_ICMP_PAYLOAD_SIZE;
    let mut min: usize = 0;
    let mut size: usize = MAX_ICMP_PAYLOAD_SIZE / 2;
    for _ in 0..LOOP_LIMIT {
        let echo_request =
            gen_echo_request(target, size).ok_or("Failed: generate ICMP packet.".to_string())?;
        us.send_to(echo_request, IpAddr::V4(target))
            .map_err(|e| e.to_string())?;

        let ipv4_response = match ur.next_with_timeout(Duration::new(ICMP_RECEIVE_TIMEOUT, 0)) {
            Ok(opt) => match opt {
                Some((r, _)) => r,
                _ => return Err("Failed: receive ICMP packet.".to_string()),
            },
            Err(e) => return Err(e.to_string()),
        };
        let result = match IcmpPacket::new(ipv4_response.payload()) {
            Some(icmp_packet) => {
                let icmp_type = icmp_packet.get_icmp_type();
                match icmp_type {
                    IcmpTypes::EchoReply => true,
                    _ => false,
                }
            }
            _ => false,
        };

        if result && max - 1 == min {
            return Ok((size + IP_HEADER_LENGTH + ICMP_HEADER_LENGTH) as u16);
        } else if result {
            min = size;
        } else {
            max = size;
        }
        size = (max + min) / 2;
    }

    Err("Failed: over loop-limit when detecting.".to_string())
}

const IP_HEADER_LENGTH: usize = 20;
const ICMP_HEADER_LENGTH: usize = 8;

fn gen_echo_request<'a>(dest: Ipv4Addr, size: usize) -> Option<MutableIpv4Packet<'a>> {
    let ip_total_length = size + IP_HEADER_LENGTH + ICMP_HEADER_LENGTH;
    let ipv4_raw_packet = vec![0u8; ip_total_length];
    let mut ipv4_packet = MutableIpv4Packet::owned(ipv4_raw_packet)?;
    // IPv4
    ipv4_packet.set_version(4);
    // Internet header length in 32-bit words
    ipv4_packet.set_header_length((IP_HEADER_LENGTH * 8 / 32) as u8);
    ipv4_packet.set_total_length(ip_total_length as u16);
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_flags(Ipv4Flags::DontFragment);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_destination(dest);

    let icmp_total_length = size + ICMP_HEADER_LENGTH;
    let icmp_raw_packet = vec![0u8; icmp_total_length];
    let mut icmp_packet = MutableEchoRequestPacket::owned(icmp_raw_packet)?;
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_identifier(0);
    icmp_packet.set_sequence_number(size as u16);
    // 0 padded payload
    let checksum = icmp::checksum(&IcmpPacket::new(icmp_packet.packet())?);
    icmp_packet.set_checksum(checksum);

    ipv4_packet.set_payload(icmp_packet.packet());
    Some(ipv4_packet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_echo_request() {
        let expected: [u8; 0 + IP_HEADER_LENGTH + ICMP_HEADER_LENGTH] = [
            0x45, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x40, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x7f, 0x00, 0x00, 0x01, 0x08, 0x00, 0xf7, 0xff, 0x00, 0x00, 0x00, 0x00,
        ];

        let dest: Ipv4Addr = "127.0.0.1".parse().unwrap();
        assert_eq!(gen_echo_request(dest, 0).unwrap().packet(), &expected);
    }
}
