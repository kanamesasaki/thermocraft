use faer::diag::DiagRef;
use faer::{Mat, Row, Col, MatRef, RowRef, ColRef};
use faer::{mat, row, col};

fn main() {
    // How to initialize Mat
    {
        // zero matrix
        let zeros: Mat<f64> = Mat::<f64>::zeros(3, 3);
        println!("Zero Matrix {:?}:\n{:?}", zeros.shape(), zeros);

        let zero_row: Row<f64> = Row::<f64>::zeros(4);
        println!("Zero Row Vector {:?}:\n{:?}", zero_row.shape(), zero_row);

        let zero_col: Col<f64> = Col::<f64>::zeros(4);
        println!("Zero Column Vector {:?}:\n{:?}", zero_col.shape(), zero_col);

        // identity matrix
        let identity: Mat<f64> = Mat::<f64>::identity(3, 3);
        println!("Identity Matrix {:?}:\n{:?}", identity.shape(), identity);

        // one matrix
        let ones: Mat<f64> = Mat::<f64>::ones(3, 3);
        println!("One Matrix {:?}:\n{:?}", ones.shape(), ones);

        // constant matrix
        let constants: Mat<f64> = Mat::<f64>::full(3, 3, 5.0);
        println!("Constant Matrix {:?}:\n{:?}", constants.shape(), constants);

        // closure initialization
        let a: Mat<f64> = Mat::from_fn(3, 3, |i, j| ((i+1)*10 + j+1) as f64);
        println!("Matrix A {:?}:\n{:?}", a.shape(), a);

        // macro initialization
        let b: Mat<f64> = mat![
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0],
        [0.0, 0.0, 3.0],
        [0.0, 8.0, 0.0],
        ];
        println!("Matrix B {:?}:\n{:?}", b.shape(), b);

        let rowvec: Row<f64> = row![1.0, 2.0, 3.0];
        println!("Row Vector {:?}:\n{:?}", rowvec.shape(), rowvec);

        let colvec: Col<f64> = col![4.0, 5.0, 6.0];
        println!("Column Vector {:?}:\n{:?}", colvec.shape(), colvec);
    }

    // Mat memory Layout
    {
        // matrix memory layout demonstration
        println!("--- Matrix Data ---");
        let mat: Mat<f64> = Mat::from_fn(3, 2, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
        println!("shape: {:?}, ptr: {:?}", mat.shape(), mat.as_ptr());
        println!("row stride: {:?}, col stride: {:?}", mat.row_stride(), mat.col_stride());
        println!("{:?}", mat);

        println!("--- Sequential Data on Memory ---");
        let base_ptr = mat.as_ptr();
        let row_stride = mat.row_stride();
        let col_stride = mat.col_stride();
        let len = mat.ncols() * col_stride as usize;
        println!("Shape: {:?}, Row Stride: {}, Col Stride: {}", mat.shape(), row_stride, col_stride);

        unsafe {
            for offset in 0..len {
                let ptr = base_ptr.add(offset);
                println!("Offset +{}, {:?}: {:.1}", offset, ptr, *ptr);
            }
        }
    }

    // MatRef basic operations
    {
        // 4x4 の行列を作成
        // (i, j) -> i * 10 + j
        let mat: Mat<f64> = Mat::from_fn(4, 4, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
        println!("mat {:?}:\n{:?}", mat.shape(), mat);

        // 行列全体のビューを取得
        let view: MatRef<f64> = mat.as_ref();
        println!("view {:?}:\n{:?}", view.shape(), view);

        // 部分行列のスライスを取得
        let sub_matrix: MatRef<f64> = mat.as_ref().submatrix(1, 1, 2, 3);
        println!("sub_matrix {:?}:\n{:?}", sub_matrix.shape(), sub_matrix);

        // 複数行のスライスを取得
        let sub_rows: MatRef<f64> = mat.as_ref().subrows(1, 2);
        println!("sub_rows {:?}:\n{:?}", sub_rows.shape(), sub_rows);

        // 複数列のスライスを取得
        let sub_cols: MatRef<f64> = mat.as_ref().subcols(0, 2);
        println!("sub_cols {:?}:\n{:?}", sub_cols.shape(), sub_cols);

        // 行ベクトルのスライスを取得
        let row_vector: RowRef<f64> = mat.as_ref().row(2);
        println!("row_vector {:?}:\n{:?}", row_vector.shape(), row_vector);

        // 列ベクトルのスライスを取得
        let col_vector: ColRef<f64> = mat.as_ref().col(3);
        println!("col_vector {:?}:\n{:?}", col_vector.shape(), col_vector);

        // 転置行列のビューを取得
        let transpose: MatRef<f64> = mat.as_ref().transpose();
        println!("transpose {:?}:\n{:?}", transpose.shape(), transpose);

        // 対角成分のビューを取得
        let diagonal: DiagRef<f64> = mat.as_ref().diagonal();
        println!("diagonal {:?}:\n{:?}", diagonal.dim(), diagonal);
    }

    {
        // 4x4 の行列を作成
        // (i, j) -> i * 10 + j
        let mat: Mat<f64> = Mat::from_fn(4, 4, |i, j| ((i + 1) * 10 + (j + 1)) as f64);
        println!("mat {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", mat.shape(), mat.row_stride(), mat.col_stride(), mat.as_ptr(), mat);

        // 転置行列のビューを取得
        let transpose: MatRef<f64> = mat.as_ref().transpose();
        println!("transpose {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", transpose.shape(), transpose.row_stride(), transpose.col_stride(), transpose.as_ptr(), transpose);

        // 行反転行列のビューを取得
        let reverse_row: MatRef<f64> = mat.as_ref().reverse_rows();
        println!("reverse_row {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", reverse_row.shape(), reverse_row.row_stride(), reverse_row.col_stride(), reverse_row.as_ptr(), reverse_row);

        // 列反転行列のビューを取得
        let reverse_col: MatRef<f64> = mat.as_ref().reverse_cols();
        println!("reverse_col {:?}, row stride {:?}, col stride {:?}, ptr {:?}:\n{:?}", reverse_col.shape(), reverse_col.row_stride(), reverse_col.col_stride(), reverse_col.as_ptr(), reverse_col);
    }
}
