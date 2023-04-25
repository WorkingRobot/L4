pub fn make_heap<T: PartialOrd>(slice: &mut [T]) {
    for i in (0..=((slice.len() - 1) / 2)).rev() {
        bubble_down(slice, i);
    }
}

pub fn peek_heap<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.first()
}

pub fn pop_heap<T: PartialOrd>(slice: &mut [T]) {
    let n = slice.len();
    if n > 1 {
        slice.swap(0, n - 1);
        bubble_down(&mut slice[0..n - 1], 0);
    }
}

pub fn push_heap<T: PartialOrd>(slice: &mut [T]) {
    bubble_up(slice);
}

pub fn is_heap<T: PartialOrd>(slice: &[T]) -> bool {
    let n = slice.len();
    for i in 0..=(n - 2) / 2 {
        if slice[2 * i + 1] > slice[i] {
            return false;
        }

        if 2 * i + 2 < n && slice[2 * i + 2] > slice[i] {
            return false;
        }
    }
    true
}

pub fn sort_heap<T: PartialOrd>(mut slice: &mut [T]) {
    let mut n = slice.len();
    while n != 0 {
        pop_heap(slice);
        slice = &mut slice[..n - 1];
        n -= 1;
    }
}

fn bubble_up<T: PartialOrd>(slice: &mut [T]) {
    let n = slice.len();
    let mut i = n;
    let mut p = i / 2;
    while p > 0 && slice[i - 1] > slice[p - 1] {
        slice.swap(p - 1, i - 1);
        i = p;
        p = i / 2;
    }
}

fn bubble_down<T: PartialOrd>(slice: &mut [T], index: usize) {
    let n = slice.len();
    let mut i = index;
    let mut l = i * 2 + 1;
    let mut r = i * 2 + 2;

    if r < n && slice[r] > slice[l] {
        l = r;
    }
    // invariant: slice[l] >= slice[r]
    // if slice[l] > slice[i], do push
    while l < n && slice[l] > slice[i] {
        slice.swap(i, l);
        i = l;
        l = i * 2 + 1;
        r = i * 2 + 2;
        if r < n && slice[r] > slice[l] {
            l = r;
        }
    }
}
