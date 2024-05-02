#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let n = sp1_zkvm::io::read::<u32>();

    // Write n to public input
    sp1_zkvm::io::commit(&n);

    // Compute the n'th fibonacci number,

    let mut nums = vec![1, 1];

    for _ in 0..n {
        let mut c = nums[nums.len() - 1] + nums[nums.len() - 2];
        c %= 7919;
        nums.push(c);
    }
    // Write the output of the program.

    sp1_zkvm::io::commit(&nums[nums.len() - 2]);
    sp1_zkvm::io::commit(&nums[nums.len() - 1]);
}
