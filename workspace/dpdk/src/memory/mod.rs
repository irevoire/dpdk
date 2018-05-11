// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


/// Memory zones.
pub mod zones;


include!("AlignmentBitMask.rs");
include!("DpdkAllocatedMemory.rs");
include!("FastMemoryCopy.rs");
include!("FastMemoryCopyDestination.rs");
include!("HugePageAllocationStrategy.rs");
include!("KiloBytes.rs");
include!("MegaBytes.rs");
include!("Memory.rs");
include!("MemoryChannels.rs");
include!("MemoryLimits.rs");
include!("MemoryRanks.rs");
include!("PerMyriad.rs");
include!("VirtualAddress.rs");
