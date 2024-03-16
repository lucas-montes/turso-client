use libsql::{Row, Rows};

pub struct RowsIter<'a> {
    rows: &'a mut Rows,
}

impl<'a> RowsIter<'a> {
    pub fn new(rows: &'a mut Rows) -> Self {
        Self { rows }
    }
}

impl<'a> Iterator for RowsIter<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        match self.rows.next() {
            Ok(row) => row,
            Err(err) => panic!("some error in the iterator getting the next row: {err:?}"),
        }
    }
}
