fn main() {
	let x = 5;
	let y = 23;
    let count = hitung(x,y);
    println!("Hasil dari {} + {} adalah {}", x, y, count);
}
fn hitung(x: i32,y:i32) -> i32
{
	let count = x+y;

	return count;
}