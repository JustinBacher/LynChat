//! General utility functions for lyn-core.

use crate::prelude::*;
use ndarray::ArrayView1; // Combined use statement

/// Calculates the cosine similarity between two vectors.
///
/// Returns `Ok(similarity)` where similarity is a value between -1.0 and 1.0,
/// or an `Err` if the vectors have different lengths, are empty, or if a norm is zero.
pub fn cosine_similarity(v1: &[f32], v2: &[f32]) -> Result<f32> {
    if v1.len() != v2.len() {
        return Err(Error::UtilsError(f!(
            "Vectors must have the same length ({} != {})",
            v1.len(),
            v2.len()
        )));
    }
    if v1.is_empty() {
        return Err(Error::UtilsError(f!("Vectors cannot be empty")));
    }

    // Convert slices to ArrayView1 for ndarray operations
    let arr1 = ArrayView1::from(v1);
    let arr2 = ArrayView1::from(v2);

    let dot_product = arr1.dot(&arr2);

    let norm1 = arr1.dot(&arr1).sqrt();
    let norm2 = arr2.dot(&arr2).sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        // Handle zero vectors - similarity is undefined or could be considered 0 or 1 depending on convention.
        // Returning an error or 0.0 are common choices. Let's return 0.0 for simplicity here.
        // Alternatively, return Err(anyhow!("Cannot compute similarity with zero vector"));
        return Ok(0.0);
    }

    let similarity = dot_product / (norm1 * norm2);

    // Clamp the value to [-1.0, 1.0] due to potential floating point inaccuracies
    Ok(similarity.clamp(-1.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq; // For float comparisons

    #[test]
    fn test_cosine_similarity_identical() {
        let v1 = [1.0, 2.0, 3.0];
        let v2 = [1.0, 2.0, 3.0];
        assert_relative_eq!(cosine_similarity(&v1, &v2).unwrap(), 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let v1 = [1.0, 0.0];
        let v2 = [0.0, 1.0];
        assert_relative_eq!(cosine_similarity(&v1, &v2).unwrap(), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_cosine_similarity_opposite() {
        let v1 = [1.0, 2.0];
        let v2 = [-1.0, -2.0];
        assert_relative_eq!(cosine_similarity(&v1, &v2).unwrap(), -1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_cosine_similarity_general() {
        let v1 = [1.0, 2.0, 3.0];
        let v2 = [4.0, 5.0, 6.0];
        // Manually calculate expected value:
        // dot = 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        // norm1 = sqrt(1^2 + 2^2 + 3^2) = sqrt(1 + 4 + 9) = sqrt(14)
        // norm2 = sqrt(4^2 + 5^2 + 6^2) = sqrt(16 + 25 + 36) = sqrt(77)
        // sim = 32 / (sqrt(14) * sqrt(77)) = 32 / sqrt(1078) approx 0.9746318
        let expected = 32.0 / (14.0f32.sqrt() * 77.0f32.sqrt());
        assert_relative_eq!(
            cosine_similarity(&v1, &v2).unwrap(),
            expected,
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_cosine_similarity_different_lengths() {
        let v1 = [1.0, 2.0];
        let v2 = [1.0, 2.0, 3.0];
        assert!(cosine_similarity(&v1, &v2).is_err());
    }

    #[test]
    fn test_cosine_similarity_empty() {
        let v1: [f32; 0] = [];
        let v2: [f32; 0] = [];
        assert!(cosine_similarity(&v1, &v2).is_err());
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let v1 = [0.0, 0.0];
        let v2 = [1.0, 2.0];
        assert_relative_eq!(cosine_similarity(&v1, &v2).unwrap(), 0.0, epsilon = 1e-6);
        assert_relative_eq!(cosine_similarity(&v2, &v1).unwrap(), 0.0, epsilon = 1e-6);

        let v3 = [0.0, 0.0];
        assert_relative_eq!(cosine_similarity(&v1, &v3).unwrap(), 0.0, epsilon = 1e-6);
    }
}
