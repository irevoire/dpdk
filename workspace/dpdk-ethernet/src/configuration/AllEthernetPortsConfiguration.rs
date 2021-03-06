// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// All ethernet ports configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct AllEthernetPortsConfiguration<FRC: FlowRuleConfiguration>
{
	/// Packet buffer pool definitions.
	pub packet_buffer_pool_definitions: HashMap<PacketBufferPoolReference, PacketBufferPoolConfiguration>,
	
	/// Packet buffer pools by NUMA node.
	#[serde(default = "AllEthernetPortsConfiguration::<FRC>::packet_buffer_pools_by_numa_node_default")]
	pub packet_buffer_pools_by_numa_node: [PacketBufferPoolReference; NumaNode::Maximum],
	
	/// Configurations by device name.
	pub ethernet_ports: HashMap<EthernetPortIdentifierReference, EthernetPortConfiguration<FRC>>,
}

impl<FRC: FlowRuleConfiguration> AllEthernetPortsConfiguration<FRC>
{
	/// Configure.
	pub fn configure(&self) -> (HashMap<PacketBufferPoolReference, PacketBufferPool>, HashMap<EthernetPortIdentifier, (EthernetDeviceCapabilities, Box<[ReceiveBurst]>, Box<[TransmitBurst]>, Vec<FRC::ActiveFlowRuleHandle>)>)
	{
		let packet_buffer_pools_to_not_drop = self.configure_packet_buffer_pools();
		
		let mut ethernet_ports = HashMap::with_capacity(self.ethernet_ports.len());
		
		for (device_name, ethernet_port_configuration) in self.ethernet_ports.iter()
		{
			let ethernet_port_identifier = device_name.ethernet_port_identifier();
			ethernet_ports.insert(ethernet_port_identifier, ethernet_port_configuration.configure(ethernet_port_identifier, &self.packet_buffer_pools_by_numa_node, &packet_buffer_pools_to_not_drop));
		}
		
		(packet_buffer_pools_to_not_drop, ethernet_ports)
	}
	
	fn configure_packet_buffer_pools(&self) -> HashMap<PacketBufferPoolReference, PacketBufferPool>
	{
		for packet_buffer_pool_reference in self.packet_buffer_pools_by_numa_node.iter()
		{
			assert!(self.packet_buffer_pool_definitions.contains_key(packet_buffer_pool_reference), "packet_buffer_pools_by_numa_node '{:?}' is missing in packet_buffer_pool_definitions", packet_buffer_pool_reference);
		}
		
		let mut packet_buffer_pools_to_not_drop = HashMap::with_capacity(self.packet_buffer_pool_definitions.len());
		
		for (packet_buffer_pool_reference, packet_buffer_pool_configuration) in self.packet_buffer_pool_definitions.iter()
		{
			packet_buffer_pools_to_not_drop.insert(*packet_buffer_pool_reference, packet_buffer_pool_configuration.configure(packet_buffer_pool_reference).unwrap());
		}
		
		packet_buffer_pools_to_not_drop
	}
	
	#[inline(always)]
	const fn packet_buffer_pools_by_numa_node_default() -> [PacketBufferPoolReference; NumaNode::Maximum]
	{
		[
			PacketBufferPoolReference::new(0),
			PacketBufferPoolReference::new(1),
			PacketBufferPoolReference::new(2),
			PacketBufferPoolReference::new(3),
			PacketBufferPoolReference::new(4),
			PacketBufferPoolReference::new(5),
			PacketBufferPoolReference::new(6),
			PacketBufferPoolReference::new(7),
		]
	}
}
