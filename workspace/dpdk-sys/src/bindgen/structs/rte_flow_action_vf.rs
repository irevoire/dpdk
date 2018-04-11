// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_action_vf
{
	pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
	pub id: u32,
}

impl Default for rte_flow_action_vf
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_action_vf
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_action_vf {{ original : {:?}, reserved : {:?} }}", self.original(), self.reserved())
	}
}

impl rte_flow_action_vf
{
	
	#[inline(always)]
	pub fn original(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_original(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn reserved(&self) -> u32
	{
		unsafe { transmute(self._bitfield_1.get(1usize, 31u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_reserved(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self._bitfield_1.set(1usize, 31u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn new_bitfield_1(original: u32, reserved: u32) -> __BindgenBitfieldUnit<[u8; 4usize], u32>
	{
		let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u32> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let original: u32 = unsafe { transmute(original) };
			original as u64
		});
		__bindgen_bitfield_unit.set(1usize, 31u8, {
			let reserved: u32 = unsafe { transmute(reserved) };
			reserved as u64
		});
		__bindgen_bitfield_unit
	}
}