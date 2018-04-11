// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_pmd_i40e_pkt_template_conf
{
	pub input: rte_pmd_i40e_pkt_template_input,
	pub action: rte_pmd_i40e_pkt_template_action,
	pub soft_id: u32,
}

impl Default for rte_pmd_i40e_pkt_template_conf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_pmd_i40e_pkt_template_conf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_pmd_i40e_pkt_template_conf {{ input: {:?}, action: {:?} }}", self.input, self.action)
	}
}