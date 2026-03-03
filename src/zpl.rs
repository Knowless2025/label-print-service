use crate::models::LabelRequest;

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
^LL260

^CF0,24
^FO20,20^FD{customer}^FS
^CF0,32
^FO330,15^FD{side}^FS

^CF0,26
^FO20,55^FD{brand}^FS

^CF0,24
^FO20,90^FD{part_no}^FS
^CF0,26
^FO330,90^FD{serial}^FS

^BY2,2,40
^FO20,135^BCN,40,N,N,N
^FD{barcode}^FS

^CF0,20
^FO20,185^FD{barcode} [ {barcode_sub} ]^FS

^XZ"#
    )
}