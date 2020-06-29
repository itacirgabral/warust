fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h <= 0.0 || bounce <= 0.0 || bounce >= 1.0 || window >= h {
        -1
    } else {
        let mut i = 0;
        let mut x = h;
        while x > window {
            i = i + 1;
            x = x * bounce;
            if x > window {
                i = i + 1;
            }
        }
        i
    }
}}

fn testequal(h: f64,  bounce: f64,  window: f64, exp: i32) -> () {
    assert_eq!(bouncing_ball(h, bounce, window), exp)
}

#[test]
fn tests_bouncing_ball() {
    testequal(2.0, 0.5, 1.0, 1);
    testequal(3.0, 0.66, 1.5, 3);
    testequal(30.0, 0.66, 1.5, 15);
    testequal(30.0, 0.75, 1.5, 21);
    testequal(30.0, 0.4, 10.0, 3);
    testequal(40.0, 0.4, 10.0, 3);
    testequal(10.0, 0.6, 10.0, -1);
    testequal(40.0, 1.0, 10.0, -1);
    testequal(-5.0, 0.66, 1.5, -1);
    testequal(5.0, -1.0, 1.5, -1);
    testequal(10.5, 0.6, 1.5, 7);
    testequal(46.0, 0.75, 46.00, -1);
    testequal(10.5, 0.6, 1.5, 7);
    testequal(109.5, 0.75, 1.09, 33);
    testequal(12.2, 12.2, 12.2, -1);
    testequal(5.0, 0.83, 5.0, -1);
    testequal(109.0, 0.75, 1.09, 33);
    testequal(3.0, 0.75, 3.0, -1);
    testequal(2.99, 0.75, 3.0, -1);
    testequal(25.8, 0.6, 2.8, 9);
    testequal(57.0, 0.9, 0.57, 87);
    testequal(15.25, 0.6, 1.0, 11);
    testequal(109.0, 0.75, 1.09, 33);
    testequal(12.0, 0.6, 1.5, 9);
    testequal(14.0, 0.24, 1.4, 3);
    testequal(7.5, 0.75, 7.5, -1);
    testequal(15.25, 0.6, 1.0, 11);
    testequal(12.0, 0.6, 1.5, 9);
    testequal(12.2, 12.2, 12.2, -1);
    testequal(67.0, 0.6, 3.0, 13);
    testequal(61.0, 1.1, 6.1, -1);
    testequal(25.8, 0.6, 2.8, 9);
    testequal(110.0, 0.75, 1.09, 33);
    testequal(19.0, 0.75, 1.9, 17);
    testequal(25.8, 0.6, 2.8, 9);
    testequal(41.8, 0.6, 4.8, 9);
    testequal(98.0, 9.0, 9.8, -1);
    testequal(15.9, 1.0, 1.9, -1);
}