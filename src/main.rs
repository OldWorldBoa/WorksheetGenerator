use crate::docx_writer::docx_expression_writer;
use crate::math::math_worksheet_generator;

pub mod docx_writer;
pub mod math;

fn main() {
    docx_expression_writer::write_expressions(math_worksheet_generator::generate_questions())
        .unwrap();
}
