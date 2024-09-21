#![feature(fundamental)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(never_type)]
#![allow(incomplete_features)]
#![feature(specialization)]

use std::marker::{PhantomData};
use generic::GenericListValue;
use crate::generic::*;

mod generic;

/*trait TupleBiMap<TCon: TypeConstraint> : Tuple{
    type GenericList: GenericLinkedList<TCon>;
}

trait TupleListMatch<T: Tuple, List: GenericLinkedListFamily<GenericLinkedListSelfType=List, GenericLinkedListTypeConstraint=TCon>, TCon: TypeConstraint>{}
impl<T: Tuple + Into<List>, List: GenericLinkedListFamily<GenericLinkedListSelfType=List, GenericLinkedListTypeConstraint=TCon>, TCon: TypeConstraint> TupleListMatch<T, List, TCon> for PhantomData<(T, List)>
where List: From<T>{
}
impl<T: Tuple + TupleListMatch<T, List, TCon>, List: GenericLinkedListFamily<GenericLinkedListSelfType=List, GenericLinkedListTypeConstraint=TCon>, TCon: TypeConstraint> TupleBiMap<TCon> for T{
    type GenericList = List;
}*/
mod same_type;

pub use same_type::{Same, SameSelf};


#[allow(private_bounds)]
pub struct NumberTuple<TGenericList: GenericLinkedList<NumberTypeConstraint>>(TGenericList);
pub struct StringTuple<TGenericList: GenericLinkedList<SingleTypeConstraint<String>>>(TGenericList);


#[allow(private_bounds)]
impl<TGenericList: GenericLinkedList<NumberTypeConstraint> + SameSelf<TGenericList>> NumberTuple<TGenericList> {
    pub extern "rust-call" fn  new<T>(tuple: T) -> NumberTuple<TGenericList>
    where T: TupleIntoList<NumberTypeConstraint, List=TGenericList>
    {
        NumberTuple(T::into_generic_list(tuple))
    }

    #[allow(private_interfaces)]
    pub fn map_to_string(self) -> StringTuple<<TGenericList as GenericLinkedList<NumberTypeConstraint>>::LinkType<ToStringFn>> {
        StringTuple(self.0.map::<ToStringFn>())
    }
}


impl<TGenericList: GenericLinkedList<NumberTypeConstraint>> GenericTuple<TGenericList, NumberTypeConstraint> for NumberTuple<TGenericList> {
}



#[allow(dead_code)]
fn test2(){
    let x = NumberTuple::new((1u16, 2.2, 3i32, 4i64, 5u128, 6isize, 7usize, 8f32, 9f64, 10i16));
    x.map_to_string();
/*    let fntest = NumberTuple::new as extern "rust-call" fn(_) -> _;
    // let x = fntest((1u16, 2.2, 3i32, 4i64, 5u128, 6isize, 7usize, 8f32, 9f64, 10i16));
    let x = fntest(("", ));
    */
    //let x = NumberTuple::new(("a", ));
    //let x = NumberTuple::new2((1u16, ));
}



enum NumberTypeConstraint{}
impl TypeConstraint for NumberTypeConstraint{
    type Type<T: TypeConstraintImpl<Self>> = T;
}
trait NumberTypeConstraintImpl: TypeConstraintImpl<NumberTypeConstraint> + ToString {}
impl<T:NumberTypeConstraintImpl> TypeConstraintImpl<NumberTypeConstraint> for T{}





impl NumberTypeConstraintImpl for i16{}
impl NumberTypeConstraintImpl for u16{}
impl NumberTypeConstraintImpl for i32{}
impl NumberTypeConstraintImpl for u32{}
impl NumberTypeConstraintImpl for i64{}
impl NumberTypeConstraintImpl for u64{}
impl NumberTypeConstraintImpl for i128{}
impl NumberTypeConstraintImpl for u128{}
impl NumberTypeConstraintImpl for isize{}
impl NumberTypeConstraintImpl for usize{}
impl NumberTypeConstraintImpl for f32{}
impl NumberTypeConstraintImpl for f64{}

pub struct SingleTypeConstraint<T>(PhantomData<T>);
impl<T> TypeConstraint for SingleTypeConstraint<T>{
    type Type<U: TypeConstraintImpl<Self>> = T;
}
impl<T> TypeConstraintImpl<SingleTypeConstraint<T>> for T{}


enum ToStringFn{}
impl FnMetaTrait for ToStringFn {
    type TMap = Self;
    type TConFrom = NumberTypeConstraint;
    type OtherArgs = ();
}
impl<T: TypeConstraintImpl<NumberTypeConstraint>> FnMap<T> for ToStringFn{
    default type Next<U: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>> = Self;

    default fn call(_value: T, _: &()) -> <<Self as FnMetaTrait>::TMap as TypeMap<<Self as FnMetaTrait>::TConFrom>>::TMap<T> {
        unreachable!()
    }
}
impl<T: NumberTypeConstraintImpl> FnMap<T> for ToStringFn{
    type Next<U: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>> = Self;

    fn call(value: T, _: &()) -> <<Self as FnMetaTrait>::TMap as TypeMap<<Self as FnMetaTrait>::TConFrom>>::TMap<T> {
        value.to_string()
    }
}

impl TypeMap<NumberTypeConstraint> for ToStringFn{
    type TConTo = SingleTypeConstraint<String>;
    type TMap<T: TypeConstraintImpl<NumberTypeConstraint>> = String;
}
/*impl FnMap<NumberTypeConstraint> for ToStringFn{
    fn call<T: TypeConstraintImpl<NumberTypeConstraint>>(value: T) -> String {
        value.to_string()
    }
}*/


fn main() {
    let list = var_args!(NumberTypeConstraint, 1u16, 2.2, 3i32, 4i64, 5u128, 6isize, 7usize, 8f32, 9f64, 10i16);
    let list = list.map::<ToStringFn>();
    let (a, b, c, d, e, f, g, h, i, j) = list.into();
    println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h, i, j);
    
    let list = NumberTuple::new((1u16, ));
    let list = NumberTuple::new((1u16, 2.2, 3i32, 4i64, 5u128, 6isize, 7usize, 8f32, 9f64, 10i16));
    let list = list.map_to_string();
    let (a, b, c, d, e, f, g, h, i, j) = list.0.into();
    println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h, i, j);
}
