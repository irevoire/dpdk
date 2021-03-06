// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate dpdk_bus;
extern crate dpdk_core;
extern crate hashbrown;
extern crate network_ethernet;
extern crate serde;
#[macro_use] extern crate serde_derive;


use ::dpdk_bus::NetworkInterfaceName;
use ::dpdk_bus::pci::IndirectPciDeviceIdentifier;
use ::dpdk_core::*;
use ::hashbrown::HashSet;
use ::network_ethernet::MediaAccessControlAddress;
use ::std::cmp::max;
use ::std::cmp::min;
use ::std::convert::TryFrom;
use ::std::fmt::Debug;
use ::std::num::NonZeroU32;
use ::std::path::PathBuf;


/// Net(work) virtual devices.
pub mod net_virtual_devices;


include!("VirtualDevice.rs");
include!("VirtualDeviceIndex.rs");
include!("VirtualDeviceName.rs");
