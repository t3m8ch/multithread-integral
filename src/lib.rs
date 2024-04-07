use std::thread;

fn integral(a: f64, b: f64, delta: f64) -> f64 {
  let mut result: f64 = 0.0;
  let mut x: f64 = a;

  while x < b {
    result += delta * x.sqrt();
    x += delta;
  }

  result
}

fn integral_multithreading(a: i32, b: i32, delta: f64, threads_count: i32) -> f64 {
  let numbers_count = b - a;

  let handles: Vec<_> = (0..threads_count).map(|t| {
    thread::spawn(move || {
      let x = f64::from(a) + f64::from(numbers_count / threads_count * t);
      let y = f64::from(a) + f64::from(numbers_count / threads_count * (t + 1));
      integral(x, y, delta) + if t == 0 { 
        let x = f64::from(a) + f64::from(numbers_count / threads_count * threads_count);
        let y = f64::from(a) + f64::from(numbers_count);
        integral(x, y, delta)
      } else { 0.0 }
    })
  }).collect();

  let mut s = 0.0;
  for h in handles {
    let a = h.join().unwrap();
    s += a;
  } 

  s
}

const A: i32 = 9;
const B: i32 = 36;
const DELTA: f64 = 0.0000001;

#[no_mangle]
pub extern fn process() {
  println!("{}", integral(A.into(), B.into(), DELTA));
}

#[no_mangle]
pub extern fn process_multi(threads_count: i32) {
  println!("{}", integral_multithreading(A, B, DELTA, threads_count));
}
