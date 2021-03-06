// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_tm_level_capabilities_1_2
{
	pub shaper_private_supported: c_int,
	pub shaper_private_dual_rate_supported: c_int,
	pub shaper_private_rate_min: u64,
	pub shaper_private_rate_max: u64,
	pub shaper_shared_n_max: u32,
	pub cman_wred_packet_mode_supported: c_int,
	pub cman_wred_byte_mode_supported: c_int,
	pub cman_head_drop_supported: c_int,
	pub cman_wred_context_private_supported: c_int,
	pub cman_wred_context_shared_n_max: u32,
	pub stats_mask: u64,
}

impl Default for rte_tm_level_capabilities_1_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_tm_level_capabilities_1_2
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_tm_level_capabilities_1_2 {{  }}")
	}
}
