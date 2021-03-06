// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
#[repr(align(64))]
pub struct rte_event_timer_adapter
{
	pub arm_burst: rte_event_timer_arm_burst_t,
	pub arm_tmo_tick_burst: rte_event_timer_arm_tmo_tick_burst_t,
	pub cancel_burst: rte_event_timer_cancel_burst_t,
	pub data: *mut rte_event_timer_adapter_data,
	pub ops: *const rte_event_timer_adapter_ops,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 1usize], u8>,
	pub __bindgen_padding_0: [u8; 23usize],
	pub __bindgen_align: [u8; 0usize],
}

impl Default for rte_event_timer_adapter
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_event_timer_adapter
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_event_timer_adapter {{ data: {:?}, ops: {:?}, allocated : {:?} }}", self.data, self.ops, self.allocated())
	}
}

impl rte_event_timer_adapter
{
	
	#[inline(always)]
	pub fn allocated(&self) -> u8
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u8) }
	}
	
	#[inline(always)]
	pub fn set_allocated(&mut self, val: u8)
	{
		unsafe {
			let val: u8 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(allocated: u8) -> BindgenBitfieldUnit<[u8; 1usize], u8>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 1usize], u8> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let allocated: u8 = unsafe { transmute(allocated) };
			allocated as u64
		});
		__bindgen_bitfield_unit
	}
}
