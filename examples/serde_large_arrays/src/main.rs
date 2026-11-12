use std::error::Error;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() -> Result<(), Box<dyn Error>> {
    let measurements = MeasurementsType {
        sample: std::array::from_fn(|index| SampleType {
            label: format!("sample-{index}"),
            value: 0.5,
        }),
    };
    let json = serde_json::to_string(&measurements)?;
    let decoded: Measurements = serde_json::from_str(&json)?;
    assert_eq!(decoded, measurements);
    println!("Round-tripped {} samples as JSON.", decoded.sample.len());
    Ok(())
}
