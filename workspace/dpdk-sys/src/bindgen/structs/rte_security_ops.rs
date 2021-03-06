// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Hash)]
pub struct rte_security_ops
{
	pub session_create: security_session_create_t,
	pub session_update: security_session_update_t,
	pub session_get_size: security_session_get_size,
	pub session_stats_get: security_session_stats_get_t,
	pub session_destroy: security_session_destroy_t,
	pub set_pkt_metadata: security_set_pkt_metadata_t,
	pub get_userdata: security_get_userdata_t,
	pub capabilities_get: security_capabilities_get_t,
}

impl Default for rte_security_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_security_ops
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_security_ops {{  }}")
	}
}
