// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_eth_vmdq_dcb_tx_conf
{
	pub nb_queue_pools: rte_eth_nb_pools,
	pub dcb_tc: [u8; 8usize],
}

impl Default for rte_eth_vmdq_dcb_tx_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_eth_vmdq_dcb_tx_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_eth_vmdq_dcb_tx_conf {{ nb_queue_pools: {:?}, dcb_tc: {:?} }}", self.nb_queue_pools, self.dcb_tc)
	}
}
