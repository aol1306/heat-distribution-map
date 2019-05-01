static EPSILON: f64 = 0.00000000001;

fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();

    for p in 0..n {
        // find pivot row and swap
        let mut max = p;
        for i in p+1..n {
            if a[i][p].abs() > a[max][p].abs() {
                max = i;
            }
        }

        let temp = a[p].clone();
        a[p] = a[max].clone();
        a[max] = temp;
        let t = b[p];
        b[p] = b[max];
        b[max] = t;

        // singular or nearly singular
        if a[p][p].abs() <= EPSILON {
            return None;
        }

        // pivot within a and b
        for i in p+1..n {
            let alpha = a[i][p] / a[p][p];
            b[i] -= alpha * b[p];
            for j in p..n {
                a[i][j] -= alpha * a[p][j];
            }
        }
    }

    // back substitution
    let mut x: Vec<f64> = vec![0.0; n];

    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in (i + 1)..n {
            sum += a[i][j] * x[j];
        }
        x[i] = (b[i] - sum) / a[i][i];
    }

    Some(x)
}

#[test]
fn test_solve() {
    let a = vec![vec![0.0, 1.0, 1.0], vec![2.0, 4.0, -2.0], vec![0.0, 3.0, 15.0]];
    let b = vec![4.0, 2.0, 36.0];

    assert_eq!(vec![-1.0, 2.0, 2.0], solve(a, b).unwrap());
}

fn calculate_temps() {
    // array sizes
    let n = 3;
    let m = 4;
    // border temperatures
    let top = 50.0;
    let right = 250.0;
    let bot = 150.0;
    let left = 100.0;

    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 1..n { // from left to right
        for j in 1..m { // from bottom to top

            let mut bv = 0.0;

            if i-1 == 0 {
                bv -= left;
            }
            if i+1 == n {
                bv -= right;
            }
            if j-1 == 0 {
                bv -= bot;
            }
            if j+1 == m {
                bv -= top;
            }

            b.push(bv);

            // n*m + n ?
            println!("T({},{}) - 4*T({},{}) + T({},{}) + T({},{}) + T({},{}) = 0 ({})", i-1, j, i, j, i+1, j, i, j-1, i, j+1, bv);

            // lets build matrix row
            // (1,1) (2,1) (1,2) (2,2) (1,3) (2,3)
            let mut row = Vec::new();
            for k in 1..m {
                for l in 1..n {
                    if i-1 == l && j == k {
                        row.push(1.0);
                    }
                    else if i == l && j == k {
                        row.push(-4.0);
                    }
                    else if i+1 == l && j == k {
                        row.push(1.0);
                    }
                    else if i == l && j-1 == k {
                        row.push(1.0);
                    }
                    else if i == l && j+1 == k {
                        row.push(1.0);
                    } else {
                        row.push(0.0);
                    }
                }
            }
            println!("{:?}", row);
            a.push(row);
        }
    }

    println!("{:?}", a);
    println!("{:?}", b);

    println!("{:?}", solve(a, b).unwrap());
}

fn main() {
    calculate_temps();
}
