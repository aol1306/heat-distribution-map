static EPSILON: f64 = 0.00000000001;
static SIZE: usize = 40;

fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();

    for p in 0..n {
        // find pivot row and swap
        let mut max = p;
        for i in p + 1..n {
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
        for i in p + 1..n {
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

fn calculate_temps(top: f64, right: f64, bot: f64, left: f64, n: i32, m: i32) -> Option<Vec<f64>> {
    /*// array sizes
    let n = 3;
    let m = 4;
    // border temperatures
    let top = 50.0;
    let right = 250.0;
    let bot = 150.0;
    let left = 100.0;*/

    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 1..n { // from left to right
        for j in 1..m { // from bottom to top

            let mut bv = 0.0;

            if i - 1 == 0 {
                bv -= left;
            }
            if i + 1 == n {
                bv -= right;
            }
            if j - 1 == 0 {
                bv -= bot;
            }
            if j + 1 == m {
                bv -= top;
            }

            b.push(bv);

            //println!("T({},{}) - 4*T({},{}) + T({},{}) + T({},{}) + T({},{}) = 0 ({})", i - 1, j, i, j, i + 1, j, i, j - 1, i, j + 1, bv);

            // lets build matrix row
            // (1,1) (2,1) (1,2) (2,2) (1,3) (2,3)
            let mut row = Vec::new();
            for k in 1..m {
                for l in 1..n {
                    if i - 1 == l && j == k {
                        row.push(1.0);
                    } else if i == l && j == k {
                        row.push(-4.0);
                    } else if i + 1 == l && j == k {
                        row.push(1.0);
                    } else if i == l && j - 1 == k {
                        row.push(1.0);
                    } else if i == l && j + 1 == k {
                        row.push(1.0);
                    } else {
                        row.push(0.0);
                    }
                }
            }
            //println!("{:?}", row);
            a.push(row);
        }
    }

    //println!("{:?}", a);
    //println!("{:?}", b);

    solve(a, b)
}

fn render_svg(temps: Vec<f64>, n: usize, m: usize) {
    let mut svg = format!("<svg viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">", n * SIZE, m * SIZE);
    for i in 0..n - 1 {
        for j in 0..m - 1 {
            svg.push_str(format!("<rect x=\"{}\" y=\"{}\" height=\"{}\" style=\"fill:hsl({},100%,50%)\" width=\"{}\" />", (j+1) * SIZE, (n-1) * SIZE - i * SIZE, SIZE, 240.0-temps[i * (n - 1) + j], SIZE).as_str());
            svg.push_str(format!("<text x=\"{}\" y=\"{}\">{}</text>", (j+1) * SIZE, (n-1) * SIZE - i * SIZE + SIZE / 2, temps[i * (n - 1) + j].round()).as_str());
        }
    }
    svg.push_str("</svg>");

    std::fs::write("image.svg", svg).expect("Unable to write data");

    println!("SVG written to image.svg");
}

fn main() {
    let top = 50.0;
    let right = 250.0;
    let bot = 150.0;
    let left = 100.0;
    let n = 21;//3
    let m = 21;//4

    println!("Start simulation");
    let temps = calculate_temps(top, right, bot, left, n, m).unwrap();
    println!("{:?}", temps);

    render_svg(temps, n as usize, m as usize);
}
