pub mod list;
pub mod comprehension;

pub fn head_or_default<T: Clone>(list: &list::List<T>, default: T) -> T {
	return unwrap_or_default(
		head(&list), 
		default
	);
}

pub fn last_or_default<T: Clone>(list: &list::List<T>, default: T) -> T {
	return unwrap_or_default(
		last(&list), 
		default
	);
}

pub fn head<T: Clone>(l: &list::List<T>) -> Option<T> {
	if l.is_empty() {
		return None
	}
	let first_index = 0usize;
	return Some(l[first_index].clone());
}

pub fn last<T: Clone>(l: &list::List<T>) -> Option<T> {
	if l.is_empty() {
		return None;
	}
	let last_index = (l.len()-1) as usize;
	return Some(l[last_index].clone());
}

pub fn init<T: Clone>(l: &list::List<T>) -> list::List<T> {
	if l.is_empty() {
		return vec![];
	}

	let mut result: list::List<T> = vec![];
	result.extend_from_slice(&l[0..l.len()-1]);
	return result;
}

pub fn tail<T: Clone>(l: &list::List<T>) -> list::List<T> {
	if l.is_empty() {
		return vec![];
	}

	let mut result: list::List<T> = vec![];
	result.extend_from_slice(&l[1..]);
	return result;
}

pub fn body<T: Clone>(l: &list::List<T>) -> list::List<T> {
	if l.is_empty() {
		return vec![];
	}

	return init(&tail(&l));
}

pub fn unwrap_or_default<T>(wrap: Option<T>, default: T) -> T {
	return {
		if let None = wrap {
			default
		} else {
			wrap.unwrap()
		}
	};
}