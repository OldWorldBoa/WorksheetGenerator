use crate::docx_writer::block_expression_writer;
use crate::math::math_worksheet_generator;

pub mod docx_writer;
pub mod image_reader;
pub mod math;

fn main() {
    block_expression_writer::write_expressions(math_worksheet_generator::generate_questions())
        .unwrap();
}
