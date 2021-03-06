// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_crypto_sym_op
{
	pub m_src: *mut rte_mbuf,
	pub m_dst: *mut rte_mbuf,
	pub _1: rte_crypto_sym_op_1,
	pub _2: rte_crypto_sym_op_2,
}

impl Default for rte_crypto_sym_op
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_crypto_sym_op
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_crypto_sym_op {{ m_src: {:?}, m_dst: {:?}, _1: {:?}, _2: {:?} }}", self.m_src, self.m_dst, self._1, self._2)
	}
}
