// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// DPDK process type of process.
///
/// Defaults to `Auto`.
///
/// Obtain from `process_type()`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProcessType
{
	/// Automatically determined.
	Auto,
	
	/// Primary process in a multi-process set-up.
	Primary,
	
	/// Secondary process in a multi-process set-up.
	Secondary,
}

impl Default for ProcessType
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcessType::Auto
	}
}

impl ProcessType
{
	#[inline(always)]
	pub(crate) fn as_initialization_argument(self) -> ConstCStr
	{
		use self::ProcessType::*;
		
		match self
		{
			Auto => const_cstr!("auto"),
			Primary => const_cstr!("primary"),
			Secondary => const_cstr!("secondary"),
		}
	}
	
	/// Is the primary process alive?
	///
	/// Only valid after `DpdkConfiguration.initialize_dpdk()` called.
	#[inline(always)]
	pub fn is_primary_dpdk_process_alive() -> bool
	{
		if let Some(primary_process_configuration_file_path) = primary_process_configuration_file_path
		{
			let c_string = primary_process_configuration_file_path.to_c_string();
			
			isTrue(unsafe { rte_eal_primary_proc_alive(c_string.as_ptr()) })
		}
		else
		{
			isTrue(unsafe { rte_eal_primary_proc_alive(null()) })
		}
	}
	
	/// Process type of current process.
	///
	/// Only valid after `DpdkConfiguration.initialize_dpdk()` called.
	#[inline(always)]
	pub fn process_type() -> Result<ProcessType, ()>
	{
		use self::rte_proc_type_t::*;
		use self::ProcessType::*;
		
		match unsafe { rte_eal_process_type() }
		{
			RTE_PROC_AUTO => Ok(Auto),
			RTE_PROC_PRIMARY => Ok(Primary),
			RTE_PROC_SECONDARY => Ok(Secondary),
			RTE_PROC_INVALID => Err(()),
		}
	}
}
