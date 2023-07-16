use crate::Benchmarkable;

pub struct SerialTable;

impl Benchmarkable for SerialTable {
    fn do_eager() {
        print!("THIS IS SOME VERY LONG LINE.THIS IS SOME VERY LONG LINE.THIS IS SOME VERY LONG LINE.THIS IS SOME VERY LONG LINE.");
    }

    fn do_lazy() {
        print!("THIS IS SOME VERY LONG LINE.");
        print!("THIS IS SOME VERY LONG LINE.");
        print!("THIS IS SOME VERY LONG LINE.");
        print!("THIS IS SOME VERY LONG LINE.");
    }
}
