use core::{
	mem::{
		ManuallyDrop, MaybeUninit,
	},
	ptr::NonNull,
};
use cppdvt::{
	VtObject, VTablePtr, VtObjectPtr,
	this_to_self, vtable, virtual_fn, virtual_call,
};

fn main() {
	let mut stack = VecStack::new();
	assert_eq!(stack.len(), 0);

	assert_eq!(stack.pop(), None);
	assert_eq!(stack.len(), 0);

	stack.push(42);
	assert_eq!(stack.len(), 1);

	stack.push(812);
	assert_eq!(stack.len(), 2);

	assert_eq!(stack.pop(), Some(812));
	assert_eq!(stack.len(), 1);

	assert_eq!(stack.pop(), Some(42));
	assert_eq!(stack.len(), 0);

	assert_eq!(stack.pop(), None);
	assert_eq!(stack.pop(), None);
	assert_eq!(stack.len(), 0);
}

vtable! {
	StackVt[T] {
		pub fn push(value: NonNull<T>);
		pub fn pop(value: NonNull<T>) -> bool;
		pub fn length() -> usize;
	}
}

#[repr(C)]
struct VecStack<T> {
	vtable: VTablePtr<StackVt<T>>,
	stack: Vec<T>,
}

impl<T: 'static> Default for VecStack<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: 'static> VecStack<T> {
	pub const fn new() -> Self {
		Self {
			vtable: VTablePtr::from_ref(Self::VTABLE),
			stack: Vec::new(),
		}
	}

	pub const fn as_object(&self) -> &VtObject<StackVt<T>> {
		unsafe { VtObject::from_ptr(VtObjectPtr::from_ref(&self.vtable)) }
	}

	pub const fn as_mut_object(&mut self) -> &mut VtObject<StackVt<T>> {
		unsafe { VtObject::from_ptr_mut(VtObjectPtr::from_mut(&mut self.vtable)) }
	}

	const VTABLE: &StackVt<T> = &StackVt {
		push: Self::push,
		pop: Self::pop,
		length: Self::length,
	};

	virtual_fn! {
		fn push(this: VtObjectPtr<StackVt<T>>, value: NonNull<T>) {
			let this = this_to_self!(mut this);
			this.stack.push(value.read());
		}
	}
	virtual_fn! {
		fn pop(this: VtObjectPtr<StackVt<T>>, value: NonNull<T>) -> bool {
			let this = this_to_self!(mut this);
			if let Some(t) = this.stack.pop() {
				value.write(t);
				true
			} else {
				false
			}
		}
	}
	virtual_fn! {
		fn length(this: VtObjectPtr<StackVt<T>>) -> usize {
			this_to_self!(ref this).stack.len()
		}
	}
}

impl<T: 'static> AsRef<VtObject<StackVt<T>>> for VecStack<T> {
	fn as_ref(&self) -> &VtObject<StackVt<T>> {
		self.as_object()
	}
}
impl<T: 'static> AsMut<VtObject<StackVt<T>>> for VecStack<T> {
	fn as_mut(&mut self) -> &mut VtObject<StackVt<T>> {
		self.as_mut_object()
	}
}

trait Stack<T>: AsRef<VtObject<StackVt<T>>> + AsMut<VtObject<StackVt<T>>> {
	fn push(&mut self, value: T) {
		let value = ManuallyDrop::new(value);
		unsafe { virtual_call!(mut self.as_mut() => push(NonNull::from_ref(&value))) }
	}
	fn pop_into(&mut self, slot: &mut Option<T>) {
		let mut value = MaybeUninit::uninit();
		let is_some = unsafe { virtual_call!(mut self.as_mut() => pop(NonNull::new_unchecked(value.as_mut_ptr()))) };
		if is_some {
			unsafe { *slot = Some(value.assume_init()) }
		}
	}
	fn pop(&mut self) -> Option<T> {
		let mut slot = None;
		self.pop_into(&mut slot);
		slot
	}
	fn len(&self) -> usize {
		unsafe { virtual_call!(self.as_ref() => length()) }
	}
}

impl<T, S: ?Sized> Stack<T> for S
where
	S: AsRef<VtObject<StackVt<T>>> + AsMut<VtObject<StackVt<T>>>,
{}
