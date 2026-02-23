use crate::LabelRequest;

pub fn generate_zpl(label: &LabelRequest) -> String {
    let customer = &label.customer_code;
    let brand = &label.brand;
    let side = &label.side;
    let part_no = &label.part_no;
    let serial = &label.serial_no;
    let barcode = &label.barcode_main;
    let barcode_sub = label.barcode_sub.as_deref().unwrap_or("");

    format!(
r#"^XA
^CI28
^PW400
^LL300

^CF0,30
^FO20,20^FD{customer}^FS

^CF0,28
^FO20,60^FD{brand}^FS
^FO300,60^FD{side}^FS

^CF0,26
^FO20,100^FD{part_no} {serial}^FS

^BY2,2,60
^FO20,140^BCN,60,Y,N,N
^FD{barcode}^FS

^CF0,22
^FO20,220^FD{barcode} [{barcode_sub}]^FS

^XZ"#
    )
}
