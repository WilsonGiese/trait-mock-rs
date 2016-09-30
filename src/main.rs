
struct Cursor;
struct MockCursor;

trait CursorType {
    fn new() -> Self;
    fn do_something(&self);
}

impl CursorType for Cursor {
    fn new() -> Cursor {
        Cursor { }
    }

    fn do_something(&self) {
        panic!()
    }
}

impl CursorType for MockCursor {
    fn new() -> MockCursor {
        MockCursor { }
    }

    fn do_something(&self) {
        println!("It's rude to mock");
    }
}

struct Owner<Cursor: CursorType> {
    cursor: Cursor,
}

impl<C: CursorType> Owner<C> {
    pub fn new(cursor: C) -> Owner<C>  {
        Owner {
            cursor: cursor,
        }
    }

    pub fn do_something(&self) {
        println!("Work");
        println!("Work");
        println!("Work");
        self.cursor.do_something();
        println!("Work");
        println!("Work");
        println!("Work");
    }
}

fn main() {
    let cursor = Cursor::new();
    let owner = Owner::new(cursor);
    owner.do_something();
}

#[test]
fn test_mock() {
    let cursor = MockCursor::new();
    let owner = Owner::new(cursor);
    owner.do_something();
}

#[test]
#[should_panic]
fn test_real() {
    let cursor = Cursor::new();
    let owner = Owner::new(cursor);
    owner.do_something();
}
