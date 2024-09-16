#![feature(fundamental)]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]
#![feature(negative_impls)]
#![feature(auto_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(never_type)]
#![feature(associated_const_equality)]
#![feature(specialization)]

//#![feature(associated_type_defaults)]

use std::arch::x86_64::__m128;
use std::hint::unreachable_unchecked;
use std::marker::{PhantomData, Tuple};
use crate::optional_type::{OptionalType, OptionalTypeMarker};

#[allow(refining_impl_trait)]

pub trait TypeConstraint : Sized {
    type Type<T: TypeConstraintImpl<Self>>: Sized + TypeConstraintImpl<Self>;
}
pub trait TypeMap<TConFrom: TypeConstraint> {
    type TConTo: TypeConstraint;
    type TMap<T: TypeConstraintImpl<TConFrom>>: TypeConstraintImpl<Self::TConTo>;
}


pub trait FnMap<TConFrom: TypeConstraint> : TypeMap<TConFrom>{
    fn call<T: TypeConstraintImpl<TConFrom>>(value: T) -> Self::TMap<T>;
}

pub trait FnMetaTrait {
    type TMap: TypeMap<Self::TConFrom>;
    type TConFrom: TypeConstraint;
    //type T: TypeConstraintImpl<Self::TConFrom>;
}

pub trait FnMap2<T: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>> : FnMetaTrait {
    type Next<U: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>>: FnMap2<U, TConFrom=<Self as FnMetaTrait>::TConFrom, TMap=<Self as FnMetaTrait>::TMap>;
    fn call(value: T) -> <<Self as FnMetaTrait>::TMap as TypeMap<<Self as FnMetaTrait>::TConFrom>>::TMap<T>;
}
/*impl<Fn, T> FnMetaTrait for Fn
where
    Fn: FnMap2<T>,
    TMap: TypeMap<TConFrom>,
    TConFrom: TypeConstraint,
    T: TypeConstraintImpl<TConFrom>
{
    type TMap = TMap;
    type TConFrom = TConFrom;
    type T = T;
}*/
pub trait IsFnParam<Fn: FnMetaTrait>{}
impl<Fn: FnMetaTrait> IsFnParam<Fn> for Fn{}



#[fundamental]
pub trait TypeConstraintImpl<TCon: TypeConstraint>: Sized{
}
/*impl<T: TypeConstraintImplDisjunction<TCon, true>, TCon: TypeConstraint> !TypeConstraintImplDisjunction<TCon, false> for T{}
impl<T: TypeConstraintImplDisjunction<TCon, false>, TCon: TypeConstraint> !TypeConstraintImplDisjunction<TCon, true> for T{}

impl<T: TypeConstraintImplDisjunction<TCon, true>, TCon: TypeConstraint> TypeConstraintImpl<TCon> for T{}
impl<T: TypeConstraintImplDisjunction<TCon, false>, TCon: TypeConstraint> TypeConstraintImpl<TCon> for T{}
pub trait TypeConstraintImplDisjunction<TCon: TypeConstraint, const FULFILLED: bool>: Sized{}*/

#[derive(Default)]
struct Sealed{}

//type OptionalNode<TNote: GenericLinkedList<TCon>, TCon: TypeConstraint> = OptionalType<TNote::THasNext, TNote::NextType>;
type OptionalNode<TNote: GenericListValueBase<T>, T: TypeConstraintImpl<TNote::TCon>> = OptionalType<<TNote as GenericLinkedList<TNote::TCon>>::THasNext, TNote::TNext>;
type OptionalNodeMapped<TNote, TCon, Map>
where
    TNote: GenericLinkedList<TCon>,
    TCon: TypeConstraint,
    Map: TypeMap<TCon>
= OptionalType<<TNote as GenericLinkedList<TCon>>::THasNext, <TNote::NextType as GenericLinkedList<TCon>>::LinkType<Map>>;

// Define the trait A
pub trait GenericLinkedList<TCon: TypeConstraint> : GenericListValueBase<Self::InnerType, TCon=TCon, TNext=Self::NextType, THasNext=<Self as GenericLinkedList<TCon>>::THasNext>
{
    type InnerType: TypeConstraintImpl<TCon>;
    type LinkType<Map: TypeMap<TCon>>: GenericLinkedList<Map::TConTo>;
    type NextType: GenericLinkedList<TCon>;

    type THasNext: OptionalTypeMarker;
    const LENGTH: usize;

    fn _create<Map: TypeMap<TCon>>(value: Map::TMap<Self::InnerType>, next: OptionalNodeMapped<Self, TCon, Map>, _: Sealed) -> Self::LinkType<Map>;

    ///if we get const fn in traits on day we should use it here

    fn map<F: FnMap<TCon>>(self) -> Self::LinkType<F>;
/*    fn map2<F>(self) -> Self::LinkType<<F as FnMetaTrait>::TMap>
    where
        <Self as GenericLinkedList<TCon>>::Variant: TypeConstraintImpl<<F as FnMetaTrait>::TConFrom>,
        F: FnMap2<<Self as GenericLinkedList<TCon>>::Variant>,
        <F as FnMetaTrait>::TMap: TypeMap<TCon>,
        <Self as GenericLinkedList<TCon>>::InnerType: IsFnParam<F>;*/


}

pub struct GenericListEnd<T, TCon>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
      T: TypeConstraintImpl<TCon>,
      TCon: TypeConstraint
{
    pub value: T,
    pub phantom: std::marker::PhantomData<TCon> //we need this to constrain the generic
}



impl<T, TCon> GenericLinkedList<TCon> for GenericListEnd<T, TCon>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint
{
    type InnerType = T;
    type LinkType<Map: TypeMap<TCon>> = GenericListEnd<Map::TMap<Self::InnerType>, Map::TConTo>;
    type NextType = Self; //only ever used together with THasNext, so impossible to use
    type THasNext = !;
    const LENGTH: usize = 1;

    fn _create<Map: TypeMap<TCon>>(value: Map::TMap<Self::InnerType>, next: OptionalNodeMapped<Self, TCon, Map>, _: Sealed) -> Self::LinkType<Map> {
        use OptionalType::*;
        match next {
            NoType(a) => {}
            Type(_, never) => {never}
        }
        GenericListEnd {
            value,
            phantom: std::marker::PhantomData::<Map::TConTo>,
        }
    }


    fn map<F: FnMap<TCon>>(self) -> Self::LinkType<F> {
        GenericListEnd {
            value: F::call(self.value),
            phantom: std::marker::PhantomData::<F::TConTo>,
        }
    }

/*    fn map2<F>(self) -> Self::LinkType<<F as FnMetaTrait>::TMap>
    where
        <Self as GenericLinkedList<TCon>>::Variant: TypeConstraintImpl<<F as FnMetaTrait>::TConFrom>,
        F: FnMap2<<Self as GenericLinkedList<TCon>>::Variant>,
        <F as FnMetaTrait>::TMap: TypeMap<TCon>,
        <Self as GenericLinkedList<TCon>>::InnerType: IsFnParam<F>
    {

        GenericListEnd {
            value: F::call(self.value),
            phantom: std::marker::PhantomData::<<<F as FnMetaTrait>::TMap as TypeMap<TCon>>::TConTo>,
        }
    }*/
}
impl<T, TCon> !GenericListLinkTrait<TCon> for GenericListEnd<T, TCon>{}

pub struct GenericListLink<T, TCon, T2>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    T2: GenericLinkedList<TCon>
{
    pub value: T,
    pub next: T2,
    pub phantom: std::marker::PhantomData<TCon>
}
trait GenericListLinkTrait<TCon: TypeConstraint>{}
impl<T, TCon, T2> GenericListLinkTrait<TCon> for GenericListLink<T, TCon, T2>
where
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    T2: GenericLinkedList<TCon>
{
}



impl<T: TypeConstraintImpl<TCon>, TCon: TypeConstraint, T2: GenericLinkedList<TCon>> GenericLinkedList<TCon> for GenericListLink<T, TCon, T2> {
    type InnerType = T;
    type LinkType<Map: TypeMap<TCon>> = GenericListLink<Map::TMap<Self::InnerType>, Map::TConTo, T2::LinkType<Map>>;
    type NextType = T2;
    type THasNext = ();
    const LENGTH: usize = T2::LENGTH + 1;

    fn _create<Map: TypeMap<TCon>>(value: Map::TMap<Self::InnerType>, next: OptionalNodeMapped<Self, TCon, Map>, _: Sealed) -> Self::LinkType<Map> {
        use OptionalType::*;
        let next = match next {
            NoType(a) => {unreachable!()}
            Type(v, _) => v
        };
        GenericListLink {
            value,
            next,
            phantom: PhantomData,
        }
    }

    fn map<F: FnMap<TCon>>(self) -> Self::LinkType<F> {
        GenericListLink {
            value: F::call(self.value),
            next: self.next.map(),
            phantom: std::marker::PhantomData::<F::TConTo>,
        }
    }
}


pub mod optional_type{

    trait Sealed<>{}

    pub trait OptionalTypeMarker: Sealed + Sized{
        type Opposite: OptionalTypeMarker<Opposite=Self, Existential=()>;
        type Existential: OptionalTypeMarker<Opposite=!, Existential=Self::Existential>;
        type TArg<T>;

        fn new<T>(value: Self::TArg<T>) -> OptionalType<Self, T>
        where Self::Existential: OptionalTypeMarker<TArg<T>=T>;
    }
    pub enum OptionalType<HasT:OptionalTypeMarker, T>{
        NoType(HasT::Opposite),
        Type(T, HasT),
    }

    impl<HasT: OptionalTypeMarker, T> OptionalType<HasT, T> {
        pub fn map<U, F>(self, f: F) -> OptionalType<HasT, U>
        where
            F: FnOnce(T) -> U,
        {
            match self {
                OptionalType::NoType(marker) => OptionalType::NoType(marker),
                OptionalType::Type(value, marker) => OptionalType::Type(f(value), marker),
            }
        }
    }

    impl Sealed for ! {}

    impl OptionalTypeMarker for !{
        type Opposite = ();
        type Existential = ();
        type TArg<T> = ();

        fn new<T>(_: ()) -> OptionalType<Self, T> {
            use crate::optional_type::OptionalType::NoType;
            NoType(())
        }
    }

    impl Sealed for () {}

    impl OptionalTypeMarker for (){
        type Opposite = !;
        type Existential = ();
        type TArg<T> = T;

        fn new<T>(value: T) -> OptionalType<Self, T> {
            use crate::optional_type::OptionalType::Type;
            Type(value, ())
        }
    }


}

pub trait GenericListValue<T, TCon>: GenericLinkedList<TCon, InnerType=T> + GenericListValueBase<T, TCon=TCon, TNext=Self::NextType>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    T: TypeConstraintImpl<TCon>,
{
    fn map2<F>(self) -> <Self as GenericLinkedList<TCon>>::LinkType<F::TMap>
    where
        F: FnMap2<T, TConFrom=TCon>
    {
        let (value, next) = self.deconstruct();
        let value = F::call(value);
        let next = next.map(|next| next.map2::<F::Next<_>>());
        Self::_create(value, next, Sealed {})


    }
}
impl<TLink, T, TCon> GenericListValue<T, TCon> for TLink
where TLink: GenericLinkedList<TCon, InnerType=T> + GenericListValueBase<T, TCon=TCon, TNext=Self::NextType>,
<TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
TCon: TypeConstraint,
T: TypeConstraintImpl<TCon>, {}

pub trait GenericListValueBase<T> : Sized
where
    <Self::TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<Self::TCon>,
    T: TypeConstraintImpl<Self::TCon>,

{
    type TCon: TypeConstraint;
    type TNext: GenericLinkedList<Self::TCon>;
    type THasNext: OptionalTypeMarker;

    fn value_ref(&self) -> &T;
    fn value_mut(&mut self) -> &mut T;
    fn deconstruct(self) -> (T, OptionalType<Self::THasNext, Self::TNext>);

}
impl<T: TypeConstraintImpl<TCon>, TCon: TypeConstraint> GenericListValueBase<T> for GenericListEnd<T, TCon> {
    type TCon = TCon;
    type TNext = Self;
    type THasNext = !;

    fn value_ref(&self) -> &T {
        &self.value
    }

    fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn deconstruct(self) -> (T, OptionalType<Self::THasNext, Self::TNext>)
    {
        (self.value, OptionalTypeMarker::new(()))
    }

    // fn deconstruct(self) -> (T, OptionalType<!, Self::TNext>) {
    //     (self.value, OptionalTypeMarker::new(()))
    // }
}

impl<T, TCon, T2> GenericListValueBase<T> for GenericListLink<T, TCon, T2>
where
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    T2: GenericLinkedList<TCon>
{
    type TCon = TCon;
    type TNext = T2;
    type THasNext = ();

    fn value_ref(&self) -> &T {
        &self.value
    }

    fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn deconstruct(self) -> (T, OptionalType<Self::THasNext,  Self::TNext>) {
        (self.value, OptionalTypeMarker::new(self.next))
    }
}

/*impl<T, TCon, TNextValue, TNextNext> GenericListValue<T> for GenericListLink<T, TCon, GenericListLink<TNextValue, TCon, TNextNext>>
where
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    TNextValue: TypeConstraintImpl<TCon>,
    TNextNext: GenericLinkedList<TCon>
{
    type TCon = TCon;
    type THasNext = ();
    type TNext = GenericListLink<TNextValue, TCon, TNextNext>;

    fn value_ref(&self) -> &T {
        &self.value
    }

    fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn deconstruct(self) -> (T, Option<(Self::TNext, ())>) {
        (self.value, Some((self.next,())))
    }
}

impl<T, TCon, TNextValue> GenericListValue<TCon::Type<T>> for GenericListLink<T, TCon, GenericListEnd<TNextValue, TCon>>
where
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    TNextValue: TypeConstraintImpl<TCon>,
{
    type TCon = TCon;
    type THasNext = ();
    type TNext = GenericListEnd<TNextValue, TCon>;

    fn value_ref(&self) -> &TCon::Type<T> {
        &self.value
    }

    fn value_mut(&mut self) -> &mut TCon::Type<T> {
        &mut self.value
    }

    fn deconstruct(self) -> (TCon::Type<T>, Option<(Self::TNext, ())>) {
        (self.value, Some((self.next,())))
    }
}*/



#[macro_export] //must be exported as the macro is used inside an exported macro I guess
macro_rules! var_args {
    // Base case for single element
    ($TCons:ident, $T1:expr) => {
        $crate::GenericListEnd {
            value: $T1,
            phantom: std::marker::PhantomData::<$TCons>,
        }
    };
    // Recursive case for multiple elements
    ($TCons:ident, $T1:expr, $T2:expr $(, $rest:expr)*) => {
        $crate::GenericListLink {
            value: $T1,
            next: $crate::var_args!($TCons, $T2 $(, $rest)*),
            phantom: std::marker::PhantomData::<$TCons>,
        }
    };
}


macro_rules! _tcon_type {
    ($TCon:ident, $($T:ident),+) => {
        ($(<$TCon as TypeConstraint>::Type<$T>),+)
    };
}

macro_rules! _impl_into_for_tuple {
    // Base case for IntoInnerListEnd
    ($T:ident) => {
        #[allow(unused_parens)]
        impl<$T, TCon: TypeConstraint> Into<($T,)> for GenericListEnd<$T, TCon>
        where $T: TypeConstraintImpl<TCon> {
            fn into(self) -> ($T,) {
                (self.value,)
            }
        }
    };
    // Recursive case for IntoInnerListLink
    ($T:ident, $($Rest:ident),+) => {
        #[allow(non_snake_case)]
        impl<$T, $($Rest),+, TCon: TypeConstraint> Into<($T, $($Rest),+)> for _impl_into_for_tuple_full_generic!($T, $($Rest),+)
        where $T: TypeConstraintImpl<TCon>, $($Rest: TypeConstraintImpl<TCon>),+ {
            fn into(self) -> ( $T, $($Rest),+) {
                let (head, tail) = (self.value, self.next);
                let ($($Rest),+ ,) = tail.into();
                (head, $($Rest),+)
            }
        }
        _impl_into_for_tuple!($($Rest),+);
    };
}

macro_rules! _impl_into_for_tuple_full_generic {
    // Base case for single element
    ($T:ident) => {
        GenericListEnd<$T, TCon>
    };
    // Recursive case for multiple elements
    ($T:ident, $($Rest:ident),+) => {
        GenericListLink<$T, TCon, _impl_into_for_tuple_full_generic!($($Rest),+)>
    };
}

macro_rules! _impl_from_for_tuple {
    // Base case for From<(_tcon_type!(TCon, $T),)> for GenericListEnd
    ($T:ident) => {
        #[allow(unused_parens)]
        impl<$T, TCon: TypeConstraint> From<($T,)> for GenericListEnd<$T, TCon>
        where $T: TypeConstraintImpl<TCon> {
            fn from(tuple: ($T,)) -> Self {
                GenericListEnd {
                    value: tuple.0,
                    phantom: std::marker::PhantomData,
                }
            }
        }
    };
    // Recursive case for From<_tcon_type!(TCon, $T, $($Rest),+)> for GenericListLink
    ($T:ident, $($Rest:ident),+) => {
        #[allow(non_snake_case)]
        impl<$T, $($Rest),+, TCon: TypeConstraint> From<($T, $($Rest),+)> for _impl_into_for_tuple_full_generic!($T, $($Rest),+)
        where $T: TypeConstraintImpl<TCon>, $($Rest: TypeConstraintImpl<TCon>),+ {
            fn from(tuple: ($T, $($Rest),+)) -> Self {
                let (head, $($Rest),+) = tuple;
                GenericListLink {
                    value: head,
                    next: From::from(($($Rest),+,)),
                    phantom: std::marker::PhantomData,
                }
            }
        }
        _impl_from_for_tuple!($($Rest),+);
    };
}



//26 letters
// _impl_from_for_tuple!(A, B, C);
_impl_from_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);

//26 letters
_impl_into_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);



/*impl <TCon, TLink> GenericLinkedListWrapper<TCon, TLink>
where
    TLink: GenericLinkedList<TCon> + GenericListLinkTrait<TCon>,
    TCon: TypeConstraint,
{
    fn from_tuple<TTuple>(tuple: TTuple) -> TLink
    where
        TLink: FromTupleConstrained<TCon, TTuple> + From<TTuple>,
        TTuple: std::marker::Tuple + NotSinglet + From<TLink>,{
        tuple.into()
    }
}

impl <TCon, T> GenericLinkedListWrapper<TCon, GenericListEnd<T, TCon>>
where
    TCon: TypeConstraint,
    T: TypeConstraintImpl<TCon>,
    GenericListEnd<T, TCon>: Into<(T,)>
{
    fn from_tuple(tuple: (TCon::Type<T>,)) -> GenericListEnd<T, TCon> {
        tuple.into()
    }
}*/

/*#[repr(transparent)]
pub struct GenericLinkedListWrapper<TCon: TypeConstraint, TGenericList: GenericLinkedList<TCon>>(TGenericList, PhantomData<TCon>);
pub trait GenericLinkedListWrapped<TCon: TypeConstraint, TGenericList: GenericLinkedList<TCon>> {
    fn into(self) -> TGenericList;
}
impl <TCon, TGenericList> GenericLinkedListWrapped<TCon, TGenericList> for GenericLinkedListWrapper<TCon, TGenericList>
where
    TCon: TypeConstraint,
    TGenericList: GenericLinkedList<TCon>
{
    fn into(self) -> TGenericList {
        self.0
    }
}

pub trait FromTupleConstrained<TCon, TTuple>
where
    TCon: TypeConstraint,
    Self: GenericLinkedListWrapped<TCon, Self::TGenericList>,
    TTuple: std::marker::Tuple,
{
    type TGenericList: GenericLinkedList<TCon> + Into<TTuple> + From<TTuple>;

    fn from_tuple<TLink>(tuple: TTuple) -> TLink
    where TLink: GenericLinkedList<TCon> + From<TTuple>,
          TTuple: std::marker::Tuple;
}
impl <TCon, T> FromTupleConstrained<TCon, (TCon::Type<T>,)> for GenericLinkedListWrapper<TCon, GenericListEnd<T, TCon>>
where
    TCon: TypeConstraint,
    T: TypeConstraintImpl<TCon>,
    GenericListEnd<T, TCon>: Into<(T,)>
{
    type TGenericList = GenericListEnd<T, TCon>;

    fn from_tuple<TLink>(tuple: (TCon::Type<T>,)) -> TLink
    where
        TLink: GenericLinkedList<TCon> + From<(TCon::Type<T>,)>,
        (TCon::Type<T>,): Tuple
    {
        tuple.into()
    }
}
auto trait NotSinglet{}
impl<T> !NotSinglet for (T,){}


impl <TCon, TTuple, TLink> FromTupleConstrained<TCon, TTuple> for GenericLinkedListWrapper<TCon, TLink>
where
    TLink: GenericLinkedList<TCon> + GenericListLinkTrait<TCon> + From<TTuple>,
    TCon: TypeConstraint,
    TTuple: std::marker::Tuple + NotSinglet + From<TLink>,
{
    type TGenericList = TLink;

    fn from_tuple<ULink>(tuple: TTuple) -> ULink
    where
        ULink: GenericLinkedList<TCon> + From<TTuple>,
        TTuple: Tuple
    {
        tuple.into()
    }
}*/


pub trait GenericTuple<TGenericList: GenericLinkedList<TCon>, TCon: TypeConstraint> {
}

pub struct NumberTuple<TGenericList: GenericLinkedList<NumberTypeConstraint>>(TGenericList);
pub struct StringTuple<TGenericList: GenericLinkedList<SingleTypeConstraint<String>>>(TGenericList);

impl<TGenericList: GenericLinkedList<NumberTypeConstraint>> NumberTuple<TGenericList> {
    pub extern "rust-call" fn  new<T>(tuple: T) -> Self
    where TGenericList: From<T>,
    T: std::marker::Tuple{
        NumberTuple(TGenericList::from(tuple))
    }
    
    pub fn map_to_string(self) -> StringTuple<<TGenericList as GenericLinkedList<NumberTypeConstraint>>::LinkType<ToStringFn>> {
        StringTuple(self.0.map2::<ToStringFn>())
    }
}

fn test2(){
    let x: NumberTuple<GenericListLink<_,_,_>> = NumberTuple::new((1u16, 2.2, 3i32, 4i64, 5u128, 6isize, 7usize, 8f32, 9f64, 10i16));
}

impl<TGenericList: GenericLinkedList<NumberTypeConstraint>> GenericTuple<TGenericList, NumberTypeConstraint> for NumberTuple<TGenericList> {
}


#[fundamental]
pub trait IntoHelper<T>: Sized {
    #[must_use]
    fn into2(self) -> T;
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

struct SingleTypeConstraint<T>(T);
impl<T> TypeConstraint for SingleTypeConstraint<T>{
    type Type<U: TypeConstraintImpl<Self>> = T;
}
impl<T> TypeConstraintImpl<SingleTypeConstraint<T>> for T{
}

enum ToStringFn{}

impl ToStringFn {
    fn call<T: NumberTypeConstraintImpl>(value: T) -> String {
        value.to_string()
    }
}

impl FnMetaTrait for ToStringFn {
    type TMap = Self;
    type TConFrom = NumberTypeConstraint;
}
impl<T: TypeConstraintImpl<NumberTypeConstraint>> FnMap2<T> for ToStringFn{
    default type Next<U: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>> = Self;

    default fn call(value: T) -> <<Self as FnMetaTrait>::TMap as TypeMap<<Self as FnMetaTrait>::TConFrom>>::TMap<T> {
        unreachable!()
    }
}
impl<T: NumberTypeConstraintImpl> FnMap2<T> for ToStringFn{
    type Next<U: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>> = Self;

    fn call(value: T) -> <<Self as FnMetaTrait>::TMap as TypeMap<<Self as FnMetaTrait>::TConFrom>>::TMap<T> {
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
    let list = list.map2::<ToStringFn>();
    let (a, b, c, d, e, f, g, h, i, j) = list.into();
    println!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", a, b, c, d, e, f, g, h, i, j);
}
