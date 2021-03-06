// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// This is a Rust (memory) allocator, ie a pluggable `malloc`, to replace the standard Rust memory allocator.
///
/// It is hybrid because it is designed to be used both before and after DPDK is initialized. Memory assigned before DPDK is initialized is ***limited to a fixed size heap of 1Mb***. Memory of size zero (0) does not consume from this heap. For DPDK, alignments are rescaled to be a minimum of cache line size. If and when const generics land (or alloca) it should be possible to enable configuration of a different memory limit than 1Mb.
///
/// Alignments should not exceed 2^31 (this is quite unlikely).
///
/// To use it, add the following code to your `src/main.rs`:-
///
/// ```
/// #![feature(const_fn)]
/// #![feature(global_allocator)]
///
/// #[global_allocator] static ALLOCATOR: HybridGlobalAllocator = HybridGlobalAllocator::new();
/// ```
///
/// Then, when DPDK has been initialized and configured, switch over to it using `ALLOCATOR.dpdk_is_now_configured()`.
///
/// Before exiting, switch back to the original using `ALLOCATOR.dpdk_was_cleaned_up()`.
pub struct HybridGlobalAllocator
{
	dpdk_configured: AtomicBool,
	heap_memory: [u8; HybridGlobalAllocator::MemoryLimitInBytes],
	heap_manager: LockedHeap,
}

macro_rules! dpdk_alignment
{
	($alignment: ident) =>
	{
		{
			const DpdkCacheLineSize: usize = 64;
			
			if $alignment < DpdkCacheLineSize
			{
				DpdkCacheLineSize as u32
			}
			else if unlikely!($alignment > ::std::u32::MAX as usize)
			{
				return Self::out_of_memory()
			}
			else
			{
				$alignment as u32
			}
		}
	}
}

macro_rules! allocate
{
	($self: expr, $layout: ident, $dpdk_allocate_function: path, $zero_heap_allocated_memory: path) =>
	{
		{
			let size = $layout.size();
			
			if unlikely!(size == 0)
			{
				$self.allocate_zero_sized()
			}
			else if likely!($self.dpdk_configured.load(Relaxed))
			{
				let alignment = $layout.align();
				let dpdk_alignment = dpdk_alignment!(alignment);
				let result = $dpdk_allocate_function(null(), size, dpdk_alignment, Self::current_numa_node());
				if unlikely!(result.is_null())
				{
					Self::out_of_memory()
				}
				else
				{
					result as *mut u8
				}
			}
			else
			{
				let layout_size = $layout.size();
				let pointer = $self.heap_memory_allocate($layout);
				$zero_heap_allocated_memory(layout_size, pointer);
				pointer
			}
		}
	}
}

unsafe impl GlobalAlloc for HybridGlobalAllocator
{
	#[inline(always)]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8
	{
		#[inline(always)]
		fn do_not_zero_heap_allocated_memory(_layout_size: usize, _pointer: *mut u8)
		{
		}
		
		allocate!(self, layout, rte_malloc_socket, do_not_zero_heap_allocated_memory)
	}
	
	#[inline(always)]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
	{
		#[inline(always)]
		fn zero_heap_allocated_memory(layout_size: usize, pointer: *mut u8)
		{
			if !pointer.is_null()
			{
				unsafe { (pointer as *mut u8).write_bytes(0, layout_size) }
			}
		}
		
		allocate!(self, layout, rte_zmalloc_socket, zero_heap_allocated_memory)
	}
	
	#[inline(always)]
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
	{
		let alignment = layout.align();
		
		if unlikely!(self.is_zero_size_or_fixed_memory_pointer(ptr, layout))
		{
			let new_layout = Layout::from_size_align_unchecked(new_size, alignment);
			let new_pointer = self.alloc(new_layout);
			if new_pointer.is_null()
			{
				return Self::out_of_memory()
			}
			else
			{
				copy_nonoverlapping(ptr as *const u8, new_pointer as *mut u8, min(layout.size(), new_size));
				self.heap_memory_free(ptr, layout);
				new_pointer
			}
		}
		else
		{
			if unlikely!(new_size == 0)
			{
				self.dpdk_free(ptr);
				self.allocate_zero_sized()
			}
			else
			{
				let dpdk_alignment = dpdk_alignment!(alignment);
				rte_realloc(ptr as *mut _, new_size, dpdk_alignment) as *mut u8
			}
		}
	}
	
	#[inline(always)]
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
	{
		if unlikely!(self.is_zero_size_or_fixed_memory_pointer(ptr, layout))
		{
			if likely!(layout.size() != 0)
			{
				self.heap_memory_free(ptr, layout)
			}
		}
		else
		{
			self.dpdk_free(ptr)
		}
	}
}

impl HybridGlobalAllocator
{
	const MemoryLimitInBytes: usize = 1024 * 1024;
	
	/// Creates a new instance.
	#[inline(always)]
	pub const fn new() -> Self
	{
		Self
		{
			dpdk_configured: AtomicBool::new(false),
			heap_memory: [0; Self::MemoryLimitInBytes],
			heap_manager: LockedHeap::empty(),
		}
	}
	
	/// Set DPDK is now configured.
	///
	/// Use this to switch over to the DPDK allocator.
	#[inline(always)]
	pub fn dpdk_is_now_configured(&self)
	{
		self.dpdk_configured.store(true, SeqCst);
	}
	
	/// Set DPDK was cleaned up.
	///
	/// Use this to switch back from the DPDK allocator.
	#[inline(always)]
	pub fn dpdk_was_cleaned_up(&self)
	{
		self.dpdk_configured.store(false, SeqCst);
	}
	
	#[inline(always)]
	fn heap_memory_allocate(&self, layout: Layout) -> *mut u8
	{
		let mut locked_heap_manager = self.heap_manager.lock();
		if locked_heap_manager.bottom() == 0
		{
			unsafe { locked_heap_manager.init(self.heap_memory.as_ptr() as usize, Self::MemoryLimitInBytes) };
		}
		
		if let Ok(non_null) = locked_heap_manager.allocate_first_fit(layout)
		{
			non_null.as_ptr()
		}
		else
		{
			null_mut()
		}
	}
	
	#[inline(always)]
	unsafe fn heap_memory_free(&self, pointer: *mut u8, layout: Layout)
	{
		debug_assert!(!pointer.is_null(), "Can not deallocate a null pointer from the heap");
		
		let mut lock = self.heap_manager.lock();
		
		lock.deallocate(NonNull::new_unchecked(pointer), layout);
	}
	
	#[inline(always)]
	fn dpdk_free(&self, pointer: *mut u8)
	{
		unsafe { rte_free(pointer as *mut _) }
	}
	
	#[inline(always)]
	fn is_zero_size_or_fixed_memory_pointer(&self, ptr: *mut u8, layout: Layout) -> bool
	{
		layout.size() == 0 || self.is_fixed_memory_pointer(ptr)
	}
	
	#[inline(always)]
	fn is_fixed_memory_pointer(&self, ptr: *mut u8) -> bool
	{
		let pointer = ptr as usize;
		let fixed_memory_inclusive_start = self.heap_memory.as_ptr() as usize;
		let fixed_memory_exclusive_end = fixed_memory_inclusive_start + Self::MemoryLimitInBytes;
		pointer >= fixed_memory_inclusive_start && pointer < fixed_memory_exclusive_end
	}
	
	#[inline(always)]
	fn current_numa_node() -> i32
	{
		let numa_node = NumaNode::numa_node_and_hyper_thread().0;
		numa_node.into()
	}
	
	#[inline(always)]
	const fn out_of_memory() -> *mut u8
	{
		0 as _
	}
	
	#[inline(always)]
	fn allocate_zero_sized(&self) -> *mut u8
	{
		self.heap_memory.as_ptr() as *mut u8
	}
}
