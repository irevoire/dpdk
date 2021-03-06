// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_item_arp_eth_ipv4
{
	pub hrd: rte_be16_t,
	pub pro: rte_be16_t,
	pub hln: u8,
	pub pln: u8,
	pub op: rte_be16_t,
	pub sha: ether_addr,
	pub spa: rte_be32_t,
	pub tha: ether_addr,
	pub tpa: rte_be32_t,
}

impl Default for rte_flow_item_arp_eth_ipv4
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_item_arp_eth_ipv4
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_item_arp_eth_ipv4 {{ sha: {:?}, tha: {:?} }}", self.sha, self.tha)
	}
}
