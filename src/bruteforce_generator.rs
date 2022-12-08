use std::ops::{Deref, DerefMut};

pub struct BruteForceGenerator<'charset, ElementType> {
    length: usize,
    charset: &'charset [ElementType],
    current_indexes: Vec<usize>,
    current_permutation: Vec<ElementType>,
}

impl<'charset, ElementType: Clone> BruteForceGenerator<'charset, ElementType> {
    /// Build a bruteforcer running from the empty result
    pub fn new(charset: &'charset [ElementType]) -> Self {
        BruteForceGenerator {
            charset,
            current_indexes: Vec::with_capacity(8),
            length: 0,
            current_permutation: Vec::with_capacity(8),
        }
    }

    /// Build a bruteforcer generating string with at least `length` characters
    pub fn start_at_length(charset: &'charset [ElementType], length: usize) -> Self {
        let mut current_indexes = Vec::with_capacity(length);
        current_indexes.resize(length, 0);
        let mut current_permutation = Vec::with_capacity(length);
        current_permutation.resize(length, charset.first().unwrap().clone());
        BruteForceGenerator {
            length,
            charset,
            current_indexes,
            current_permutation,
        }
    }

    /// Build a bruteforcer that starts from a specific permutation
    pub fn start_from(
        charset: &'charset [ElementType],
        current_permutation: &[ElementType],
    ) -> Self {
        BruteForceGenerator {
            charset,
            current_indexes: Vec::with_capacity(8.max(current_permutation.len())),
            length: current_permutation.len(),
            current_permutation: current_permutation.to_owned(),
        }
    }

    fn raw_next(&mut self) -> Vec<ElementType> {
        // Move to next permutation, may overflow and set all existing counter to 0
        let mut index = self.current_indexes.len() - 1;
        for i in self.current_indexes.iter_mut() {
            *i += 1;
            let carry = *i >= self.charset.len();
            *i %= self.charset.len();
            self.current_permutation[index] = self.charset[*i].clone();
            index -= 1;
            if !carry {
                break;
            }
        }
        // In that case, just increase the guess length
        if self.current_indexes.iter().all(|index| *index == 0) {
            self.length += 1;
            self.current_indexes.push(0);
            self.current_permutation.insert(0, self.charset[0].clone());
        }
        self.current_permutation.clone()
    }
}

impl<'charset, ElementType: Clone> Iterator for BruteForceGenerator<'charset, ElementType> {
    type Item = Vec<ElementType>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.raw_next())
    }
}

/// For ASCII string, we can make use of unsafe to speed it up
pub struct AsciiStringBruteForceGenerator<'a>(BruteForceGenerator<'a, u8>);

impl<'charset> AsciiStringBruteForceGenerator<'charset> {
    pub fn new(charset: &'charset [u8]) -> Self {
        Self(BruteForceGenerator::new(charset))
    }
}

impl<'a> Deref for AsciiStringBruteForceGenerator<'a> {
    type Target = BruteForceGenerator<'a, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AsciiStringBruteForceGenerator<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'charset> Iterator for AsciiStringBruteForceGenerator<'charset> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|bytes| unsafe { String::from_utf8_unchecked(bytes) })
    }
}
