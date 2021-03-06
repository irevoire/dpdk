// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


/// Capabilities.
pub mod capabilities;


/// Linux kernel modules.
pub mod linux_kernel_modules;


/// Mounts.
pub mod mounts;


/// Page table.
pub mod page_table;


/// PCI
pub mod pci;


/// Process control.
pub mod process_control;


/// Resource limits.
pub mod resource_limits;


include!("CpuFeatures.rs");
include!("WarningsToSuppress.rs");
include!("LinuxKernelCommandLineParameters.rs");
include!("LinuxKernelCommandLineValidator.rs");
include!("ProcessCommonConfiguration.rs");
include!("ProcessCommonConfigurationExecutionError.rs");
include!("TransparentHugePageDefragmentationChoice.rs");
include!("TransparentHugePageRegularMemoryChoice.rs");
include!("TransparentHugePageSharedMemoryChoice.rs");
