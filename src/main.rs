extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;

fn main() {
    let iface_name = String::from("en1");
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", iface_name));

    datalink::interfaces().into_iter().for_each(|interface| {
        println!("-----------------------------------------");
        println!("interface.name : {}", interface.name);
        println!("interface.is_loopback() : {}", interface.is_loopback());
        println!("interface.is_broadcast() : {}", interface.is_broadcast());
        println!("interface.is_up() : {}", interface.is_up());
        println!("interface.is_multicast() : {}", interface.is_multicast());
        println!("interface.is_point_to_point() : {}", interface.is_point_to_point());
        println!("-----------------------------------------");
    });

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let ethernet_packet = EthernetPacket::new(packet).unwrap();
                handle_ethernet_packet(&ethernet_packet);
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    }
}

fn handle_ethernet_packet(ethernet_packet: &EthernetPacket) {
    println!("ethernet_packet : {:?}", ethernet_packet);
    match ethernet_packet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ipv4_packet = Ipv4Packet::new(ethernet_packet.payload()).unwrap();
            println!("{:?}", ipv4_packet);
            println!("{}", ipv4_packet.get_next_level_protocol());
        }
        _ => println!("みじっそう"),
    };
}
