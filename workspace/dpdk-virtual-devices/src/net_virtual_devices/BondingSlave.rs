// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// How to choose a bonding slave.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub enum BondingSlave
{
	/// By PCI device address.
	ByDpdkPciDeviceAddress(IndirectPciDeviceIdentifier),
	
	/// By virtual device name.
	///
	/// This should not be another bonding device.
	ByVirtualDeviceName(NetVirtualDeviceName),
	
	/// By ethernet port identifier.
	ByEthernetPortIdentifier(u16),
}

impl BondingSlave
{
	#[inline(always)]
	pub(crate) fn as_dpdk_string(&self) -> String
	{
		use self::BondingSlave::*;
		
		match *self
		{
			ByDpdkPciDeviceAddress(ref indirect_pci_device_identifier) => indirect_pci_device_identifier.to_pci_device().to_string(),
			
			ByVirtualDeviceName(ref virtual_device_name) =>
			{
				assert!(virtual_device_name.is_not_backed_by_driver_name(NetVirtualDeviceDriverName::Bonding), "A bonding slave can not itself be a bonding device");
				
				virtual_device_name.to_string()
			}

			ByEthernetPortIdentifier(ref ethernet_port_identifier) => format!("{}", ethernet_port_identifier),
		}
	}
}
