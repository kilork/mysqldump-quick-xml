use mysqldump_quick_xml::MysqlDumpQuickXml;

#[derive(Debug, PartialEq, MysqlDumpQuickXml)]
struct Row {
    id: String,
    code: String,
}

#[test]
fn should_parse_xml_from_str() {
    let xml = r##"
<?xml version="1.0"?>
<mysqldump xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<database name="db">
<table_data name="table1">
    <row>
        <field name="id">1</field>
        <field name="code">sample 1</field>
    </row>
    <row>
        <field name="id">2</field>
        <field name="code">sample 2</field>
    </row>
    <row>
        <field name="id">3</field>
        <field name="code" xsi:nil="true" />
    </row>
</table_data>
</database>
</mysqldump>
        "##;

    let rows = Row::from_str(xml);

    assert_eq!(
        rows,
        vec![
            Row {
                id: "1".into(),
                code: "sample 1".into()
            },
            Row {
                id: "2".into(),
                code: "sample 2".into()
            },
            Row {
                id: "3".into(),
                code: "".into()
            }
        ]
    )
}
