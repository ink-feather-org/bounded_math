#![feature(const_mut_refs)]
#![feature(core_intrinsics)]
#![feature(const_heap)]
#![feature(const_ptr_write)]
#![feature(const_eval_select)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![feature(const_trait_impl)]
#![feature(new_uninit)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(const_alloc_layout)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(strict_provenance)]
#![feature(inline_const)]
#![warn(fuzzy_provenance_casts)]
#![no_std]

extern crate alloc;

use core::{
  alloc::Layout,
  borrow::{Borrow, BorrowMut},
  cmp::Ordering,
  intrinsics::const_deallocate,
  intrinsics::{const_allocate, const_eval_select},
  marker::{PhantomData, Unsize},
  mem,
  mem::MaybeUninit,
  ops::CoerceUnsized,
  ops::{Deref, DerefMut},
  ptr::slice_from_raw_parts_mut,
};

pub struct ConstBox<T: ?Sized> {
  layout: Layout,
  ptr: *mut T,
  // Here so dropcheck knowns we own a T
  _p: PhantomData<T>,
}
impl<T: ?Sized> ConstBox<T> {
  pub const fn leak_to_rt(self) -> &'static T {
    let ret = unsafe { &*self.ptr };
    mem::forget(self);
    ret
  }
}

impl<T> ConstBox<T> {
  pub const fn new(val: T) -> ConstBox<T> {
    const fn alloc_const<T>(val: T) -> ConstBox<T> {
      let layout = Layout::new::<T>();

      let ptr = unsafe { const_allocate(layout.size(), layout.align()) as *mut T };
      unsafe { ptr.write(val) };
      ConstBox {
        layout,
        ptr,
        _p: PhantomData,
      }
    }
    fn alloc_rt<T>(_: T) -> ConstBox<T> {
      panic!("Const Box can't be allocated at Runtime")
    }
    unsafe { const_eval_select((val,), alloc_const, alloc_rt) }
  }
}

impl<T> From<T> for ConstBox<T> {
  fn from(value: T) -> Self {
    ConstBox::new(value)
  }
}
impl<T: ~const Clone + ?Sized> From<&T> for ConstBox<T> {
  fn from(val: &T) -> Self {
    const fn alloc_const<T: ~const Clone + ?Sized>(val: &T) -> ConstBox<T> {
      let layout = Layout::new::<T>();

      let ptr = unsafe { const_allocate(layout.size(), layout.align()) as *mut T };
      unsafe { ptr.write(val.clone()) };
      ConstBox {
        layout,
        ptr,
        _p: PhantomData,
      }
    }
    fn alloc_rt<T: Clone + ?Sized>(_: &T) -> ConstBox<T> {
      panic!("Const Box can't be allocated at Runtime")
    }
    unsafe { const_eval_select((val,), alloc_const, alloc_rt) }
  }
}

impl<T> ConstBox<[MaybeUninit<T>]> {
  pub const fn new_uninit_slice(len: usize) -> ConstBox<[MaybeUninit<T>]> {
    const fn alloc_const<T>(len: usize) -> ConstBox<[MaybeUninit<T>]> {
      let Ok(layout) = Layout::array::<T>(len) else {
        panic!("Tried to allocate with invalid layout");
      };

      let data = unsafe { const_allocate(layout.size(), layout.align()) as *mut MaybeUninit<T> };

      let slice = slice_from_raw_parts_mut(data, len);

      ConstBox {
        layout,
        ptr: slice,
        _p: PhantomData,
      }
    }
    fn alloc_rt<T>(_: usize) -> ConstBox<[MaybeUninit<T>]> {
      panic!("Const Box can't be allocated at Runtime")
    }
    unsafe { const_eval_select((len,), alloc_const, alloc_rt) }
  }
  pub const fn new_zeroed_slice(len: usize) -> ConstBox<[MaybeUninit<T>]> {
    const fn alloc_const<T>(len: usize) -> ConstBox<[MaybeUninit<T>]> {
      let Ok(layout) = Layout::array::<T>(len) else {
        panic!("Tried to allocate with invalid layout");
      };

      let data = unsafe { const_allocate(layout.size(), layout.align()) as *mut MaybeUninit<T> };

      let slice = slice_from_raw_parts_mut(data, len);

      let mut i = 0;
      while i < len {
        unsafe { (*slice)[i] = MaybeUninit::zeroed() };

        i += 1;
      }

      ConstBox {
        layout,
        ptr: slice,
        _p: PhantomData,
      }
    }
    fn alloc_rt<T>(_: usize) -> ConstBox<[MaybeUninit<T>]> {
      panic!("Const Box can't be allocated at Runtime")
    }
    unsafe { const_eval_select((len,), alloc_const, alloc_rt) }
  }
  pub const unsafe fn assume_init(self) -> ConstBox<[T]> {
    let ret = ConstBox {
      layout: self.layout,
      ptr: self.ptr as *mut [T],
      _p: PhantomData,
    };
    mem::forget(self);
    ret
  }
}

impl<T: ~const Clone> const Clone for ConstBox<T> {
  fn clone(&self) -> Self {
    Self::new(self.as_ref().clone())
  }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<ConstBox<U>> for ConstBox<T> {}

impl<T: ?Sized> const Drop for ConstBox<T> {
  fn drop(&mut self) {
    const fn drop_ct<T: ?Sized>(val: &mut ConstBox<T>) {
      if mem::needs_drop::<T>() {
        panic!("?Sized types can't be dropped in const yet")
      }
      //unsafe { val.ptr.drop_in_place() };
      unsafe { const_deallocate(val.ptr as *mut u8, val.layout.size(), val.layout.align()) }
    }
    fn drop_rt<T: ?Sized>(_: &mut ConstBox<T>) {
      unreachable!()
    }

    unsafe { const_eval_select((self,), drop_ct, drop_rt) };
  }
}

impl<T: ?Sized> const Deref for ConstBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.as_ref()
  }
}

impl<T: ?Sized> const DerefMut for ConstBox<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut()
  }
}

impl<T: ?Sized> const Borrow<T> for ConstBox<T> {
  fn borrow(&self) -> &T {
    self.as_ref()
  }
}

impl<T: ?Sized> const BorrowMut<T> for ConstBox<T> {
  fn borrow_mut(&mut self) -> &mut T {
    self.as_mut()
  }
}

impl<T: ?Sized> const AsRef<T> for ConstBox<T> {
  fn as_ref(&self) -> &T {
    unsafe { &*self.ptr }
  }
}
impl<T: ?Sized> const AsMut<T> for ConstBox<T> {
  fn as_mut(&mut self) -> &mut T {
    const fn as_mut_ct<T: ?Sized>(val: &mut ConstBox<T>) -> &mut T {
      unsafe { &mut *val.ptr }
    }
    fn as_mut_rt<T: ?Sized>(_: &mut ConstBox<T>) -> &mut T {
      unreachable!();
    }
    unsafe { const_eval_select((self,), as_mut_ct, as_mut_rt) }
  }
}

impl<T: Eq + ?Sized> Eq for ConstBox<T> {}

impl<T: ~const PartialEq + ?Sized> const PartialEq for ConstBox<T> {
  fn eq(&self, other: &Self) -> bool {
    unsafe { &*self.ptr }.eq(other)
  }
}

impl<T: ~const Ord + ?Sized> const Ord for ConstBox<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    unsafe { &*self.ptr }.cmp(other)
  }
}

impl<T: ~const PartialOrd + ?Sized> const PartialOrd for ConstBox<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    unsafe { &*self.ptr }.partial_cmp(other)
  }
}
