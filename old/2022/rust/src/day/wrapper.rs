use crate::day::{Calculation, Coordinate, Input, Loc, Path, Pos, Sensor, Valve};
use std::collections::{BTreeMap, HashSet, HashMap};

pub trait Wrapper<T> {
    fn unwrap(self) -> T;
}

macro_rules! wrapping {
    ($type:ty, $element:ident) => {
        impl Wrapper<$type> for Input {
            fn unwrap(self) -> $type {
                match self {
                    Input::$element(x) => x,
                    _ => panic!("invalid wrapping"),
                }
            }
        }
    };
}

wrapping!(Vec<u32>, Vu32);
wrapping!(Vec<String>, Vstr);
wrapping!(Vec<(i32, i32)>, VTi32);
wrapping!(Vec<(Loc, Loc)>, VTLoc);
wrapping!((HashSet<Pos>, i32), D14);
wrapping!((Vec<Sensor>, Vec<(i64, i64)>, (i64, i64)), D15);
wrapping!(HashMap<String, Valve>, D16);
wrapping!((Vec<char>, Vec<(char, Vec<Pos>)>), D17);
wrapping!(Calculation, D21);
wrapping!((Vec<Path>, BTreeMap<Coordinate, char>), D22);