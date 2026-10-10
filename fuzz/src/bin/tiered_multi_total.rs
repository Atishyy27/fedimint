use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data| {
            fedimint_fuzz::test_total_amount::<fedimint_mint_common::Note>(data)
        });
    }
}
