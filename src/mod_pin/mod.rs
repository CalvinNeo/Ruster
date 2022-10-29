use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::marker::PhantomPinned;
use std::ops::{Deref, DerefMut};
use std::borrow::BorrowMut;
use pin_cell;
use pin_cell::{PinCell, PinMut};

#[derive(Default, Debug)]
struct TestUnpin {
    a: String,
}
#[derive(Default, Debug)]
struct TestNUnpin {
    b: String,
}

impl !Unpin for TestNUnpin {}

#[derive(Default, Debug)]
struct EvilNUnpin {
    b: String,
}
impl !Unpin for EvilNUnpin {}

impl Deref for EvilNUnpin {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.b
    }
}

impl DerefMut for EvilNUnpin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.b = "3".to_owned();
        &mut self.b
    }
}

struct SimpleNUnPin {
    a: u64,
}

impl !Unpin for SimpleNUnPin {}

fn test_new_unchecked() {
    let mut x1 = TestNUnpin{ b: "1".to_owned() };
    let mut x2 = TestNUnpin{ b: "2".to_owned() };
    let ptr1 = &x1 as *const _ as isize;
    let ptr2 = &x2 as *const _ as isize;
    unsafe {
        let _pin1 = Pin::new_unchecked(&x1);
        let _pin2 = Pin::new_unchecked(&x1);
    }
    std::mem::swap(&mut x1, &mut x2);
    unsafe {
        let n1 = &*(ptr1 as *const TestNUnpin);
        let n2 = &*(ptr2 as *const TestNUnpin);
        assert_eq!(n1.b, "2");
        assert_eq!(n2.b, "1");
    }
}

fn test_box_pin() {
    let mut x1 = TestNUnpin{ b: "1".to_owned() };
    let mut x2 = TestNUnpin{ b: "2".to_owned() };
    let ptr1 = &x1 as *const _ as isize;
    let ptr2 = &x2 as *const _ as isize;

    let mut bx1 = Box::pin(x1);
    let mut bx2 = Box::pin(x2);
    std::mem::swap(&mut bx1, &mut bx2);

    unsafe {
        let n1 = &*(ptr1 as *const TestNUnpin);
        let n2 = &*(ptr2 as *const TestNUnpin);
        assert_eq!(n1.b, "1");
        assert_eq!(n2.b, "2");
    }
}

fn test_move_rc() {
    let mut x = Rc::new(TestNUnpin{ b: "1".to_owned() });
    let pinned = unsafe { Pin::new_unchecked(Rc::clone(&x)) };
    {
        let p = pinned.as_ref();
    }
    drop(pinned);
    // We can get &mut T now.
    assert!(Rc::get_mut(&mut x).is_some());
}

fn test_get_mut_from_pin() {
    {
        let mut p = TestUnpin { a: "a".to_owned() };
        let mut p2 = TestUnpin { a: "b".to_owned() };
        let mut rp = unsafe {
            Pin::new(&mut p)
        };
        let mut rp2 = unsafe {
            Pin::new(&mut p2)
        };
        std::mem::swap(Pin::get_mut(rp), Pin::get_mut(rp2));
        println!("{} {}", p.a, p2.a);
        assert_eq!(p.a, "b");
        assert_eq!(p2.a, "a");
    }
}

macro_rules! pin_mut2 {
    ($($x:ident),* $(,)?) => { $(
        // Shadow the original binding so that it can't be directly accessed
        // ever again.
        #[allow(unused_mut)]
        let mut $x = unsafe {
            Pin::new_unchecked(&mut $x)
        };
    )* }
}

fn test_pin_util() {
    {
        let mut x = TestNUnpin { b: "b".to_owned() };
        pin_utils::pin_mut!(x);
    }
    {
        let mut x = TestNUnpin { b: "b".to_owned() };
        // no rules expected this token in macro call
        // pin_utils::pin_mut!(&mut x);
    }
    {
        let mut x = TestNUnpin { b: "b".to_owned() };
        let y = &x;
        pin_utils::pin_mut!(y);
    }
    {
        let mut x = TestNUnpin { b: "b".to_owned() };
        let y = x;
        // value used here after move
        // pin_utils::pin_mut!(x);
        // pin_mut2!(x);
    }
    {
        let mut x1 = EvilNUnpin { b: "1".to_owned() };
        let mut x2 = EvilNUnpin { b: "2".to_owned() };
        let ptr1 = &x1 as *const _ as isize;
        let ptr2 = &x2 as *const _ as isize;
        let mut xp = unsafe { Pin::new_unchecked(x1) };
        let mut xp2 = unsafe { Pin::new_unchecked(x2) };
        std::mem::swap(&mut xp.as_mut(), &mut xp2.as_mut());
        unsafe {
            let n1 = &*(ptr1 as *const TestNUnpin);
            let n2 = &*(ptr2 as *const TestNUnpin);
            println!("{:?} {:?}", n1, n2);
        }
    }
}

fn test_shadow() {
    let mut x = TestNUnpin { b: "b".to_owned() };
    let mut xp = unsafe { Pin::new_unchecked(&mut x) };
    drop(xp);
    assert_eq!(x.b, "b");
    let mut x2 = TestNUnpin { b: "b2".to_owned() };
    std::mem::swap(&mut x, &mut x2);
    assert_eq!(x.b, "b2");
}

fn test_mutability() {
    {
        let x = SimpleNUnPin { a: 1 };
        pin_utils::pin_mut!(x);
        x.as_mut().a = 2;
    }
    {
        let mut x1 = TestNUnpin{ b: "1".to_owned() };
        let mut x2 = TestNUnpin{ b: "2".to_owned() };
        let ptr1 = &x1 as *const _ as isize;
        let ptr2 = &x2 as *const _ as isize;

        let x1 = RefCell::new(x1);
        let x2 = RefCell::new(x2);
        pin_utils::pin_mut!(x1);
        pin_utils::pin_mut!(x2);
        std::mem::swap(&mut x1, &mut x2);

        unsafe {
            let n1 = &*(ptr1 as *const TestNUnpin);
            let n2 = &*(ptr2 as *const TestNUnpin);
            assert_eq!(n1.b, "1");
            assert_eq!(n2.b, "2");
        }
    }
    {
        let x = SimpleNUnPin { a: 1 };
        let ptr1 = &x as *const _ as isize;
        let xp = PinCell::new(x);
        let mut b = xp.borrow_mut();
        b.a = 2;
        unsafe {
            let n1 = &*(ptr1 as *const SimpleNUnPin);
            assert_eq!(n1.a, 2);
        }
    }
}

pub fn run() {
    test_new_unchecked();
    test_box_pin();
    test_move_rc();
    test_get_mut_from_pin();
    test_pin_util();
    test_shadow();
    test_mutability();

    let rp = Pin::new(&mut TestUnpin::default());
    // let rnp = Pin::new(&mut TestNUnpin::default());
    // let rnp2 = Pin::new(&TestUnpin::default()); // error[E0277]: `PhantomPinned` cannot be unpinned
    let rnb = Box::pin(TestNUnpin::default());
}