use esp_println::println;
use heapless::Vec;

pub fn be_bytes_to_f64 <
    const FLOATS: usize, 
    >(
    input: [u8; 2*FLOATS],
    conversion_factor: f64
    ) -> [f64; FLOATS] {
    input
        .chunks(2)
        .map(|num| single_be_to_f64([num[0], num[1]]) * conversion_factor)
        .collect::<Vec<f64, FLOATS>>()
        .into_array()
        .unwrap_or({
            println!("conversion failed!!!!");
            [0f64; FLOATS]
        })
}

pub fn single_be_to_f64(bytes: [u8; 2]) -> f64 {
    f64::from(i16::from_be_bytes(bytes))
}
