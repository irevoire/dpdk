// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_flow_attr
{
	pub group: u32,
	pub priority: u32,
	pub bitfield_1: BindgenBitfieldUnit<[u8; 4usize], u32>,
}

impl Default for rte_flow_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_flow_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rte_flow_attr {{ ingress : {:?}, egress : {:?}, transfer : {:?}, reserved : {:?} }}", self.ingress(), self.egress(), self.transfer(), self.reserved())
	}
}

impl rte_flow_attr
{
	
	#[inline(always)]
	pub fn ingress(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(0usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_ingress(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(0usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn egress(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(1usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_egress(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(1usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn transfer(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(2usize, 1u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_transfer(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(2usize, 1u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn reserved(&self) -> u32
	{
		unsafe { transmute(self.bitfield_1.get(3usize, 29u8) as u32) }
	}
	
	#[inline(always)]
	pub fn set_reserved(&mut self, val: u32)
	{
		unsafe {
			let val: u32 = transmute(val);
			self.bitfield_1.set(3usize, 29u8, val as u64)
		}
	}
	
	#[inline(always)]
	pub fn newbitfield_1(ingress: u32, egress: u32, transfer: u32, reserved: u32) -> BindgenBitfieldUnit<[u8; 4usize], u32>
	{
		let mut __bindgen_bitfield_unit: BindgenBitfieldUnit<[u8; 4usize], u32> = Default::default();
		__bindgen_bitfield_unit.set(0usize, 1u8, {
			let ingress: u32 = unsafe { transmute(ingress) };
			ingress as u64
		});
		__bindgen_bitfield_unit.set(1usize, 1u8, {
			let egress: u32 = unsafe { transmute(egress) };
			egress as u64
		});
		__bindgen_bitfield_unit.set(2usize, 1u8, {
			let transfer: u32 = unsafe { transmute(transfer) };
			transfer as u64
		});
		__bindgen_bitfield_unit.set(3usize, 29u8, {
			let reserved: u32 = unsafe { transmute(reserved) };
			reserved as u64
		});
		__bindgen_bitfield_unit
	}
}
