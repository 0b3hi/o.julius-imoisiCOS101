fn  main() {
	let tf:f64 = 2.0;
	let mf:f64 = 1.0;
	let hf:f64 = 3.0;
	let df:f64 = 3.0;
	let af:f64 = 1.0;

	let tx:f64 = 450000.00;
	let mx:f64 = 1500000.00;
	let hx:f64 = 750000.00;
	let dx:f64 = 2850000.00;
	let ax:f64 = 250000.00;

	//Sum
	let s = (tf * tx) + (mf * mx) + (hf * hx) + (df * dx) + (af * ax);
	println! ("sum is {}", s);
	//Average
	let a = s / (tf + mf + hf + df + af);
	println! ("Average is {}", a);
}