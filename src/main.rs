extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;

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

    println!("interface.is_loopback() : {}", interface.is_loopback());
    println!("interface.is_broadcast() : {}", interface.is_broadcast());
    println!("interface.is_up() : {}", interface.is_up());
    println!("interface.is_multicast() : {}", interface.is_multicast());
    println!("interface.is_point_to_point() : {}", interface.is_point_to_point());

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                // プリアンブル、SFD、FSCがない。なんでだろ。
                println!("ethernet packet : {:x?}", packet);
                println!("ethernet packet mac to : {:x?}", &packet[0..6]);
                println!("ethernet packet mac from : {:x?}", &packet[7..13]);
                println!("ethernet packet type : {:x?}", &packet[14..16]);
                // インスタンスを使う
                let ethernet_packet = EthernetPacket::new(packet);
                println!("ethernet_packet : {:?}", ethernet_packet);
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    }
}
