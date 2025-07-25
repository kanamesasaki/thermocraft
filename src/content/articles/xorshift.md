---
title: 'Xorshift Random Number Generator'
description: 'Xorshift is one of the simplest and high-speed random number generator (RNG) algorithms. In this article, we explore the concept of Xorshift, and implement Rust code for searching valid Xorshift parameters for 32-bit binary.'
pubDate: 2025-06-01
updatedDate: 2025-06-04
heroImage: ''
tags: ['programming', 'numerical analysis']
---

## Introduction

There are various well-known algorithms for generating pseudo-random numbers, such as Linear Congruential Generator, Xorshift, and Mersenne Twister.
Among them, Xorshift is known for its simple operations and high speed.
In this article, we will explore the basic concepts of Xorshift, referring to the original papers by Marsaglia [[1,2]](#reference).
The focus will be to grasp the intuition behind the algorithm rather than just implementing the Xorshift algorithm.

## Pseudo-Random Number Generation by Binary Matrix

We will consider a method to generate pseudo-random numbers using binary matrices.
Applying a $n \times n$ binary matrix $T$ to an $n$-bit binary vector $\beta$, we can generate a sequence of vectors $\beta T,~\beta T^2,~\cdots$.

In this setup, the following two statements are equivalent:

1. For any non-zero $n$-bit binary vector, applying the $n \times n$ binary matrix $T$ will generate all non-zero $n$-bit Boolean vectors.
2. The period of the $n \times n$ binary matrix $T$ is $2^n-1$.

In the original paper [[2]](#reference), this equivalence is discussed in detail using characteristic polynomials.
But for rough understanding, it can be intuitively explained as follows.

1 $\Leftarrow$ 2:

If the period of $T$ is $2^n-1$, it means that for any non-zero $n$-bit Boolean vector $\beta$, we have $\beta T^{2^n-1} = \beta$.
In addition, the sequence $\beta T,~ \beta T^2,~ \cdots,~ \beta T^{2^n-2}$ does not revisit $\beta$ until it reaches $\beta T^{2^n-1}$.
Therefore, the sequentially generated $2^{2^n-1}$ vectors are all distinct and non-zero, and they cover all possible non-zero $n$-bit Boolean vectors.

1 $\Rightarrow$ 2:

When applying the matrix $T$ to a vector $\beta$, it returns a specific vector $\beta T$.
To generate all possible non-zero $n$-bit Boolean vectors, at each step, we need to generate a different vector.
Thus, the sequence must return to the original vector $\beta$ after $2^n-1$ operations, resulting in a period of $2^n-1$.

## Binary Matrix

Before discussing the Xorshift algorithm, we should quickly review the properties of binary matrix.
One important remark is that the binary matrix and its operations in this context are NOT the definitions from the Boolean algebra (For more info about the Boolean matrices, Luce's paper could be a good introduction for the basics [[3]](#reference)).
The basic operations (addition and multiplication) used here are defined as shown in Eqs. (1) and (2).
The difference from the Boolean algebra is that the addition operation is defined as XOR rather than OR.
With this definition, many properties from the real/complex matrix can be applied to the binary matrix as well.

$$
\begin{align}
\left\{\begin{array}{l}
0 + 0 = 0\\
0 + 1 = 1\\
1 + 0 = 1\\
1 + 1 = 0
\end{array}\right.
\end{align}
$$

$$
\begin{align}
\left\{ \begin{array}{l}
0 \cdot 0 = 0\\
0 \cdot 1 = 0\\
1 \cdot 0 = 0\\
1 \cdot 1 = 1
\end{array} \right.
\end{align}
$$

For matrix operations, the calculation order is basically the same as the usual matrix calculations with the addition and multiplication rules defined above.
So, for example, if we define $3 \times 3$ binary matrices $A$ and $B$ as shown in Eq. (3), the addition and multiplication operations are described as follows.

$$
\begin{align}
A = \begin{bmatrix}
1 & 0 & 1 \\
0 & 1 & 0 \\
0 & 0 & 1
\end{bmatrix}, \quad
B = \begin{bmatrix}
0 & 0 & 1 \\
1 & 1 & 1 \\
0 & 1 & 0
\end{bmatrix}
\end{align}
$$

$$
\begin{align}
A + B = \begin{bmatrix}
1 & 0 & 0 \\
1 & 0 & 1 \\
0 & 1 & 1
\end{bmatrix}, \quad
AB = \begin{bmatrix}
0 & 1 & 1 \\
1 & 1 & 1 \\
0 & 1 & 0
\end{bmatrix}
\end{align}
$$

## Motivation to Use Xorshift

Now, the question is how to find an $n \times n$ binary matrix $T$ with a period of $2^n-1$.
One requirement is that the matrix $T$ must be regular, because...

- The matrix $T$ must be bijective, to be able to generate all $n$-bit Boolean vectors.
- The matrix $T$ is bijective ⇔ The matrix $T$ has an inverse matrix $T^{-1}$.

However, this requirement alone does not specify the form of the matrix, and in fact, there are many matrices that satisfy the periodic condition.

Rather than trying to narrow down the required form of the matrix, let us think about some matrices that have low computational cost for the operations, and then check if they satisfy the periodic condition.
The key concept that comes into play at this point is the Xorshift operation.

Bit-shift operation is a low-cost operation in computer systems, and it can be represented as shift matrices: $L^a$ (left/upper shift by a-bits) and $R^b$ (right/lower shift by b-bits).
For example, in a $4 \times 4$ matrix, the matrix $L^1$ for left shifting by 1 bit and the matrix $R^1$ for right shifting by 1 bit are typically represented as shown in Eq. (5). If you apply a row vector to $L^1$, it looks like shifting 1 bit to the right. Also, if you apply a row vector to $R^1$, it looks like shifting 1 bit to the left. However, we keep the setup like this by understanding that "shift left" means that the bits are shifted towards the MSB (most significant bit) direction, and "shift right" means that the bits are shifted towards the LSB (least significant bit) direction.

$$
\begin{align}
L^1 =
\begin{bmatrix}
0 & 1 & 0 & 0 \\
0 & 0 & 1 & 0 \\
0 & 0 & 0 & 1 \\
0 & 0 & 0 & 0 \\
\end{bmatrix},
\quad
R^1 =
\begin{bmatrix}
0 & 0 & 0 & 0 \\
1 & 0 & 0 & 0 \\
0 & 1 & 0 & 0 \\
0 & 0 & 1 & 0 \\
\end{bmatrix}
\end{align}
$$

These matrices are not regular. However, by adding an XOR (exclusive OR) operation, we can create matrices $T=I+L^a$ and $T=I+R^b$ that are regular.
This one-step Xorshift operation can be performed as the binary operation without performing the matrix calculation. An example code for a 32-bit binary vector is shown below.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryVector32 {
    row: u32,
}

impl BinaryVector32 {
    pub fn new() -> Self {
        Self { row: 0 }
    }

    pub fn xorshift_r(mut self, bits: usize) -> Self {
        self.row ^= self.row >> bits;
        self
    }

    pub fn xorshift_l(mut self, bits: usize) -> Self {
        self.row ^= self.row << bits;
        self
    }
}
```

Since a product of regular matrices is also regular, we can create candidates for the matrix $T$ by multiplying these matrices.
According to the original paper [[1]](#reference), for $n=32$, there are no matrices of the form $T=(I+L^a)(I+R^b)$ or $T=(I+R^b)(I+L^a)$ that satisfy the periodic condition.
But if we consider matrices of the form $T=(I+L^a)(I+R^b)(I+L^c)$, there are several combinations of $(a, b, c)$ that satisfy the condition.

## Searching for Valid Parameters for 32-bit Binary

Let's search for the parameter set $(a, b, c)$ that satisfies the periodic condition, for 32-bit binary.
The period of the $32 \times 32$ matrix $T$ must be $2^{32}-1$.
The value $2^{32}-1$ can be factored as shown in Eq. (6).

$$
\begin{align}
2^{32} - 1
% &= (2^{16} + 1)(2^{16} - 1) \notag \\
% &= (2^{16} + 1)(2^8 + 1)(2^8 - 1) \notag \\
% &= (2^{16} + 1)(2^8 + 1)(2^4 + 1)(2^4 - 1) \notag \\
&= (2^{16} + 1)(2^8 + 1)(2^4 + 1)(2^2 + 1)(2^2 - 1) \notag \\
&= 65537 \cdot 257 \cdot 17 \cdot 5 \cdot 3
\end{align}
$$

We should confirm that the matrix $T$ becomes $I$ after $2^{32}-1$ multiplications, but it does not become $I$ with shorter periods, specifically at the periods of any composite factors of $2^{32}-1$.
In other words, we should check if the following conditions are satisfied (By checking Eqs. (8) to (12), we can confirm that $T^{3} \neq I,~ T^{5} \neq I,~ \cdots$ as well).

$$
\begin{align}
&T^{2^{32}-1} = I \quad \text{or} \quad T^{2^{32}} = T \\
&T^{(2^{32}-1) / 3} \neq I \\
&T^{(2^{32}-1) / 5} \neq I \\
&T^{(2^{32}-1) / 17} \neq I \\
&T^{(2^{32}-1) / 257} \neq I \\
&T^{(2^{32}-1) / 65537} \neq I
\end{align}
$$

The following Rust code implements a $32 \times 32$ binary matrix and provides methods to perform the operations described above. For the calculation of the matrix power, we use the binary exponentiation method (Exponentiation by squaring), which is efficient for large exponents.

```rust
/// A 32x32 binary matrix implemented using bitwise operations for efficiency
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryMatrix32 {
    /// Each u32 represents a row of 32 binary values
    rows: [u32; 32],
}

impl BinaryMatrix32 {
    /// Create a new zero matrix
    pub fn new() -> Self {
        Self { rows: [0; 32] }
    }

    /// Create an identity matrix
    pub fn identity() -> Self {
        let mut matrix = Self::new();
        for i in 0..32 {
            matrix.rows[i] = 1 << i;
        }
        matrix
    }

    /// Get a specific bit at (row, col)
    pub fn get(&self, row: usize, col: usize) -> bool {
        if row >= 32 || col >= 32 {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
        (self.rows[row] & (1 << col)) != 0
    }

    /// Set a specific bit at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        if row >= 32 || col >= 32 {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
        if value {
            self.rows[row] |= 1 << col;
        } else {
            self.rows[row] &= !(1 << col);
        }
    }

    /// Binary matrix addition
    pub fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..32 {
            result.rows[i] = self.rows[i] ^ other.rows[i];
        }
        result
    }

    /// Create a shift matrix for right shift by `bits` positions
    /// Example for 3 x 3 matrix with bits = 1
    ///                              |0, 0, 0|
    /// [v1, v2, 0] = [v0, v1, v2] * |1, 0, 0|
    ///                              |0, 1, 0|
    pub fn shift_right(bits: usize) -> Self {
        let mut matrix = Self::new();
        if bits >= 32 || bits == 0 {
            return matrix;
        }

        for i in 0..(32 - bits) {
            matrix.rows[i + bits] = 1 << i;
        }
        matrix
    }

    /// Create a shift matrix for left shift by `bits` positions
    /// Example for 3 x 3 matrix with bits = 1
    ///                              |0, 1, 0|
    /// [0, v0, v1] = [v0, v1, v2] * |0, 0, 1|
    ///                              |0, 0, 0|
    pub fn shift_left(bits: usize) -> Self {
        let mut matrix = Self::new();
        if bits >= 32 || bits == 0 {
            return matrix;
        }

        for i in bits..32 {
            matrix.rows[i - bits] = 1 << i;
        }
        matrix
    }

    /// Binary matrix multiplication
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for i in 0..32 {
            for j in 0..32 {
                let mut sum = false;
                for k in 0..32 {
                    sum ^= self.get(i, k) && other.get(k, j);
                }
                if sum {
                    result.rows[i] |= 1 << j;
                }
            }
        }
        result
    }

    pub fn pow(&self, n: u64) -> Self {
        if n == 0 {
            return Self::identity(); // T^0 = I
        }
        if n == 1 {
            return *self; // T^1 = T
        }

        // binary exponentation in iterative form
        let mut base = *self;
        let mut result = Self::identity();
        let mut exp = n;

        while exp > 0 {
            if exp & 1 == 1 {               // if LSB is odd, multiply base
                result = result.mul(&base);
            }
            base = base.mul(&base);         // calculate next base
            exp >>= 1;                      // shift right by 1
        }
        result
    }

    /// Comprehensive check for correct xorshift period (2^32 - 1)
    /// Verifies that T^(2^32) = T and that T doesn't have smaller periods
    pub fn has_correct_full_period(&self) -> bool {
        // First check: T^(2^32) = T
        let matrix_pow_2_32 = self.pow(2_u64.pow(32));
        if *self != matrix_pow_2_32 {
            return false;
        }

        let identity = Self::identity();

        // Check that no smaller periods exist by testing T^k != I for proper divisors k of (2^32-1)
        // 2^32 - 1 = 3 × 5 × 17 × 257 × 65537, so check each quotient
        let quotients = [
            (2_u64.pow(32) - 1) / 3,     // Missing factor 3
            (2_u64.pow(32) - 1) / 5,     // Missing factor 5
            (2_u64.pow(32) - 1) / 17,    // Missing factor 17
            (2_u64.pow(32) - 1) / 257,   // Missing factor 257
            (2_u64.pow(32) - 1) / 65537, // Missing factor 65537
        ];

        !quotients.iter().any(|&quotient| self.pow(quotient) == identity)
    }

    /// Test if a triplet (a, b, c) creates a valid xorshift matrix
    pub fn test_xorshift_triplet(a: usize, b: usize, c: usize) -> bool {
        let identity = Self::identity();
        let il_a = identity.add(&Self::shift_left(a));
        let ir_b = identity.add(&Self::shift_right(b));
        let il_c = identity.add(&Self::shift_left(c));

        let xorshift = il_a.mul(&ir_b).mul(&il_c);
        xorshift.has_correct_full_period()
    }

    /// Search for all valid xorshift parameter triplets in the range 1..32
    pub fn search_valid_triplets() -> Vec<(usize, usize, usize)> {
        let mut valid_triplets = Vec::new();
        let mut tested = 0;
        const TOTAL: usize = 15376; // Total cases (a, b, c) with a <= c

        for a in 1..32 {
            for b in 1..32 {
                for c in a..32 {
                    tested += 1;

                    if tested % 1000 == 0 {
                        println!("Progress: {}/{} ({:.1}%)", tested, TOTAL,
                               (tested as f64 / TOTAL as f64) * 100.0);
                    }

                    if Self::test_xorshift_triplet(a, b, c) {
                        valid_triplets.push((a, b, c));
                    }
                }
            }
        }

        valid_triplets
    }
}
```

Executing the above code, we can obtain the parameter sets $(a, b, c)$ that satisfy the periodic condition.
The following output is consistent with the results presented in the original paper [[1]](#reference).

```console
Found 81 valid triplets:
| 1, 3,10| 1, 5,16| 1, 5,19| 1, 9,29| 1,11, 6| 1,11,16| 1,19, 3| 1,21,20| 1,27,27|
| 2, 5,15| 2, 5,21| 2, 7, 7| 2, 7, 9| 2, 7,25| 2, 9,15| 2,15,17| 2,15,25| 2,21, 9|
| 3, 1,14| 3, 3,26| 3, 3,28| 3, 3,29| 3, 5,20| 3, 5,22| 3, 5,25| 3, 7,29| 3,13, 7|
| 3,23,25| 3,25,24| 3,27,11| 4, 3,17| 4, 3,27| 4, 5,15| 5, 3,21| 5, 7,22| 5, 9, 7|
| 5, 9,28| 5, 9,31| 5,13, 6| 5,15,17| 5,17,13| 5,21,12| 5,27, 8| 5,27,21| 5,27,25|
| 5,27,28| 6, 1,11| 6, 3,17| 6,17, 9| 6,21, 7| 6,21,13| 7, 1, 9| 7, 1,18| 7, 1,25|
| 7,13,25| 7,17,21| 7,25,12| 7,25,20| 8, 7,23| 8, 9,23| 9, 5,14| 9, 5,25| 9,11,19|
| 9,21,16|10, 9,21|10, 9,25|11, 7,12|11, 7,16|11,17,13|11,21,13|12, 9,23|13, 3,17|
|13, 3,27|13, 5,19|13,17,15|14, 1,15|14,13,15|15, 1,29|17,15,20|17,15,23|17,15,26|
```

## Reference

1. Marsaglia, G. (2003). Xorshift RNGs. Journal of Statistical Software, 8(14), 1–6. doi: [10.1109/IEEESTD.2019.8766229](https://doi.org/10.18637/jss.v008.i14).
2. George Marsaglia, Liang-Huei Tsay, Matrices and the structure of random number sequences, Linear Algebra and its Applications, Volume 67, 1985, Pages 147-156, ISSN 0024-3795, doi: [10.1016/0024-3795(85)90192-2](<https://doi.org/10.1016/0024-3795(85)90192-2>).
3. Luce, R. Duncan. "A note on Boolean matrix theory." Proceedings of the American Mathematical Society 3.3, 1952, 382-388, doi: [10.2307/2031888](https://doi.org/10.2307/2031888).
