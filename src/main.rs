use pool::Pool;

mod pool;
fn main() {
    let p = Pool::new(4);
    p.execute(|| println!("do 1 "));
    p.execute(|| println!("do 2"));
    p.execute(|| println!("do 3"));
    p.execute(|| println!("do 4"));
}
