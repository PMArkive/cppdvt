/// Implementation detail of VTable-defining macros.
#[doc(hidden)]
#[macro_export]
macro_rules! vtable_impl {
	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ {}
		/* this */ {}
		/* bounds */ {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
	} => {
		::core::compile_error! {
			"expected `{` to begin vtable body, `[` to begin generic parameter list, `for` to specify the `this` type, or `where` to begin generic bound list"
		}
	};
	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ {}
		/* bounds */ {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
	} => {
		::core::compile_error! {
			"expected `{` to begin vtable body, `for` to specify the `this` type, or `where` to begin generic bound list"
		}
	};
	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ $this:tt
		/* bounds */ {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
	} => {
		::core::compile_error! {
			"expected `{` to begin vtable body, or `where` to begin generic bound list"
		}
	};
	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ $this:tt
		/* bounds */ $bounds:tt
		$attrs:tt
		$vt_vis:vis $VTable:ident
	} => {
		::core::compile_error! {
			"expected `{` to begin vtable body"
		}
		$crate::vtable_impl! {
			@fill_this $this
			$attrs
			$vt_vis $VTable
			$fn_ty_macro
			$generics
			$bounds
		}
	};

	{
		@parse_after_name_bounds
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ $this:tt
		$attrs:tt
		$vt_vis:vis $VTable:ident
		$bounds:tt
		{$($item:tt)*}
		$($trailing:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name
			$fn_ty_macro;
			/* generics */ $generics
			/* this */ $this
			/* bounds */ $bounds
			$attrs
			$vt_vis $VTable
			{$($item)*}
			$($trailing)*
		}
	};
	{
		@parse_after_name_bounds
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ $this:tt
		$attrs:tt
		$vt_vis:vis $VTable:ident
		{$($bound:tt)*}
		$tt:tt
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name_bounds
			$fn_ty_macro;
			$generics
			$this
			$attrs
			$vt_vis $VTable
			{$($bound)* $tt}
			$($rest)*
		}
	};

	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ $this:tt
		/* bounds */ {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
		where
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name_bounds
			$fn_ty_macro;
			/* generics */ $generics
			/* this */ $this
			$attrs
			$vt_vis $VTable
			{where}
			$($rest)*
		}
	};
	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ {}
		/* bounds */ {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
		for $This:ty
		{$($item:tt)*}
		$($trailing:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name
			$fn_ty_macro;
			/* generics */ $generics
			/* this */ {$This}
			/* bounds */ {}
			$attrs
			$vt_vis $VTable
			{$($item)*}
			$($trailing)*
		}
	};
	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ {}
		/* bounds */ {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
		for $This:ty
		where
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name_bounds
			$fn_ty_macro;
			/* generics */ $generics
			/* this */ {$This}
			$attrs
			$vt_vis $VTable
			{where}
			$($rest)*
		}
	};

	{
		@parse_after_name
		$fn_ty_macro:path;
		/* generics */ $generics:tt
		/* this */ $this:tt
		/* bounds */ $bounds:tt
		$attrs:tt
		$vt_vis:vis $VTable:ident
		{$($item:tt)*}
		$($trailing:tt)*
	} => {
		$crate::vtable_impl! {@deny_trailing $($trailing)*}
		$crate::vtable_impl! {
			@fill_this $this
			$attrs
			$vt_vis $VTable
			$fn_ty_macro
			$generics
			$bounds
			$($item)*
		}
	};

	{@deny_trailing} => {};
	{@deny_trailing $($trailing:tt)+} => {
		::core::compile_error! {
			"unexpected trailing characters"
		}
	};

	{
		@fill_this {}
		$attrs:tt
		$vt_vis:vis $VTable:ident
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@create
			$crate::VtObjectPtr<Self>
			$attrs
			$vt_vis $VTable
			$($rest)*
		}
	};
	{
		@fill_this {$This:ty}
		$attrs:tt
		$vt_vis:vis $VTable:ident
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@create
			$This
			$attrs
			$vt_vis $VTable
			$($rest)*
		}
	};

	{
		@create
		$This:ty
		{$($attr:tt)*}
		$vt_vis:vis $VTable:ident
		$fn_ty_macro:path
		{$($generic:tt)*}
		{$($bound:tt)*}
		$(
			$(#[$fn_attr:meta])*
			$fn_vis:vis fn $fn_name:ident($($fn_param:tt)*) $(-> $FnRet:ty)?;
		)*
	} => {
		#[repr(C)]
		$($attr)*
		$vt_vis struct $VTable $($generic)* $($bound)* {
			$(
				$(#[$fn_attr])*
				$fn_vis $fn_name: $fn_ty_macro!(
					fn(this: $This, $($fn_param)*) $(-> $FnRet)?
				),
			)*
		}
	};
	{
		@create
		$This:ty
		{$($attr:tt)*}
		$vt_vis:vis $VTable:ident
		$fn_ty_macro:path
		{$($generic:tt)*}
		$bounds:tt
		$($whatever:tt)*
	} => {
		::core::compile_error! {
			"only `fn` items are allowed in vtable bodies"
		}
	};
}

/// Generates a VTable `struct` with a domain-specific language.
/// 
/// Generated structs have `#[repr(C)]` applied to them,
/// and functions defined within them
/// have the appropriate calling convention
/// based on the target and the function signature.
/// 
/// VTables can be used with [`virtual_call!`](crate::virtual_call).
/// 
/// # Function order
/// Generally, virtual functions must be defined in order of their appearance
/// in the header file of the class they are defined in.
/// However, some functions may or may not be in-lined by the compiler
/// for various reasons.
/// 
/// On `cfg(not(windows))`, there are *two* virtual destructors.
/// 
/// # Examples
/// A simple VTable can be defined like this:
/// ```
/// # use cppdvt::vtable;
/// // Assume that `Pet` is a class with a VTable that has the method
/// // `Pet::speak()`, which returns nothing.
/// 
/// vtable! {
/// 	/// VTable for `Pet`.
/// 	#[derive(Debug)]
/// 	pub(crate) PetVt {
/// 		pub fn speak();
/// 	}
/// }
/// ```
/// 
/// A VTable for a pre-defined type (for example, if there is inheritance) can
/// be defined by making the `this` a different type:
/// ```
/// # use cppdvt::vtable;
/// use core::ffi::c_char;
/// 
/// // Assume that `Pet` is a class with a VTable that has the method
/// // `Pet::speak()`, which returns nothing, the method `Pet::kind()`, which
/// // returns the kind of pet it is, and the method `Pet::name()`, which
/// // returns the name of the pet.
/// 
/// /// Enumeration of possible pets as defined in C.
/// #[repr(C)]
/// pub enum PetKind {
/// 	Lizard = 0,
/// 	Snake = 1,
/// }
/// 
/// vtable! {
/// 	/// VTable for `Pet`.
/// 	pub PetVt {
/// 		#[doc = "Make the Pet call their callback for speaking."]
/// 		pub fn speak() -> ();
/// 		pub fn kind() -> PetKind;
/// 		pub fn name() -> *const c_char;
/// 	}
/// }
/// 
/// vtable! {
/// 	/// VTable extension for `Lizard`, which extends from `PetVt`.
/// 	LizardVtExt {
/// 		pub fn derp();
/// 	}
/// }
/// 
/// /// VTable for `Lizard`.
/// #[repr(C)]
/// pub struct LizardVt {
/// 	base: PetVt,
/// 	lizard: LizardVtExt
/// }
/// 
/// vtable! {
/// 	/// VTable extension for `Snake`, which extends from `PetVt`.
/// 	SnakeVtExt {
/// 		pub fn curl(outer_radius: u32);
/// 	}
/// }
/// 
/// /// VTable for `Snake`.
/// #[repr(C)]
/// pub struct SnakeVt {
/// 	base: PetVt,
/// 	snake: SnakeVtExt
/// }
/// ```
/// 
/// VTables can also have generic types:
/// ```
/// # use cppdvt::{VtObjectPtr, vtable, virtual_fn};
/// vtable! {
/// 	pub List[T] {
/// 		pub fn is_empty() -> bool;
/// 		pub fn push(value: T) -> bool;
/// 	}
/// }
/// 
/// let list = {
/// 	virtual_fn! {
/// 		fn is_empty(this: VtObjectPtr<List<u8>>) -> bool {
/// 			true
/// 		}
/// 	}
/// 	virtual_fn! {
/// 		fn push(this: VtObjectPtr<List<u8>>, value: u8) -> bool {
/// 			let _ = this;
/// 			let _ = value;
/// 			false
/// 		}
/// 	}
/// 	List {
/// 		is_empty,
/// 		push,
/// 	}
/// };
/// let _: virtual_fn!(fn(this: VtObjectPtr<List<u8>>) -> bool) = list.is_empty;
/// let _: virtual_fn!(fn(this: VtObjectPtr<List<u8>>, value: u8) -> bool) = list.push;
/// ```
#[macro_export]
macro_rules! vtable {
	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $VTable:ident
		[$($generic:tt)*]
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name
			$crate::virtual_fn;
			{<$($generic)*>}
			{}
			{}
			{$(#[$vt_attr])*}
			$vt_vis $VTable
			$($rest)*
		}
	};
	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $VTable:ident
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name
			$crate::virtual_fn;
			{}
			{}
			{}
			{$(#[$vt_attr])*}
			$vt_vis $VTable
			$($rest)*
		}
	};
}

/// Like [`vtable!`](crate::vtable!),
/// but uses unwinding functions.
/// 
/// See the documentation of [`vtable!`](crate::vtable!) for more information.
#[macro_export]
macro_rules! unwind_vtable {
	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $VTable:ident
		[$($generic:tt)*]
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name
			$crate::unwind_virtual_fn;
			{<$($generic)*>}
			{}
			{}
			{$(#[$vt_attr])*}
			$vt_vis $VTable
			$($rest)*
		}
	};
	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $VTable:ident
		$($rest:tt)*
	} => {
		$crate::vtable_impl! {
			@parse_after_name
			$crate::unwind_virtual_fn;
			{}
			{}
			{}
			{$(#[$vt_attr])*}
			$vt_vis $VTable
			$($rest)*
		}
	};
}
