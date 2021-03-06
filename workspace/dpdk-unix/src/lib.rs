// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(renamed_and_removed_lints)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![feature(core_intrinsics)]


//! #dpdk-unix
//!
//! This crate proves additional mid-level functionality for Unix-like Operating Systems which wraps functionality found in low-level FFI bindings for libc.
//!
//! It also provides a very small modicum of Windows support to get the current program name.


#[cfg(any(target_os = "android", target_os = "linux"))] #[macro_use] extern crate bitflags;
extern crate errno;
extern crate hashbrown;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate likely;
extern crate raw_cpuid;
#[macro_use] extern crate serde_derive;
extern crate rust_extra;
#[cfg(unix)] extern crate syscall_alt;


#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::capabilities::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::mounts::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::page_table::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::process_control::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::resource_limits::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::android_linux::linux_kernel_modules::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::daemonize::*;
use ::hashbrown::HashMap;
use ::hashbrown::HashSet;
use self::hyper_thread::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::logging::*;
use self::memory_information::*;
use self::numa::*;
use self::process_status::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::scheduling::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use self::signals::*;
use self::strings::*;
use ::errno::errno;
use ::libc::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::c_void;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::FILE;
use ::libc::gid_t;
use ::libc::mode_t;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::mount;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::SCHED_BATCH;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::SCHED_FIFO;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::SCHED_IDLE;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::SCHED_OTHER;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::SCHED_RR;
use ::libc::uid_t;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc::umount2;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::linux::capability;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::linux::ethtool::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::linux::seccomp::SECCOMP_MODE_STRICT;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::linux::securebits::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::linux::sockios::SIOCETHTOOL;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::mntent::setmntent;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::mntent::getmntent;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::mntent::endmntent;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::mntent::mntent;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::net::if_::*;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::stdio::cookie_io_functions_t;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::stdio::cookie_write_function_t;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::stdio::fopencookie;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::sys::prctl::PR_CAPBSET_DROP;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::sys::prctl::PR_CAPBSET_READ;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::sys::prctl::PR_CAP_AMBIENT;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::libc_extra::android_linux::sys::prctl::PR_CAP_AMBIENT_CLEAR_ALL;
#[cfg(target_os = "linux")] use ::libc_extra::linux::errno::program_invocation_short_name;
#[cfg(any(target_os = "android", target_os = "linux"))]  use ::libc_extra::unix::stdio::stderr;
#[cfg(any(target_os = "android", target_os = "linux"))]  use ::libc_extra::unix::stdio::stdout;
#[cfg(unix)] use ::libc_extra::unix::unistd::setegid;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::raw_cpuid::*;
use ::std::any::Any;
use ::std::collections::BTreeSet;
use ::std::env::set_var;
use ::std::env::var_os;
use ::std::error;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::ffi::NulError;
use ::std::ffi::OsStr;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::ffi::OsString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::fs::create_dir_all;
use ::std::fs::File;
use ::std::fs::OpenOptions;
use ::std::fs::metadata;
use ::std::fs::Permissions;
use ::std::fs::read_to_string;
use ::std::fs::remove_file;
use ::std::fs::set_permissions;
use ::std::io;
use ::std::io::BufRead;
use ::std::io::BufReader;
use ::std::io::ErrorKind;
#[allow(unused_imports)] use ::std::io::Read;
#[allow(unused_imports)] use ::std::io::Seek;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::io::SeekFrom;
use ::std::io::Write;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::mem::size_of;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::mem::transmute;
use ::std::mem::uninitialized;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::mem::zeroed;
use ::std::num::ParseIntError;
use ::std::ops::Add;
use ::std::ops::AddAssign;
use ::std::ops::Sub;
use ::std::ops::SubAssign;
#[cfg(any(target_os = "android", target_os = "linux"))]  use ::std::os::unix::io::RawFd;
#[cfg(unix)] use ::std::os::unix::io::AsRawFd;
#[cfg(unix)] use ::std::os::unix::ffi::OsStrExt;
#[cfg(unix)] #[allow(unused_imports)] use ::std::os::unix::ffi::OsStringExt;
#[cfg(unix)] use ::std::os::unix::fs::PermissionsExt;
use ::std::panic::*;
use ::std::process;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::process::Command;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::process::Stdio;
use ::std::path::Path;
use ::std::path::PathBuf;
use ::std::ptr::NonNull;
use ::std::ptr::null_mut;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::std::ptr::write;
use ::std::str::from_utf8;
use ::std::str::FromStr;
use ::std::str::Utf8Error;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::syscall_alt::constants::E;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::syscall_alt::constants::SYS::SYS_finit_module;
#[cfg(any(target_os = "android", target_os = "linux"))] use ::syscall_alt::constants::SYS::SYS_delete_module;


#[cfg(any(target_os = "android", target_os = "linux"))]
/// Functionality to provide mid-level wrappers on Linux and Android.
pub mod android_linux;


/// Daemonization support.
pub mod daemonize;


/// HyperThread support.
pub mod hyper_thread;


/// Logging support.
pub mod logging;


/// Memory Information.
pub mod memory_information;


/// NUMA (non-uniform memory architecture) information.
pub mod numa;


/// Scheduling.
pub mod scheduling;


/// Process status.
pub mod process_status;


/// Support for signals.
#[cfg(unix)] pub mod signals;


/// String utilties.
pub mod strings;


/// Thread support.
pub mod thread;


include!("assert_effective_user_id_is_root.rs");
include!("DevPath.rs");
include!("DisableTransparentHugePagesError.rs");
include!("get_program_name.rs");
include!("ListParseError.rs");
include!("PathExt.rs");
include!("ProcPath.rs");
include!("SysPath.rs");
