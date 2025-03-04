use std::collections::HashMap;

#[derive(Default)]
pub struct Letterbox<const N: usize> {
    /// Mapping from (process id, function id) to energy/runtime values.
    letterboxes: HashMap<(i32, i32), Samples<N>>,
}

struct Samples<const N: usize> {
    elems: [i32; N],
    len: usize,
}

impl<const N: usize> Letterbox<N> {
    pub fn update(&mut self, pid: i32, fid: i32, value: i32) -> Option<[i32; N]> {
        if let Some(samples) = self.letterboxes.get_mut(&(pid, fid)) {
            samples.push(value);

            if samples.len >= N {
                Some(samples.clear())
            } else {
                None
            }
        } else {
            self.letterboxes.insert((pid, fid), Samples::from(value));
            None
        }
    }
}

impl<const N: usize> Samples<N> {
    fn clear(&mut self) -> [i32; N] {
        let res = self.elems;
        self.elems = [0; N];
        res
    }

    fn push(&mut self, value: i32) {
        assert!(self.len < N);
        self.elems[self.len] = value;
        self.len += 1;
    }
}

impl<const N: usize> From<i32> for Samples<N> {
    fn from(value: i32) -> Self {
        let mut elems = [0; N];
        elems[0] = value;
        Self { elems, len: 1 }
    }
}
