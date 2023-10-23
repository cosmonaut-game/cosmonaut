pub use bnum::cast::As as _;
use bnum::types::U256;

pub type UInt = U256;
pub const PRECISION: UInt = UInt::TEN.pow(6);
pub const PREC_F64: f64 = 10u32.pow(6) as _;
pub const PI_INT: UInt = UInt::parse_str_radix("3141592", 10);
pub const TAU_INT: UInt = UInt::parse_str_radix("6283185", 10);
pub use bnum;
