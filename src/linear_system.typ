#import "plugin.typ": comet-plugin

/// Solve a system of linear equations $A dot arrow(x) = arrow(b)$
/// where $A$ is a tridiagonal matrix.
/// See https://en.wikipedia.org/wiki/Tridiagonal_matrix_algorithm
/// for more information.
///
/// Returns the solutions $arrow(x) in RR^n$ of the system of linear equqations.
///
/// -> array
#let thomas-algorithm(
  /// The matrix $A in RR^(n times n)$ of the system of linear equations.
  /// The data format is an array of arrays, in row-major order.
  ///
  /// -> array
  A,
  /// The vector $arrow(b) in RR^n$ of the system of linear equations.
  ///
  /// -> array
  b,
) = {
  let A = cbor.encode(A.map(row => row.map(float)))
  let b = cbor.encode(b.map(float))

  cbor(comet-plugin.thomas_algorithm(A, b))
}

