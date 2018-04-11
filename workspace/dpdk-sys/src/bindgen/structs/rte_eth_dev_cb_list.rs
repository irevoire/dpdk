// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct rte_eth_dev_cb_list
{
	pub tqh_first: *mut rte_eth_dev_callback,
	pub tqh_last: *mut *mut rte_eth_dev_callback,
}

impl Default for rte_eth_dev_cb_list
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
