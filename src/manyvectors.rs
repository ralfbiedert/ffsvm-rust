use std::fmt;
use std::iter::{IntoIterator};
use std::marker::{Sized,Copy};

/// Basic "matrix' we use for fast SIMD and parallel operations.
/// 
/// Note: Right now we use a Matrix mostly as a vector of vectors and is mostly 
/// intended for read operations.
pub struct ManyVectors<T> where T : Copy + Sized,
{
    /// Number of vectors this matrix has 
    pub vectors: usize,
    
    /// Number of attributes this matrix has per subvector
    pub attributes: usize,
    
    /// We store all data in one giant array for performance reasons (caching)
    pub data: Vec<T>
}



/// Basic iterator struct to go over matrix 
pub struct IterManyVectors<'a, T: 'a> where  T : Copy + Sized
{
    /// Reference to the matrix we iterate over.
    pub matrix: &'a ManyVectors<T>,
    
    /// Current index of vector iteration.
    pub index: usize,
}




impl<T> ManyVectors<T> where T : Copy + Sized
{
    /// Creates a new empty Matrix.
    pub fn with_dimension(vectors: usize, attributes: usize, default: T) -> ManyVectors<T> {
        ManyVectors::<T> {
            vectors,
            attributes,
            data: vec![default; vectors * attributes],
        }
    }
    
    /// Given a flat vec and dimensions, set the matrix with the given dimensions 
    pub fn from_flat(vector: Vec<T>, vectors: usize, attributes: usize) -> ManyVectors<T> {
        ManyVectors::<T> {
            vectors,
            attributes,
            data: vector,
        } 
    }
    

    #[inline]
    pub fn get_vector(&self, index_vector: usize) -> &[T] {
        let start_index = self.offset(index_vector, 0);
        &self.data[start_index..start_index + self.attributes]
    }

    #[inline]
    pub fn get_vector_mut(&mut self, index_vector: usize) -> &mut [T] {
        let start_index = self.offset(index_vector, 0);
        &mut self.data[start_index..start_index + self.attributes]
    }
    
    
    #[inline]
    pub fn set_vector(&mut self, index_vector: usize, vector: &[T]) {
        let start_index = self.offset(index_vector, 0);
        for i in 0 .. self.attributes {
            self.data[start_index + i] = vector[i];    
        }
    }
    
    #[inline]
    pub fn offset(&self, index_vector: usize, index_attribute: usize) -> usize {
        (index_vector * self.attributes + index_attribute)
    }
    
    #[inline]
    pub fn set(&mut self, index_vector: usize, index_attribute: usize, value: T) {
        let  index = self.offset(index_vector, index_attribute);
        self.data[index] = value;
    }

    #[inline]
    pub fn get(&self, index_vector: usize, index_attribute: usize) -> T {
        let  index = self.offset(index_vector, index_attribute);
        self.data[index]
    }
}



impl <T> fmt::Debug for ManyVectors<T> where T : Copy + Sized
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, [data])", self.vectors, self.attributes)
    }
    
}



impl <'a, T> IntoIterator for &'a ManyVectors<T> where T : Copy + Sized
{
    type Item = &'a [T];
    type IntoIter = IterManyVectors<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterManyVectors { matrix: self, index: 0 }
    }
}



impl <'a, T> Iterator for IterManyVectors<'a, T> where T : Copy + Sized
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.matrix.vectors { 
            None 
        } else {
            self.index += 1;
            Some(self.matrix.get_vector(self.index-1))
        }
    }
}



#[test]
fn test_iter() {
    let matrix = ManyVectors::with_dimension(10, 5, 0);
    for x in &matrix {
        assert_eq!(x[0], 0);
    }
}