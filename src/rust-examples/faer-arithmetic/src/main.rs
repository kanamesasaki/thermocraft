use faer::linalg::matmul::matmul;
use faer::{mat, zip, unzip, Accum, Mat, Scale, get_global_parallelism};
use std::time::Instant;
use std::hint::black_box;

fn main() {
    {
        let a: Mat<f64> = mat![[1.0, 2.0], [3.0, 4.0]];
        let b: Mat<f64> = mat![[5.0, 6.0], [7.0, 8.0]];

        // 1. 行列の和
        let c: Mat<f64> = &a + &b;
        println!("{:?}", c);

        // 2. 行列の差
        let d: Mat<f64> = &a - &b;
        println!("{:?}", d);

        // 3. 行列の積
        let e: Mat<f64> = &a * &b;
        println!("{:?}", e);
    }

    {
        // 1024x1024 の f64 行列は約8MB
        let n: usize = 1024;
        let a: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i + j) as f64);
        let b: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i as isize - j as isize) as f64);
        let c: Mat<f64> = Mat::<f64>::full(n, n, 1.0);

        // ==========================================
        // Case 1: 3つの行列の足し算 (D = A + B + C)
        // ==========================================
        println!("--- Case 1: D = A + B + C ---");

        // ==========================================
        // 1. operators
        // ==========================================
        let start = Instant::now();
        let d1 = &a + &b + &c;
        println!("Operators : {:.2?}", start.elapsed());

        // ==========================================
        // 2. zip!
        // ==========================================
        let start = Instant::now();
        let mut d2 = Mat::zeros(n, n);
        zip!(&mut d2, &a, &b, &c).for_each(|unzip!(d, a, b, c)| {
            *d = *a + *b + *c;
        });
        println!("zip!      : {:.2?}", start.elapsed());
    }

    {
        // ==========================================
        // Case 2: 積和演算 (E = A + B * C)
        // ==========================================
        println!("--- Case 2: E = A + B * C ---");
        let n: usize = 1024;
        let a: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i + j) as f64);
        let b: Mat<f64> = Mat::<f64>::from_fn(n, n, |i, j| (i as isize - j as isize) as f64);
        let c: Mat<f64> = Mat::<f64>::full(n, n, 1.0);
        let loop_count: u32 = 50;
        // ==========================================
        // 1. operators
        // ==========================================
        let start = Instant::now();
        // Process:
        // 1. tmp = B * C (Alloc + matmul Replace)
        // 2. res = A + tmp (Alloc + Zip Add)
        for _ in 0..loop_count {
            let e1 = &a + 2.0 * &b * &c;
            black_box(e1);
        }
        println!("operators                 : {:.2?}", start.elapsed() / loop_count);

        // ==========================================
        // 2. clone + Add
        // ==========================================
        let start = Instant::now();
        // Process:
        // 1. res = A.clone() (Alloc + Copy)
        // 2. res += B * C (matmul Add)
        for _ in 0..loop_count {
            let mut e2 = a.clone();
            matmul(
                e2.as_mut(),
                Accum::Add,
                &b,
                &c,
                2.0,
                get_global_parallelism()
            );
            black_box(e2);
        }
        println!("copy + matmul Add         : {:.2?}", start.elapsed() / loop_count);


        // ==========================================
        // 3. Replace + Zip
        // ==========================================
        let start = Instant::now();
        // Process:
        // 1. res (Alloc)
        // 2. res = B * C (matmul Replace)
        // 3. res += A (Zip Add)
        
        for _ in 0..loop_count {
            let mut e3 = Mat::<f64>::zeros(n, n);
            matmul(
                e3.as_mut(),
                Accum::Replace,
                &b,
                &c,
                2.0,
                get_global_parallelism()
            );
            // zip!(&mut e3, &a).for_each(|unzip!(e, a)| *e += *a);
            e3 += &a;
            black_box(e3);
        }
        println!("matmul Replace + zip Add  : {:.2?}", start.elapsed() / loop_count);
    }

    {
        // ==========================================
        // Case 3: 複数回の行列積 (F = A * B * C * D * E)
        // ==========================================
        println!("--- Chain: A * B * C * D * E ---");
        let n = 1024; 
        let loop_count = 50;

        let a = Mat::<f64>::from_fn(n, n, |i, j| (i + j) as f64);
        let b = Mat::<f64>::from_fn(n, n, |i, j| (i as isize - j as isize) as f64);
        let c = Mat::<f64>::full(n, n, 1.0);
        let d = Mat::<f64>::from_fn(n, n, |i, j| (j as isize - i as isize) as f64);
        let e = Mat::<f64>::from_fn(n, n, |i, j| (i * j) as f64);

        // ==========================================
        // 1. Operators
        // ==========================================
        let start = Instant::now();
        for _ in 0..loop_count {
            let res = &a * &b * &c * &d * &e;
            black_box(res);
        }
        println!("operators (4 allocs)  : {:.2?}", start.elapsed() / loop_count);

        // ==========================================
        // 2. ping-pong buffering
        // ==========================================
        let par = get_global_parallelism();
        let start = Instant::now();

        for _ in 0..loop_count {
            let mut buf1 = Mat::<f64>::zeros(n, n);
            let mut buf2 = Mat::<f64>::zeros(n, n);
            matmul(buf1.as_mut(), Accum::Replace, &a, &b, 1.0, par);
            matmul(buf2.as_mut(), Accum::Replace, &buf1, &c, 1.0, par);
            matmul(buf1.as_mut(), Accum::Replace, &buf2, &d, 1.0, par);
            matmul(buf2.as_mut(), Accum::Replace, &buf1, &e, 1.0, par);
            black_box(buf2);
        }
        println!("ping-pong (2 allocs)  : {:.2?}", start.elapsed() / loop_count);
    }
}