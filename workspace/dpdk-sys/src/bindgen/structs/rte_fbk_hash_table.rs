// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct rte_fbk_hash_table
{
	pub name: [c_char; 32usize],
	pub entries: u32,
	pub entries_per_bucket: u32,
	pub used_entries: u32,
	pub bucket_mask: u32,
	pub bucket_shift: u32,
	pub hash_func: rte_fbk_hash_fn,
	pub init_val: u32,
	pub t: IncompleteArrayField<rte_fbk_hash_entry>,
}

impl Default for rte_fbk_hash_table
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rte_fbk_hash_table
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(
			f,
			"rte_fbk_hash_table {{ name: [{}], t: {:?} }}",
			self.name
				.iter()
				.enumerate()
				.map(|(i, v)| format!(
					"{}{:?}",
					if i > 0
					{
						", "
					}
					else
					{
						""
					},
					v
				))
				.collect::<String>(),
			self.t
		)
	}
}
