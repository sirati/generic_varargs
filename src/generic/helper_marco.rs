
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
            #[inline(always)]
            fn from(tuple: ($T,)) -> Self {
                <($T,)>::into_generic_list(tuple)
            }
        }
        #[allow(unused_parens)]
        impl<$T, TCon: TypeConstraint> TupleIntoList<TCon> for ($T,)
        where $T: TypeConstraintImpl<TCon> {
            type List = GenericListEnd<$T, TCon>;
            
            #[inline(always)]
            extern "rust-call" fn into_generic_list(tuple: Self) -> Self::List {
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
            #[inline(always)]
            fn from(tuple: ($T, $($Rest),+)) -> Self {
                <($T, $($Rest),+)>::into_generic_list(tuple)
            }
        }
        #[allow(unused_parens)]
        #[allow(non_snake_case)]
        impl<$T, $($Rest),+, TCon: TypeConstraint> TupleIntoList<TCon> for ($T, $($Rest),+)
        where $T: TypeConstraintImpl<TCon>, $($Rest: TypeConstraintImpl<TCon>),+ {
            type List = _impl_into_for_tuple_full_generic!($T, $($Rest),+);
            
            #[inline(always)]
            extern "rust-call" fn into_generic_list((head, $($Rest),+): Self) -> Self::List {
                //let (head, $($Rest),+) = tuple;
                GenericListLink {
                    value: head,
                    next: <<Self::List as GenericLinkedList<_>>::NextType as FromTuple<_>>::from_tuple(($($Rest),+,)),
                    phantom: std::marker::PhantomData::<TCon>,
                }
            }
        }
        _impl_from_for_tuple!($($Rest),+);
    };
}

macro_rules! _impl_into_for_tuple {
    // Base case for IntoInnerListEnd
    ($T:ident) => {
        #[allow(unused_parens, clippy::from_over_into)]
        impl<$T, TCon: TypeConstraint> Into<($T,)> for GenericListEnd<$T, TCon>
        where $T: TypeConstraintImpl<TCon> {
            fn into(self) -> ($T,) {
                (self.value,)
            }
        }
    };
    // Recursive case for IntoInnerListLink
    ($T:ident, $($Rest:ident),+) => {
        #[allow(non_snake_case, clippy::from_over_into)]
        impl<$T, $($Rest),+, TCon: TypeConstraint> Into<($T, $($Rest),+)> for _impl_into_for_tuple_full_generic!($T, $($Rest),+)
        where $T: TypeConstraintImpl<TCon>, $($Rest: TypeConstraintImpl<TCon>),+ {
            _impl_into_for_tuple_direct_repacking!(self, ( $T, $($Rest),+), $T, $($Rest),+; );
            /*fn into(self) -> ( $T, $($Rest),+) {
                _impl_into_for_tuple_direct_repacking!(self, $T, $($Rest),+; )
            }*/
        }
        _impl_into_for_tuple!($($Rest),+);
    };
}

macro_rules! _impl_into_for_tuple_direct_repacking {
    
    ($Self:ident, $ReturnTy:ty , $T:ident, $($Rest:ident),+; ) => {
        _impl_into_for_tuple_direct_repacking!($Self, $ReturnTy, ($Self.next), $($Rest),+; ($Self.value));
    };
    ($Self:ident, $ReturnTy:ty, $Prefix:tt,$T:ident, $($Rest:ident),+; $($Done:tt);+ ) => {
        _impl_into_for_tuple_direct_repacking!($Self, $ReturnTy, ($Prefix.next), $($Rest),+; $($Done);+;  ($Prefix.value));
    };
    ($Self:ident, $ReturnTy:ty, $Prefix:tt, $T:ident; $($Done:tt);+ ) => {
        fn into($Self) -> $ReturnTy {
            ($($Done),+, $Prefix.value, )
        }
    };
}


