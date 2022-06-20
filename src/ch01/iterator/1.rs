
/* just run in vscode */
fn main(){
    // for 循环的scope
    let v = vec![1, 2, 3, 4, 5];
    {
        let mut _iter = v.into_iter();
        loop {
            match _iter.next() {
                Some(x) => println!("{}", x),
                None => break,
            }
        }
    }

    // use own iterator
    let mut xs = vec![1, 2, 3];
    
    xs.each(|x| x*3);

    assert_eq!(vec![3, 6, 9], &xs[..3]);
    // https://doc.rust-lang.org/std/macro.assert_ne.html
    assert_ne!(vec![6, 9], &xs[..]);
}

trait InIter<T: Copy> {
    // fn next(&mut self) -> Option<T>;
    fn each<F: Fn(T) -> T>(&mut self, f: F);
}

impl<T: Copy> InIter<T> for Vec<T> {
    fn each<F: Fn(T) -> T>(&mut self, f: F) {
       let mut i = 0;
       while i < self.len() {
           self[i] = f(self[i]);
           i += 1;
       }
    }
}

