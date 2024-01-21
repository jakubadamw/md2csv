use std::io::Read;

use anyhow::Result;

fn parse_markdown_table(input: &str) -> Result<Vec<Vec<String>>> {
    use markdown::mdast::*;

    let parsed = markdown::to_mdast(input, &markdown::ParseOptions::gfm())
        .map_err(|err| anyhow::format_err!(err))?;
    let table = match parsed {
        Node::Root(Root {
            ref children,
            position: _,
        }) => match children.as_slice() {
            [Node::Table(table)] => Some(table.clone()),
            _ => None,
        },
        _ => None,
    }
    .ok_or_else(|| anyhow::format_err!("input isn't a table: {parsed:#?}"))?;

    table
        .children
        .into_iter()
        .map(|node| match node {
            Node::TableRow(row) => Ok(row
                .children
                .into_iter()
                .map(|node| match node {
                    Node::TableCell(_) => Ok(node.to_string()),
                    other => anyhow::bail!("input isn't a cell: {other:#?}"),
                })
                .collect::<anyhow::Result<Vec<_>>>()?),
            other => anyhow::bail!("input isn't a row: {other:#?}"),
        })
        .collect()
}

fn write_table_as_csv(table: Vec<Vec<String>>) -> anyhow::Result<()> {
    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(std::io::stdout());
    for row in table {
        for cell in row {
            csv_writer.write_field(cell)?;
        }
        csv_writer.write_record(None::<&[u8]>)?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let table = parse_markdown_table(&input)?;
    write_table_as_csv(table)
}
