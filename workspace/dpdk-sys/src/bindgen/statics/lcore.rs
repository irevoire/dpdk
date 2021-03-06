// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}lcore_config"] pub static mut lcore_config: [lcore_config; 128usize];
	#[link_name = "\u{1}per_lcore__cpuset"] pub static mut per_lcore__cpuset: rte_cpuset_t;
	#[link_name = "\u{1}per_lcore__lcore_id"] pub static mut per_lcore__lcore_id: c_uint;
	#[link_name = "\u{1}per_lcore__rte_errno"] pub static mut per_lcore__rte_errno: c_int;
}
