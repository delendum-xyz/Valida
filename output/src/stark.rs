use crate::columns::{OutputCols, NUM_OUTPUT_COLS};
use crate::OutputChip;
use core::borrow::Borrow;
use valida_opcodes::WRITE;

use p3_air::{Air, AirBuilder, BaseAir};
use p3_field::{AbstractField, PrimeField};
use p3_matrix::MatrixRowSlices;

impl<F: AbstractField> BaseAir<F> for OutputChip {
    fn width(&self) -> usize {
        NUM_OUTPUT_COLS
    }
}

impl<F, AB> Air<AB> for OutputChip
where
    F: PrimeField,
    AB: AirBuilder<F = F>,
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local: &OutputCols<AB::Var> = main.row_slice(0).borrow();
        let next: &OutputCols<AB::Var> = main.row_slice(1).borrow();

        // Range check constraints
        builder
            .when_transition()
            .assert_eq(local.diff, next.clk - local.clk);
        builder
            .when_transition()
            .assert_eq(next.counter, local.counter + AB::Expr::from(AB::F::one()));

        // Bus opcode constraint
        builder.when(local.is_real).assert_eq(
            local.opcode,
            AB::Expr::from(AB::F::from_canonical_u32(WRITE)),
        );
    }
}
