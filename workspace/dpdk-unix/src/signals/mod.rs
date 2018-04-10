// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::pthread_sigmask;
use ::libc::SIG_SETMASK;
use ::libc::SIGCHLD;
use ::libc::SIGHUP;
use ::libc::SIGTERM;
use ::libc::sigdelset;
use ::libc::sigfillset;
use ::std::collections::HashSet;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;


include!("block_all_signals_on_current_thread.rs");
include!("block_all_signals_on_current_thread_bar.rs");
include!("block_all_signals_on_current_thread_bar_child.rs");
include!("block_all_signals_on_current_thread_bar_hang_up_and_terminate_and_child.rs");
include!("SignalNumber.rs");
