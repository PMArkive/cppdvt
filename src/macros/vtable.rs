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
/// use cppdvt::vtable;
/// 
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
/// use core::ffi::c_char;
/// 
/// use cppdvt::vtable;
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
#[macro_export]
macro_rules! vtable {
	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $vt_name:ident for $vt_this:ty {
			$(
				$(#[$fn_attr:meta])*
				$fn_vis:vis fn $fn_name:ident($($fn_param:tt)*) $(-> $fn_ret:ty)?;
			)*
		}
	} => {
		$(#[$vt_attr])*
		#[repr(C)]
		$vt_vis struct $vt_name {
			$(
				$(#[$fn_attr])*
				$fn_vis $fn_name: $crate::virtual_fn!(
					fn(this: $vt_this, $($fn_param)*) $(-> $fn_ret)?
				),
			)*
		}
	};

	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $vt_name:ident {
			$($arg:tt)*
		}
	} => {
		$crate::vtable! {
			$(#[$vt_attr])*
			$vt_vis $vt_name for $crate::VtObjectPtr<$vt_name> {
				$($arg)*
			}
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
		$vt_vis:vis $vt_name:ident for $vt_this:ty {
			$(
				$(#[$fn_attr:meta])*
				$fn_vis:vis fn $fn_name:ident($($fn_param:tt)*) $(-> $fn_ret:ty)?;
			)*
		}
	} => {
		$(#[$vt_attr])*
		#[repr(C)]
		$vt_vis struct $vt_name {
			$(
				$(#[$fn_attr])*
				$fn_vis $fn_name: $crate::unwind_virtual_fn!(
					fn(this: $vt_this, $($fn_param)*) $(-> $fn_ret)?
				),
			)*
		}
	};

	{
		$(#[$vt_attr:meta])*
		$vt_vis:vis $vt_name:ident {
			$($arg:tt)*
		}
	} => {
		$crate::vtable! {
			$(#[$vt_attr])*
			$vt_vis $vt_name for $crate::VtObjectPtr<$vt_name> {
				$($arg)*
			}
		}
	};
}
