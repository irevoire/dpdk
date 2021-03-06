// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(C)]
pub struct vfio_pci_hot_reset_info
{
	pub argsz: __u32,
	pub flags: __u32,
	pub count: __u32,
	pub devices: IncompleteArrayField<vfio_pci_dependent_device>,
}

impl Default for vfio_pci_hot_reset_info
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for vfio_pci_hot_reset_info
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "vfio_pci_hot_reset_info {{ devices: {:?} }}", self.devices)
	}
}
