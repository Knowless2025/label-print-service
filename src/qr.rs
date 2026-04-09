use qrcode::QrCode;
use base64::{engine::general_purpose, Engine as _};

/// สร้าง QR Code Format: ARKR-{ItemId}-{BatchNo}-{Running}-{Side}-{ProdId}
/// ตัวอย่าง: ARKR-7113800055-260202G71-0001-LH-P992383
pub fn generate_qr_data(
    item_id: &str,
    batch_no: &str,
    running_no: &str,
    side: &str,
    prod_id: &str,
) -> String {
    format!("ARKR-{}-{}-{}-{}-{}", item_id, batch_no, running_no, side, prod_id)
}

/// สร้าง QR Code เป็น SVG String (สำหรับ Web UI)
pub fn generate_qr_svg(data: &str) -> Result<String, String> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| format!("QR generation failed: {}", e))?;
    
    let svg = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(qrcode::render::svg::Color("#000000"))
        .light_color(qrcode::render::svg::Color("#ffffff"))
        .build();
    
    Ok(svg)
}

/// สร้าง QR Code เป็น Base64 SVG (สำหรับส่งให้ Web UI)
pub fn generate_qr_base64_svg(data: &str) -> Result<String, String> {
    let svg = generate_qr_svg(data)?;
    Ok(general_purpose::STANDARD.encode(svg.as_bytes()))
}

/// สร้าง QR Code เป็น ASCII Art (สำหรับ Debug)
pub fn generate_qr_ascii(data: &str) -> Result<String, String> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| format!("QR generation failed: {}", e))?;
    
    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    
    Ok(string)
}

/// สร้าง QR Code Binary Data (สำหรับ ZPL)
/// Returns: width, height, และ binary data
pub fn generate_qr_binary(data: &str) -> Result<(usize, usize, Vec<u8>), String> {
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| format!("QR generation failed: {}", e))?;
    
    let colors = code.to_colors();
    let width = code.width();
    
    // แปลงเป็น binary (0 = white, 1 = black)
    let mut binary_data = Vec::new();
    for color in colors {
        binary_data.push(match color {
            qrcode::Color::Dark => 1,
            qrcode::Color::Light => 0,
        });
    }
    
    Ok((width, width, binary_data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_data_format() {
        let qr = generate_qr_data(
            "7113800055",
            "260202G71",
            "0001",
            "LH",
            "P992383"
        );
        assert_eq!(qr, "ARKR-7113800055-260202G71-0001-LH-P992383");
    }

    #[test]
    fn test_qr_svg_generation() {
        let data = "ARKR-7113800055-260202G71-0001-LH-P992383";
        let result = generate_qr_svg(data);
        assert!(result.is_ok());
        
        let svg = result.unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_qr_base64_svg() {
        let data = "ARKR-TEST-001";
        let result = generate_qr_base64_svg(data);
        assert!(result.is_ok());
        
        let base64 = result.unwrap();
        assert!(base64.len() > 0);
    }

    #[test]
    fn test_qr_ascii() {
        let data = "ARKR-TEST";
        let result = generate_qr_ascii(data);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("█"));
    }

    #[test]
    fn test_qr_binary() {
        let data = "ARKR-TEST";
        let result = generate_qr_binary(data);
        assert!(result.is_ok());
        
        let (width, height, binary) = result.unwrap();
        assert_eq!(width, height);
        assert_eq!(binary.len(), width * height);
        assert!(binary.iter().all(|&b| b == 0 || b == 1));
    }
}