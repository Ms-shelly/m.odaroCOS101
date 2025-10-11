fn main() {
	// price of items
	let toshiba = 450000.00;
	let mac = 1500000.00;
	let hp = 750000.00;
	let dell = 2850000.00;
	let acer = 250000.00;

	// quantity of items
	let toshiba_qty = 2.0;
	let mac_qty = 1.0;
	let hp_qty = 3.0;
	let dell_qty = 3.0;
	let acer_qty = 1.0;

	// total quantity for each item
	let toshiba_total_qty = toshiba * toshiba_qty;
	let mac_total_qty = mac * mac_qty;
	let hp_total_qty = hp * hp_qty;
	let dell_total_qty = dell * dell_qty;
	let acer_total_qty = acer * acer_qty;

	// formula for sum, total quantity and average
	let sum = toshiba_total_qty + mac_total_qty + hp_total_qty + dell_total_qty + acer_total_qty;
	let total_qty = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
	let average = sum / total_qty;

	println!("Sum is {} and Average is {}", sum,average);

}