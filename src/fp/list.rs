pub type List<T> = Vec<T>;

pub fn new<T>(v: T) -> List<T> {
    return push(empty(), v);
}

pub fn empty<T>() -> List<T> {
    return vec![];
}

pub fn new_from_slice<T: Clone>(s: &[T]) -> List<T> {
	let mut result: Vec<T> = vec![];
	result.extend_from_slice(s);

	return result;
}

pub fn push<T>(mut l: List<T>, v: T) -> List<T> {
    l.push(v);
    
    return l;
}

pub fn concat<T>(mut init: List<T>, mut tail: List<T>) -> List<T> {
    init.append(&mut tail);
    
    return init;
}