#[test]
fn debug_naming() {
    use xsd_parser::{Config, generate};
    use xsd_parser::config::Schema;
    
    let schema = r#"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema">
  <xsd:complexType name="Test_">
    <xsd:sequence>
      <xsd:element name="value" type="xsd:string"/>
    </xsd:sequence>
  </xsd:complexType>
  
  <xsd:complexType name="Test">
    <xsd:sequence>
      <xsd:element name="value" type="xsd:string"/>
    </xsd:sequence>
  </xsd:complexType>
</xsd:schema>"#;
    
    std::fs::write("/tmp/test_naming.xsd", schema).unwrap();
    
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("/tmp/test_naming.xsd".into())];
    
    let code = generate(config).unwrap();
    let code_str = code.to_string();
    
    println!("Generated code:\n{}", code_str);
    
    assert!(code_str.contains("Test_Type") || code_str.contains("Test_"), "Should contain Test_ type");
    assert!(code_str.contains("TestType") || code_str.contains("pub struct Test "), "Should contain Test type");
}
