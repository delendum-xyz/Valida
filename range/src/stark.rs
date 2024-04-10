use crate::RangeCheckerChip;

use crate::columns::NUM_RANGE_COLS;
use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix;

impl<AB, const MAX: u32> Air<AB> for RangeCheckerChip<MAX>
where
    AB: AirBuilder,
{
    fn eval(&self, _builder: &mut AB) {
        // TODO
    }
}

impl<F: Field, const MAX: u32> BaseAir<F> for RangeCheckerChip<MAX> {
    fn width(&self) -> usize {
        NUM_RANGE_COLS
    }

    fn preprocessed_trace(&self) -> RowMajorMatrix<F> {
        let column = (0..MAX).map(F::from_canonical_u32).collect();
        RowMajorMatrix::new_col(column)
    }
}
