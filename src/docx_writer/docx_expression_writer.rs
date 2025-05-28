use std::fs::File;
use std::io::Read;

use docx_rs::{Docx, DocxError, Paragraph, Pic, Run, Table, TableCell, TableRow};

use crate::math::expression::Expression;

pub fn write_expressions(expressions: Vec<Expression>) -> Result<(), DocxError> {
    let mut doc = Docx::new();

    for expr in expressions {
        let mut term1run = Run::new();
        for _ in 0..expr.term1 / 10 {
            term1run = term1run.add_image(get_image(10));
        }
        term1run = term1run.add_text(" ").add_image(get_image(expr.term1 % 10));

        let mut term2run = Run::new();
        for _ in 0..expr.term2 / 10 {
            term2run = term2run.add_image(get_image(10));
        }
        term2run = term2run.add_text(" ").add_image(get_image(expr.term2 % 10));

        doc = doc
            .add_table(
                Table::new(vec![TableRow::new(vec![
                    TableCell::new().add_paragraph(
                        Paragraph::new()
                            .add_run(term1run)
                            .add_run(Run::new().size(200).add_text("-"))
                            .add_run(term2run),
                    ),
                    TableCell::new().add_table(Table::new(vec![
                        TableRow::new(vec![
                            TableCell::new().add_paragraph(
                                Paragraph::new()
                                    .add_run(Run::new().add_text((expr.term1 / 10).to_string())),
                            ),
                            TableCell::new().add_paragraph(
                                Paragraph::new()
                                    .add_run(Run::new().add_text((expr.term1 % 10).to_string())),
                            ),
                        ]),
                        TableRow::new(vec![
                            TableCell::new().add_paragraph(
                                Paragraph::new()
                                    .add_run(Run::new().add_text((expr.term2 / 10).to_string())),
                            ),
                            TableCell::new().add_paragraph(
                                Paragraph::new()
                                    .add_run(Run::new().add_text((expr.term2 % 10).to_string())),
                            ),
                        ]),
                        TableRow::new(vec![TableCell::new(), TableCell::new()]),
                    ])),
                ])])
                .width(40 * 9525, docx_rs::WidthType::Auto),
            )
            .add_paragraph(Paragraph::new());
    }

    let path = std::path::Path::new("./out/demo.docx");
    let file = std::fs::File::create(path).unwrap();
    doc.build().pack(file)?;

    Ok(())
}

fn get_image(cube_amt: i32) -> Pic {
    let mut file_name = String::new();
    let height: i32;
    let width: i32;

    match cube_amt {
        10 => {
            file_name = String::from("./img/10 Rod.png");
            height = 135;
            width = 25
        }
        1..=5 => {
            file_name.push_str("./img/Cube ");
            file_name.push_str(&cube_amt.to_string());
            file_name.push_str(".png");

            height = 20 * cube_amt;
            width = 22;
        }
        6..=9 => {
            file_name.push_str("./img/Cube ");
            file_name.push_str(&cube_amt.to_string());
            file_name.push_str(".png");

            height = 100;
            width = 44;
        }
        _ => {
            panic!("No image for amount {cube_amt}");
        }
    }

    let mut file = File::open(file_name).unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf).unwrap();

    Pic::new(&buf).size((width * 9525) as u32, (height * 9525) as u32)
}
