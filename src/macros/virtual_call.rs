/// Given an invokation of the form `vt_object => field1.field2.func(...)`,
/// invoke the virtual method `func` while traversing `field`s
/// of the [`VtObject`](crate::VtObject) `vt_object`'s VTable
/// with the specified arguments, if any.
#[macro_export]
macro_rules! virtual_call {
	(mut $vt_object:expr => $field:ident$(.$suffix:ident)*($($arg:tt)*)) => {{
		let vt_object = &mut $vt_object;
		let this = $crate::VtObject::as_mut_ptr(vt_object);
		($crate::VtObject::vtable(vt_object).$field$(.$suffix)*)(this, $($arg)*)
	}};
	($vt_object:expr => $field:ident$(.$suffix:ident)*($($arg:tt)*)) => {{
		let vt_object = &$vt_object;
		let this = $crate::VtObject::as_ptr(vt_object);
		($crate::VtObject::vtable(vt_object).$field$(.$suffix)*)(this, $($arg)*)
	}};

	($($whatever:tt)*) => {
		::core::compile_error! {
			"expected invocation of the form `(mut)? <VtObject> => path.to.func(...)`"
		}
	};
}
