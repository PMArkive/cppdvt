/// Given an invokation of the form `vt_object => field1.field2.func(...)`,
/// invoke the virtual method `func` while traversing `field`s
/// of the [`VtObject`](crate::VtObject) `vt_object`'s VTable
/// with the specified arguments, if any.
#[macro_export]
macro_rules! virtual_call {
	($vt_object:expr => $name:ident$(.$name1:ident)*($($arg:tt)*)) => {{
		let vt_object = &$vt_object;
		($crate::VtObject::vtable(vt_object).$name$(.$name1)*)($crate::VtObject::as_ptr(vt_object), $($arg)*)
	}};
}
