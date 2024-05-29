use ark_ff::PrimeField;

/// Describes the common interface for univariate and multivariate polynomials
/// This F generic parameter should be a field
pub trait PolynomialInterface<F: PrimeField> {
    /// The type of evaluation points for this polynomial.
    /// this could be a set of real numbers or roots of unity depending on the intrepolation logic
    type Point;

    /// Return the total degree of the polynomial
    fn degree(&self) -> usize;

    /// Evaluates `self` at the given `point` in `Self::Point`.
    fn evaluate(&self, point: &Self::Point) -> F;

    /// Checks if the polynomial is zero
    fn is_zero(&self) -> bool;
}

pub trait UnivariantPolynomialInterface<F: PrimeField>: PolynomialInterface<F> {
    /// This function returs an array of co-efficents of this polynomial
    fn coefficients(&self) -> &[F];
    /// This function createsa new polynomial from a list of coefficients slice
    fn from_coefficients_slice(coeffs: &[F]) -> Self;
    /// This function creates a new polynomial from a list of coefficients vector
    fn from_coefficients_vec(coeffs: Vec<F>) -> Self;
    /// This function is used to create a new univariate polynomial using an interpolation
    fn interpolate(point_ys: Vec<F>, domain: Vec<F>) -> Self;
}

pub trait MultivariantPolynomialInterface<F: PrimeField> {
    /// This function returns the number of variables in the polynomial
    fn num_vars(&self) -> usize;
    /// This function creates a new polynomial from a list of evaluations
    fn partial_evaluation(&self, evaluation_point: F, variable_index: usize) -> Self;
    /// This function allows for multiple parial evaluations
    fn partial_evaluations(&self, evaluation_points: Vec<F>, variable_indices: Vec<usize>) -> Self;
    /// This function is used to evaluate the polynomial at a given point
    fn evaluate(&self, point: &Vec<F>) -> Option<F>;
}
