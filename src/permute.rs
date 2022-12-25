struct PermuteSlice<'a, 'b, T> {
    order: &'b [usize],
    data: &'a [T],
}

struct Permute<'a, T> {
    orig: &'a [T],
    state: Vec<usize>,
}

impl<'a, T> Permute<'a, T> {
    pub fn new(data: &'a [T]) -> Permute<'a, T> {
        Permute {
            orig: data,
            state: (0..data.len()).collect(),
        }
    }

    // pub fn permute<'b>(&'b mut self) -> PermuteIterator<'a, 'b, T> {
    //     let Permute { orig, state } = self;
    //     PermuteIterator {
    //         orig,
    //         state: &mut state,
    //         stack: vec![0; orig.len()],
    //         depth: 0,
    //     }
    // }
}

struct PermuteIterator<'a, 'b, T> {
    orig: &'a [T],
    state: &'b mut [usize],
    stack: Vec<usize>,
    depth: usize,
}

impl<'a, 'b, T> PermuteIterator<'a, 'b, T> {
    fn emit(&'b self) -> PermuteSlice<'a, 'b, T> {
        PermuteSlice {
            order: self.state,
            data: self.orig,
        }
    }
}

// impl<'a, 'b, T> Iterator for PermuteIterator<'a, 'b, T> {
//     // type Item = PermuteSlice<'a, 'b, T>;
//     type Item = &'b [usize];

//     // fn next(&mut self) -> Option<PermuteSlice<'a, 'b, T>> {
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.depth == 0 {
//             return Some(self.state);
//         }
//         todo!()
//     }
// }
