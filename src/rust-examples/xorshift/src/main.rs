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
            panic!("Index out of bounds: ({row}, {col})");
        }
        (self.rows[row] & (1 << col)) != 0
    }

    /// Set a specific bit at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        if row >= 32 || col >= 32 {
            panic!("Index out of bounds: ({row}, {col})");
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

    /// Print matrix in a readable format (first num rows/cols for debugging)
    pub fn print_debug(&self, size: usize) {
        let size = size.min(32);
        for i in 0..size {
            for j in 0..size {
                print!("{}", if self.get(i, j) { "1" } else { "0" });
            }
            println!();
        }
        println!();
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
        
        // Check no smaller periods exist by testing T^k != I for proper divisors k of (2^32-1)
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

    /// Search for all valid xorshift parameter triplets in range 1..32
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BinaryVector32 {
    row: u32,
}

impl BinaryVector32 {
    pub fn new() -> Self {
        Self { row: 0 }
    }

    pub fn get(&self, col: usize) -> bool {
        if col >= 32 {
            panic!("Index out of bounds: {col}");
        }
        (self.row & (1 << col)) != 0
    }

    pub fn set(&mut self, col: usize, value: bool) {
        if col >= 32 {
            panic!("Index out of bounds: {col}");
        }
        if value {
            self.row |= 1 << col;
        } else {
            self.row &= !(1 << col);
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            row: self.row ^ other.row,
        }
    }

    pub fn mul(&self, other: &BinaryMatrix32) -> Self {
        let mut result = Self::new();
        for j in 0..32 {
            let mut sum = false;
            for i in 0..32 {
                sum ^= self.get(i) && other.get(i, j);
            }
            result.set(j, sum);
        }
        result
    }

    pub fn xorshift_r(mut self, bits: usize) -> Self {
        self.row ^= self.row >> bits;
        self
    }

    pub fn xorshift_l(mut self, bits: usize) -> Self {
        self.row ^= self.row << bits;
        self
    }

    /// Perform one step of xorshift with parameters (a, b, c)
    /// Equivalent to: x ^= x << a; x ^= x >> b; x ^= x << c;
    pub fn xorshift(self, a: usize, b: usize, c: usize) -> Self {
        self.xorshift_l(a).xorshift_r(b).xorshift_l(c)
    }
}

impl Default for BinaryMatrix32 {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BinaryVector32 {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("Starting Xorshift Parameter search...");
    let valid_triplets = BinaryMatrix32::search_valid_triplets();
    
    println!("Found {} valid triplets:", valid_triplets.len());
    // Print in table format, 9 triplets per row
    for (i, (a, b, c)) in valid_triplets.iter().enumerate() {
        print!("|{a:2},{b:2},{c:2}");
        
        // Add row separator every 9 items or at the end
        if (i + 1) % 9 == 0 || i == valid_triplets.len() - 1 {
            println!("|");
        }
    }
}
