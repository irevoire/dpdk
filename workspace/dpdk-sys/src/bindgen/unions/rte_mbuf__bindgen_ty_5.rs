// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub union rte_mbuf__bindgen_ty_5
{
	pub userdata: *mut c_void,
	pub udata64: u64,
	_bindgen_union_align: u64,
}

impl Default for rte_mbuf__bindgen_ty_5
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_mbuf__bindgen_ty_5
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_mbuf__bindgen_ty_5 {{ union }}")
	}
}
