fn main() {
    // Create a CPU client.
    let client = xla::PjRtClient::cpu()?;

// A builder object is used to store the graph of XlaOp.
    let xla_builder = xla::XlaBuilder::new("test-builder");

// Build a simple graph summing two constants.
    let cst20 = xla_builder.constant_r0(20f32);
    let cst22 = xla_builder.constant_r0(22f32);
    let sum = (cst20 + cst22)?;

// Create a computation from the final node.
    let sum = sum.build()?;

// Compile this computation for the target device and then execute it.
    let result = client.compile(&sum)?;
    let result = &result.execute::<xla::Literal>(&[])?;

// Retrieve the resulting value.
    let result = result[0][0].to_literal_sync()?.to_vec::<f32>()?;
}
