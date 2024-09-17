trait Sealed<T>: Sized{}
#[allow(private_bounds)]
pub trait Same<T1, T2>: Sealed<T1> + Sealed<T2>{}
pub trait SameSelf<TOther>: Same<TOther, Self> + Same<Self, TOther>{
    fn into_other_self(self) -> TOther;
}
impl<T: Same<T, T>> Sealed<T> for T{}
impl<T: Same<T, T>> SameSelf<T> for T{
    fn into_other_self(self) -> T {
        self
    }
}
impl<T> Same<T, T> for T{}
