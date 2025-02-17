use arrow2::{
    array::{Array, Int32Array},
    chunk::Chunk,
    error::Result,
    io::csv::write,
};

fn write_batch<A: AsRef<dyn Array>>(path: &str, columns: &[Chunk<A>]) -> Result<()> {
    let writer = &mut write::WriterBuilder::new().from_path(path)?;

    write::write_header(writer, &["c1"])?;

    let options = write::SerializeOptions::default();
    columns
        .iter()
        .try_for_each(|batch| write::write_chunk(writer, batch, &options))
}

fn main() -> Result<()> {
    let array = Int32Array::from(&[
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
    ]);
    let batch = Chunk::try_new(vec![&array as &dyn Array])?;

    write_batch("example.csv", &[batch])
}
