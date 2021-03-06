// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2019 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;


#[cfg(any(target_os = "android", target_os = "linux"))] include!("CpuSet.rs");
include!("Nice.rs");
include!("ProcessNiceness.rs");
include!("ProcessNicenessAdjustmentError.rs");
include!("RealTimeSchedulerPriority.rs");
#[cfg(any(target_os = "android", target_os = "linux"))] include!("Scheduler.rs");
#[cfg(any(target_os = "android", target_os = "linux"))] include!("sched_attr.rs");
#[cfg(any(target_os = "android", target_os = "linux"))] include!("sched_setattr.rs");
