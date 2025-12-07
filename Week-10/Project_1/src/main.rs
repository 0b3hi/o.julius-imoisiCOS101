struct LaptopPrices {
    hp: u32,
    ibm: u32,
    toshiba: u32,
    dell: u32,
}

fn total_cost(prices: &LaptopPrices, qty: u32)-> u32{
    (prices.hp * qty)
        + (prices.ibm * qty)
        + (prices.toshiba * qty)
        + (prices.dell * qty)
}

fn main() {
    let prices = LaptopPrices{
        hp: 650_000,
        ibm:755_000,
        toshiba: 550_000,
        dell: 850_000,
    };

    let qty = 3;

    let total = total_cost(&prices, qty);
    println!("Total cost is ${}", total);
}
