pub mod err;
pub mod calculator;
pub mod matrix;
pub mod memory;

pub type LAResult<T> = Result<T, err::LAError>;

const INIT_MEMORY_CAPACITY: usize = 3;

const PROGRAM_LIST: [&str; 1] = [
    "mat_mul"
];

const PROGRAM_SOURCE: &str = r#"
kernel void mat_mul (
    global float* c,
    const int N,
    const int K,
    const global float* a,
    const global float* b
) {
    const int globalRow = get_global_id(0);
    const int globalCol = get_global_id(1);

    float interm = 0.0f;
    for (int k = 0; k < K; k++) {
        interm += a[globalRow * K + k] * b[k * N + globalCol];
    }

    c[globalRow * N + globalCol] = interm;
}
"#;