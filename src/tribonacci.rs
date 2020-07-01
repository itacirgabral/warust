fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    println!("len = :{}", signature.len());
    println!("n = ${}", n);
    if n < 3 {
      signature[0..n].to_vec()
    } else {
      let mut tri = signature.to_vec();
      for _i in 3..n {
        let minus1 = tri[tri.len() - 1];
        let minus2 = tri[tri.len() - 2];
        let minus3 = tri[tri.len() - 3];
        tri.push(minus1 + minus2 + minus3);
      }
      tri
    }
}

#[test]
fn basic_tests() {
    assert_eq!(tribonacci(&[0., 1., 1.], 10), vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
    assert_eq!(tribonacci(&[1., 0., 0.], 10), vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]);
    assert_eq!(tribonacci(&[0., 0., 0.], 10), vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
    assert_eq!(tribonacci(&[1., 2., 3.], 10), vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]);
    assert_eq!(tribonacci(&[3., 2., 1.], 10), vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]);
    assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
    assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
    assert_eq!(tribonacci(&[0.5, 0.5, 0.5], 30), vec![0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5]);
}