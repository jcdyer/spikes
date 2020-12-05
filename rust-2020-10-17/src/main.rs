use core::cell::RefCell;

struct MultOwner<'rc> {
    refcell: &'rc RefCell<u8>,
    offset: u8,
}

impl MultOwner<'_> {
    fn add(&self) {
        *self.refcell.borrow_mut() += 1;
    }

    fn result(&self) -> u8 {
        *self.refcell.borrow() + self.offset
    }
}

fn main() {

    dbg!(core::mem::size_of::<MultOwner>());
    let refcell = RefCell::new(0);

    let zero = MultOwner { refcell: &refcell, offset: 0 };
    let one = MultOwner { refcell: &refcell, offset: 1 };
    let two = MultOwner { refcell: &refcell, offset: 2 };

    zero.add();
    one.add();
    dbg!(two.result(), "should equal 4");
    dbg!(zero.result(), "should equal 2");
}
