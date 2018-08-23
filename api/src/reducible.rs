use num_rational::BigRational;

pub(crate) trait Reducible<Value=BigRational> {
    /// Calculates `value` count of `self` in terms of `self`'s base-unit.
    ///
    fn reduce_value(&self, value: Value) -> BigRational;

    fn calculate_magnitude(&self, value: Value) -> BigRational;
}
