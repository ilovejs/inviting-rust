## std iterator

intolter
iter
iterMut

iterator adapter

iterator consumer

application in string and slice

## forms

iter -> &T

into_iter -> T
    convert into an iterator
    e.g. v.into_iter(), 把v所有权消耗掉。v就不能用了 
    不想消耗所有权的话，可以用Vec，因为实现了FromIterator, IntoIterator
    slice 实现了Iterator, IntoIterator,所以可以for循环使用

iter_mut() -> &mut T


## doc

https://doc.rust-lang.org/std/iter/index.html
https://doc.rust-lang.org/std/iter/trait.Iterator.html

https://doc.rust-lang.org/std/iter/index.html#adapters

iter is lazy until compute trigger by collect() lastly.

[for each](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)

[fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

since it's lazy, we can have infinite iterator. e.g. let n = 0..;

n.take(3)

## structs

[structs](https://doc.rust-lang.org/std/iter/index.html#structs)

所有权的复制和拷贝
Cloned	An iterator that clones the elements of an underlying iterator. 显示
Copied	An iterator that copies the elements of an underlying iterator. 隐示
但是，适配器里，都是显示调用

clone is deep copy
copy is shallow copy

string
vec
hashmnap

## traits

DoubleEndedIterator

FromIterator

Extend
