#[macro_use]
mod helper_marco;

use std::marker::Tuple;
use crate::generic::optional_type::{OptionalType, OptionalTypeMarker};

pub trait TypeConstraint : Sized {
    type Type<T: TypeConstraintImpl<Self>>: Sized + TypeConstraintImpl<Self>;
}

pub trait TypeMap<TConFrom: TypeConstraint> {
    type TConTo: TypeConstraint;
    type TMap<T: TypeConstraintImpl<TConFrom>>: TypeConstraintImpl<Self::TConTo>;
}

pub trait FnMetaTrait {
    type TMap: TypeMap<Self::TConFrom>;
    type TConFrom: TypeConstraint;
}

pub trait FnMap<T: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>> : FnMetaTrait {
    type Next<U: TypeConstraintImpl<<Self as FnMetaTrait>::TConFrom>>: FnMap<U, TConFrom=<Self as FnMetaTrait>::TConFrom, TMap=<Self as FnMetaTrait>::TMap>;
    fn call(value: T) -> <<Self as FnMetaTrait>::TMap as TypeMap<<Self as FnMetaTrait>::TConFrom>>::TMap<T>;
}

trait TypeConstraintImplSealed<TCon: TypeConstraint>{}
impl<T: TypeConstraintImpl<TCon>, TCon: TypeConstraint<Type<T>= T>> TypeConstraintImplSealed<TCon> for T{}

#[allow(private_bounds)]
#[fundamental]
pub trait TypeConstraintImpl<TCon: TypeConstraint>: TypeConstraintImplSealed<TCon> + Sized{
}

/*impl<T: TypeConstraintImpl<TCon>, TCon: TypeConstraint<Type<T>= T>> TypeConstraintImplIntern<TCon> for T{}

#[fundamental]
pub trait TypeConstraintImpl<TCon: TypeConstraint<Type<Self>= Self>>: TypeConstraintImplIntern<TCon> {    
}*/

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

    #[doc(hidden)]
    #[allow(private_interfaces)]
    fn _create<Map: TypeMap<TCon>>(value: Map::TMap<Self::InnerType>, next: OptionalNodeMapped<Self, TCon, Map>, _: Sealed) -> Self::LinkType<Map>;
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

impl<T, TCon> GenericLinkedListFamily for GenericListEnd<T, TCon>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint
{
    type GenericLinkedListTypeConstraint = TCon;
    type GenericLinkedListSelfType = Self;
}

impl<T, TCon> GenericLinkedListSeal for GenericListEnd<T, TCon>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint
{}

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

    #[allow(private_interfaces)]
    fn _create<Map: TypeMap<TCon>>(value: Map::TMap<Self::InnerType>, next: OptionalNodeMapped<Self, TCon, Map>, _: Sealed) -> Self::LinkType<Map> {
        use optional_type::OptionalType::*;
        match next {
            NoType(_a) => {}
            Type(_, never) => {never}
        }
        GenericListEnd {
            value,
            phantom: std::marker::PhantomData::<Map::TConTo>,
        }
    }

}

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

impl<T, TCon, T2> GenericLinkedListFamily for GenericListLink<T, TCon, T2>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    T2: GenericLinkedList<TCon>
{
    type GenericLinkedListTypeConstraint = TCon;
    type GenericLinkedListSelfType = Self;
}

impl<T, TCon, T2> GenericLinkedListSeal for GenericListLink<T, TCon, T2>
where
    <TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
    T: TypeConstraintImpl<TCon>,
    TCon: TypeConstraint,
    T2: GenericLinkedList<TCon>
{}

impl<T: TypeConstraintImpl<TCon>, TCon: TypeConstraint, T2: GenericLinkedList<TCon>> GenericLinkedList<TCon> for GenericListLink<T, TCon, T2> {
    type InnerType = T;
    type LinkType<Map: TypeMap<TCon>> = GenericListLink<Map::TMap<Self::InnerType>, Map::TConTo, T2::LinkType<Map>>;
    type NextType = T2;
    type THasNext = ();
    const LENGTH: usize = T2::LENGTH + 1;

    #[allow(private_interfaces)]
    fn _create<Map: TypeMap<TCon>>(value: Map::TMap<Self::InnerType>, next: OptionalNodeMapped<Self, TCon, Map>, _: Sealed) -> Self::LinkType<Map> {
        use optional_type::OptionalType::*;
        use std::marker::PhantomData;
        let next = match next {
            NoType(_a) => {unreachable!()}
            Type(v, _) => v
        };
        GenericListLink {
            value,
            next,
            phantom: PhantomData,
        }
    }
}

pub mod optional_type{

    trait Sealed<>{}


    #[allow(private_bounds)]
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
            use crate::generic::optional_type::OptionalType::NoType;
            NoType(())
        }
    }

    impl Sealed for () {}

    impl OptionalTypeMarker for (){
        type Opposite = !;
        type Existential = ();
        type TArg<T> = T;

        fn new<T>(value: T) -> OptionalType<Self, T> {
            use crate::generic::optional_type::OptionalType::Type;
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
    fn map<F>(self) -> <Self as GenericLinkedList<TCon>>::LinkType<F::TMap>
    where
        F: FnMap<T, TConFrom=TCon>
    {
        let (value, next) = self.deconstruct();
        let value = F::call(value);
        let next = next.map(|next| next.map::<F::Next<_>>());
        Self::_create(value, next, Sealed {})


    }
}

impl<TLink, T, TCon> GenericListValue<T, TCon> for TLink
where TLink: GenericLinkedList<TCon, InnerType=T> + GenericListValueBase<T, TCon=TCon, TNext=Self::NextType>,
<TCon as TypeConstraint>::Type<T>: TypeConstraintImpl<TCon>,
TCon: TypeConstraint,
T: TypeConstraintImpl<TCon>, {}

#[allow(private_bounds)]
pub trait GenericListValueBase<T> : Sized + GenericLinkedListSeal + GenericLinkedListFamily<GenericLinkedListSelfType=Self, GenericLinkedListTypeConstraint=Self::TCon>
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

trait GenericLinkedListSeal{}

#[allow(private_bounds)]
pub trait GenericLinkedListFamily: GenericLinkedListSeal{
    type GenericLinkedListTypeConstraint: TypeConstraint;
    type GenericLinkedListSelfType: GenericLinkedList<Self::GenericLinkedListTypeConstraint>;
}


trait FromTupleSeal<Tuple: std::marker::Tuple + Sized>{}
//this seals! and enforces that the parameter List actually is not free, but that a bijection exists between List and Tuple
impl<Tuple: TupleIntoList<List::GenericLinkedListTypeConstraint, List=Self>, List: GenericLinkedListFamily> FromTupleSeal<Tuple> for List{}
//impl<TFromTuple: FromTuple<Tuple>, Tuple: TupleIntoList<TFromTuple::TCon, List=Self>> FromTupleSeal<Tuple> for TFromTuple{}
#[allow(private_bounds)]
pub trait FromTuple<Tuple: TupleIntoList<Self::TCon>>: FromTupleSeal<Tuple>
{
    type TCon: TypeConstraint;

    #[inline(always)]
    extern "rust-call" fn from_tuple(tuple: Tuple) -> <Tuple as TupleIntoList<Self::TCon>>::List {
        Tuple::into_generic_list(tuple)
    }
}
impl<Tuple: TupleIntoList<List::GenericLinkedListTypeConstraint>, List: GenericLinkedListFamily + FromTupleSeal<Tuple>> FromTuple<Tuple> for List
{
    type TCon = List::GenericLinkedListTypeConstraint;
}

pub trait TupleIntoList<TCon: TypeConstraint>: Tuple + Sized
{
    type List: GenericLinkedList<TCon> + FromTuple<Self, TCon = TCon>;


    #[allow(clippy::wrong_self_convention)]
    extern "rust-call" fn into_generic_list(tuple: Self) -> Self::List;
}





pub trait GenericTuple<TGenericList: GenericLinkedList<TCon>, TCon: TypeConstraint> {
}




_impl_from_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
_impl_into_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);