use std::ops::Deref;

#[derive(Debug)]
struct MySmartPointer<T>(T);

impl<T> MySmartPointer<T> {
    fn new(data: T) -> MySmartPointer<T> {
        MySmartPointer(data)
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main(){
    let x = 5;
    let y = MySmartPointer::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}