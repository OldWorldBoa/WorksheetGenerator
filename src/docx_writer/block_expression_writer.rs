use crate::{image_reader::block_image_reader, math::expression::Expression};
use docx_rs::{
    CharacterSpacing, Docx, DocxError, PageMargin, Paragraph, Run, RunFonts, Table, TableCell,
    TableCellBorder, TableCellBorders, TableRow,
};

pub fn write_expressions(expressions: Vec<Expression>) -> Result<(), DocxError> {
    let mut doc = Docx::new().page_margin(PageMargin::new().top(1500).left(1000).right(1000));

    for expr in expressions {
        let mut term1run = Run::new();
        for _ in 0..expr.term1 / 10 {
            let t1_ten_pic = block_image_reader::get_block_image(10);
            term1run = term1run.add_image(t1_ten_pic);
        }
        let t1_ones_pic = block_image_reader::get_block_image(expr.term1 % 10);
        term1run = term1run.add_text(" ").add_image(t1_ones_pic);

        let mut term2run = Run::new();
        for _ in 0..expr.term2 / 10 {
            let t2_ten_pic = block_image_reader::get_block_image(10);
            term2run = term2run.add_image(t2_ten_pic);
        }
        let t2_ones_pic = block_image_reader::get_block_image(expr.term2 % 10);
        term2run = term2run.add_text(" ").style("").add_image(t2_ones_pic);

        let font_size = 40;
        doc = doc.add_table(
            Table::without_borders(vec![TableRow::new(vec![
                TableCell::new()
                    .vertical_align(docx_rs::VAlignType::Center)
                    .add_paragraph(
                        Paragraph::new()
                            .add_run(term1run)
                            .add_run(Run::new().size(150).add_text(" - "))
                            .add_run(term2run),
                    ),
                TableCell::new()
                    .vertical_align(docx_rs::VAlignType::Center)
                    .add_table(Table::without_borders(vec![
                        TableRow::new(vec![
                            TableCell::new(),
                            TableCell::new()
                                .set_border(TableCellBorder::new(
                                    docx_rs::TableCellBorderPosition::Tl2br,
                                ))
                                .add_paragraph(
                                    Paragraph::new()
                                        .align(docx_rs::AlignmentType::Center)
                                        .add_run(
                                            Run::new()
                                                .size(font_size)
                                                .add_text((expr.term1 / 10).to_string()),
                                        ),
                                ),
                            TableCell::new()
                                .set_border(TableCellBorder::new(
                                    docx_rs::TableCellBorderPosition::Tl2br,
                                ))
                                .add_paragraph(
                                    Paragraph::new()
                                        .align(docx_rs::AlignmentType::Center)
                                        .add_run(
                                            Run::new()
                                                .size(font_size)
                                                .add_text((expr.term1 % 10).to_string()),
                                        ),
                                ),
                        ]),
                        TableRow::new(vec![
                            TableCell::new()
                                .vertical_align(docx_rs::VAlignType::Center)
                                .add_paragraph(
                                    Paragraph::new()
                                        .align(docx_rs::AlignmentType::Center)
                                        .add_run(Run::new().size(font_size).add_text("-")),
                                ),
                            TableCell::new()
                                .set_border(TableCellBorder::new(
                                    docx_rs::TableCellBorderPosition::Tl2br,
                                ))
                                .vertical_align(docx_rs::VAlignType::Center)
                                .add_paragraph(
                                    Paragraph::new()
                                        .align(docx_rs::AlignmentType::Center)
                                        .add_run(
                                            Run::new()
                                                .size(font_size)
                                                .add_text((expr.term2 / 10).to_string()),
                                        ),
                                ),
                            TableCell::new()
                                .set_border(TableCellBorder::new(
                                    docx_rs::TableCellBorderPosition::Tl2br,
                                ))
                                .vertical_align(docx_rs::VAlignType::Center)
                                .add_paragraph(
                                    Paragraph::new()
                                        .align(docx_rs::AlignmentType::Center)
                                        .add_run(
                                            Run::new()
                                                .size(font_size)
                                                .add_text((expr.term2 % 10).to_string()),
                                        ),
                                ),
                        ]),
                        TableRow::new(vec![
                            TableCell::new().clear_all_border().set_border(
                                TableCellBorder::new(docx_rs::TableCellBorderPosition::Top)
                                    .size(15),
                            ),
                            TableCell::new()
                                .set_borders(TableCellBorders::new())
                                .set_border(
                                    TableCellBorder::new(docx_rs::TableCellBorderPosition::Top)
                                        .size(15),
                                )
                                .add_paragraph(Paragraph::new().size(font_size)),
                            TableCell::new()
                                .set_borders(TableCellBorders::new())
                                .set_border(
                                    TableCellBorder::new(docx_rs::TableCellBorderPosition::Top)
                                        .size(15),
                                )
                                .add_paragraph(Paragraph::new().size(font_size)),
                        ]),
                    ])),
            ])])
            .align(docx_rs::TableAlignmentType::Center)
            .set_grid(vec![8200, 1500]),
        );
    }

    let path = std::path::Path::new("./out/demo.docx");
    let file = std::fs::File::create(path).unwrap();
    doc.build().pack(file)?;

    Ok(())
}
