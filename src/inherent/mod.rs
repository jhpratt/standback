#[cfg(shim = "1.37")] mod v1_37;
#[cfg(shim = "1.38")] mod v1_38;
#[cfg(shim = "1.40")] mod v1_40;
#[cfg(shim = "1.41")] mod v1_41;
#[cfg(shim = "1.42")] mod v1_42;
#[cfg(shim = "1.43")] mod v1_43;
#[cfg(shim = "1.44")] mod v1_44;
#[cfg(shim = "1.45")] mod v1_45;
#[cfg(shim = "1.46")] mod v1_46;
#[cfg(shim = "1.47")] mod v1_47;
#[cfg(shim = "1.48")] mod v1_48;
#[cfg(shim = "1.49")] mod v1_49;
#[cfg(shim = "1.50")] mod v1_50;
#[cfg(shim = "1.51")] mod v1_51;
#[cfg(shim = "1.52")] mod v1_52;
#[cfg(shim = "1.53")] mod v1_53;
#[cfg(shim = "1.54")] mod v1_54;
#[cfg(shim = "1.55")] mod v1_55;

#[allow(unused_imports)] use sealed::Sealed;
mod sealed {
    #[allow(unreachable_pub)]
    pub trait Sealed<T: ?Sized> {}
    impl<T: ?Sized> Sealed<T> for T {}
}

#[cfg(shim = "1.37")]
#[allow(unreachable_pub)]
pub use v1_37::*;
#[cfg(shim = "1.38")]
#[allow(unreachable_pub)]
pub use v1_38::*;
#[cfg(shim = "1.40")]
#[allow(unreachable_pub)]
pub use v1_40::*;
#[cfg(shim = "1.41")]
#[allow(unreachable_pub)]
pub use v1_41::*;
#[cfg(shim = "1.42")]
#[allow(unreachable_pub)]
pub use v1_42::*;
#[cfg(shim = "1.43")]
#[allow(unreachable_pub)]
pub use v1_43::*;
#[cfg(shim = "1.44")]
#[allow(unreachable_pub)]
pub use v1_44::*;
#[cfg(shim = "1.45")]
#[allow(unreachable_pub)]
pub use v1_45::*;
#[cfg(shim = "1.46")]
#[allow(unreachable_pub)]
pub use v1_46::*;
#[cfg(shim = "1.47")]
#[allow(unreachable_pub)]
pub use v1_47::*;
#[cfg(shim = "1.48")]
#[allow(unreachable_pub)]
pub use v1_48::*;
#[cfg(shim = "1.49")]
#[allow(unreachable_pub)]
pub use v1_49::*;
#[cfg(shim = "1.50")]
#[allow(unreachable_pub)]
pub use v1_50::*;
#[cfg(shim = "1.51")]
#[allow(unreachable_pub)]
pub use v1_51::*;
#[cfg(shim = "1.52")]
#[allow(unreachable_pub)]
pub use v1_52::*;
#[cfg(shim = "1.53")]
#[allow(unreachable_pub)]
pub use v1_53::*;
#[cfg(shim = "1.54")]
#[allow(unreachable_pub)]
pub use v1_54::*;
#[cfg(shim = "1.55")]
#[allow(unreachable_pub)]
pub use v1_55::*;
