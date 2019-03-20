extern crate num_traits;

use std::str::FromStr;

use std::fmt::{Debug, Display, Formatter};

use std::ops::{Add, Sub, AddAssign, SubAssign};

use num_traits::identities;
use num_traits::{Bounded, WrappingAdd, WrappingSub, CheckedAdd, CheckedSub};
                                                             // This is pretty wonk..

pub trait AddressDisplay {
    fn stringy(&self) -> String;
}

pub trait Address where Self:
    Debug + Display + AddressDisplay +
    Copy + Clone + Sized +
    Ord + Eq + PartialEq + Bounded +
    Add<Output=Self> + Sub<Output=Self> +
    AddAssign + SubAssign +
    WrappingAdd + WrappingSub +
    CheckedAdd + CheckedSub +
    FromStr +
    identities::One + identities::Zero {
    fn to_linear(&self) -> usize;

}
/*
impl <T> Address for T where T: Sized + Ord + Add<Output=Self> + From<u16> + Into<usize> {
    fn to_linear(&self) -> usize { *self.into() }
}
*/

impl AddressDisplay for usize {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddressDisplay for u64 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddressDisplay for u32 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl AddressDisplay for u16 {
    fn stringy(&self) -> String {
        format!("{:#x}", self)
    }
}

impl Address for u16 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u32 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for u64 {
    fn to_linear(&self) -> usize { *self as usize }
}

impl Address for usize {
    fn to_linear(&self) -> usize { *self }
}

pub trait Decodable where Self: Sized {
    fn decode<T: IntoIterator<Item=u8>>(bytes: T) -> Option<Self>;
    fn decode_into<T: IntoIterator<Item=u8>>(&mut self, bytes: T) -> Option<()>;
}

pub trait Arch {
    type Address: Address + Debug;
    type Instruction: Decodable + LengthedInstruction<Unit=Self::Address> + Debug;
    type Operand;
}

pub trait LengthedInstruction {
    type Unit;
    fn len(&self) -> Self::Unit;
    fn min_size() -> Self::Unit;
}

pub struct ColorSettings {
}

/*
 * can this be a derivable trait or something?
 */
/*
impl <T: Colorize> Display for T {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        self.colorize(None, fmt)
    }
}
*/

pub trait Colorize<T: std::fmt::Write> {
    fn colorize(&self, colors: Option<&ColorSettings>, out: &mut T) -> std::fmt::Result;
}

/*
 * and make this auto-derive from a ShowContextual impl?
 */
/*
impl <T, U> Colorize for T where T: ShowContextual<Ctx=U> {
    fn colorize(&self, colors: Option<&ColorSettings>, fmt: &mut Formatter) -> std::fmt::Result {
        self.contextualize(colors, None, fmt)
    }
}
*/

pub trait ShowContextual<Addr, Ctx: ?Sized, T: std::fmt::Write> {
    fn contextualize(&self, colors: Option<&ColorSettings>, address: Addr, context: Option<&Ctx>, out: &mut T) -> std::fmt::Result;
}

/*
impl <C: ?Sized, T: std::fmt::Write, U: Colorize<T>> ShowContextual<C, T> for U {
    fn contextualize(&self, colors: Option<&ColorSettings>, context: Option<&C>, out: &mut T) -> std::fmt::Result {
        self.colorize(colors, out)
    }
}
*/
