// vid 15

/**
 * In std,
 * 
 * unsafe impl<#[may_dangle] T: ?Sized> Drop for Box<T> {
 *   fn drop(&mut self) {
 *    // borrow from CPP RAII
 *   }
 * }
 * 
 * pub trait Deref{
 *  type Target: ?Sized;
 *  fn deref(&self) -> &Self::Target;
 * }
 */
fn main(){
    let x: Box<i32> = Box::new(42);
    let y = *x;
    assert_eq!(y, 42);
}

