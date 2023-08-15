// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Circular convolution, also known as cyclic convolution,
//! is a special case of periodic convolution, which is the convolution of two
//! periodic functions that have the same period.
//!
//! Periodic convolution arises, for example, in the context of
//! the discrete-time Fourier transform (DTFT).
//! In particular, the DTFT of the product of two discrete sequences is
//! the periodic convolution of the DTFTs of the individual sequences.
//! And each DTFT is a periodic summation of a continuous Fourier transform function.
//!
//! [Circular convolution](https://en.wikipedia.org/wiki/Circular_convolution)


    
/// This struct stores the first and second signal and performs the circular convolution
pub struct CircularConvolution {
    // First signal and second signal are stored as 1-D array
first_signal: Vec<f64>,
   second_signal:  Vec<f64>,
}

impl CircularConvolution {
    pub  fn new() -> Self {
        Self {
        first_signal: vec![2.0, 1.0, 2.0, -1.0],
        second_signal: vec![1.0, 2.0, 3.0, 4.0],
        }
    }


        
    /// This function performs the circular convolution of the first and second signal
    /// using matrix method
    pub fn circular_convolution(&mut self) -> Vec<f64> {
        let length_first_signal = self.first_signal.len();
        let length_second_signal = self.second_signal.len();

        let max_length = length_first_signal.max(length_second_signal);

        // create a zero matrix of max_length x max_length
        let mut matrix = (0..max_length).iter().map(|_i| vec![0; max_length]).collect();

        // fills the smaller signal with zeros to make both signals of same length
        if length_first_signal < length_second_signal {
            self.first_signal += vec![0; max_length - length_first_signal];
        } else if length_first_signal > length_second_signal {
            self.second_signal += vec![0; max_length - length_second_signal];
        }

        //Fills the matrix in the following way assuming 'x' is the signal of length 4
        // [
        //  [x[0], x[3], x[2], x[1]],
        //  [x[1], x[0], x[3], x[2]],
        //  [x[2], x[1], x[0], x[3]],
        //  [x[3], x[2], x[1], x[0]]
        // ]
        for _i in 0..max_length {
            let rotated_signal = deque(self.second_signal);
            let rotated_signal.rotate(i);
            for j, item in rotated_signal.iter().enumerate() {
                matrix[i][j] += item;
            }
        }

        // multiply the matrix with the first signal
        let mut final_signal = np.matmul(np.transpose(matrix), np.transpose(self.first_signal));

        // rounding-off to two decimal places
        final_signal.iter().map(|mut s| (s * 100.0).round() / 100.0).collect()

    }

    pub fn first_signal(&self) -> &[f64] {
        &self.first_signal
    }

    pub fn second_signal(&self) -> &[f64] {
        &self.second_signal
    }

    pub fn set_first_signal(&mut self, set: &[f64]) {
        self.first_signal.clear();
        self.first_signal.extent(set);
    }

    pub fn set_second_signal(&mut self, set: &[f64]) {
        self.second_signal.clear();
        self.second_signal.extent(set);
    }
}

#[cfg(test)]
mod tests {
    use super::CircularConvolution;

    #[test]
    fn test_circular_convolution() {
        let mut convolution = CircularConvolution::new();
        assert_eq!(convolution.circular_convolution(), &[10.0, 10.0, 6.0, 14.0]);
        assert_eq!(convolution.first_signal(), &[0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6]);
        assert_eq!(convolution.second_signal(), &[0.1, 0.3, 0.5, 0.7, 0.9, 1.1, 1.3, 1.5]);
        assert_eq!(convolution.circular_convolution(), &[5.2, 6.0, 6.48, 6.64, 6.48, 6.0, 5.2, 4.08]);

        convolution.set_first_signal(&[-1, 1, 2, -2]);
        convolution.set_second_signal(&[0.5, 1, -1, 2, 0.75]);
        assert_eq!(convolution.circular_convolution(), &[6.25, -3.0, 1.5, -2.0, -2.75]);

        convolution.set_first_signal(&[1, -1, 2, 3, -1]);
        convolution.set_second_signal(&[1, 2, 3]);
        assert_eq!(convolution.circular_convolution(), [8, -2, 3, 4, 11]);
    }
}
