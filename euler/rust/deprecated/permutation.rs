// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Heap's algorithms to generate all possible permutations of n objects.
//! See: https://en.wikipedia.org/wiki/Heap%27s_algorithm

#[derive(Clone, Debug)]
pub struct Permutation<T> {
    data: Vec<T>,
    swaps: Vec<usize>,
    index: usize,
}

impl<T: Clone> Permutation<T> {
    pub fn new(data: Vec<T>) -> Permutation<T> {
        let len = data.len();
        Permutation {
            data,
            swaps: vec![0; len],
            index: 0,
        }
    }
}

// TODO(Shaohua): Returns references only.
impl<T: Clone> Iterator for Permutation<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            loop {
                if self.index >= self.swaps.len() {
                    return None;
                }
                if self.swaps[self.index] < self.index {
                    break;
                }
                self.swaps[self.index] = 0;
                self.index += 1;
            }

            let pos = (self.index & 1) * self.swaps[self.index];
            self.data.swap(self.index, pos);
            self.swaps[self.index] += 1;
        }
        self.index = 1;
        Some(self.data.clone())
    }
}

#[derive(Debug)]
pub struct Combination<T> {
    chunk_len: u32,
    min: u32,
    mask: u32,
    data: Vec<T>,
}

impl<T: Clone> Combination<T> {
    pub fn new(chunk_len: u32, data: Vec<T>) -> Self {
        let len = data.len() as u32;
        let min = 2_u32.pow(chunk_len) - 1;
        // FIXME(Shaohua): multiply overflow for large vectors.
        let max = 2_u32.pow(len) - 2_u32.pow(len - chunk_len);

        Combination {
            chunk_len,
            min: min as u32,
            mask: max as u32,
            data,
        }
    }

    fn get_chunk(&self) -> Vec<T> {
        let b = format!("{:01$b}", self.mask, self.data.len());
        b.chars()
            .enumerate()
            .filter(|&(_, e)| e == '1')
            .map(|(i, _)| self.data[i].clone())
            .collect()
    }
}

// TODO(Shaohua): Returns reference.
impl<T: Clone> Iterator for Combination<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.mask >= self.min {
            if self.mask.count_ones() == self.chunk_len {
                let res = self.get_chunk();
                self.mask -= 1;
                return Some(res);
            }
            self.mask -= 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Permutation;

    #[test]
    fn test_permutation() {
        let arr = vec![1, 2, 3, 5, 8];
        let p = Permutation::new(arr);
        assert_eq!(p.into_iter().count(), 120);
    }
}
