use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

// 定義操作結構
#[derive(Serialize, Deserialize, Debug)]
struct Operation {
    dependencies: Vec<String>,
    opId: OperationId,
    output: Output,
}

#[derive(Serialize, Deserialize, Debug)]
struct OperationId {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {
    fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Field {
    field: String,
    #[serde(rename = "type")]
    field_type: String,
}

fn main() {
    // 模擬 graph_data 的 JSON 字串
    let graph_data = r#"
    {"isParse":1,"operations":[{"dependencies":["38e66737-83a9-4db8-a040-a7d9730c5add"],"error":[],"opId":{"id":"6ace790d-baac-4be8-b603-6376d6fb2cdd","name":"輸出節點"},"output":{"fields":[{"encrypt":0,"field":"理專分行代碼","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"INTEGER","zh":"理專分行代碼"},{"encrypt":0,"field":"CIF_ID","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"CIF_ID"},{"encrypt":0,"field":"CUST_NAME","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"CUST_NAME"},{"encrypt":0,"field":"理專員工編號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專員工編號"},{"encrypt":0,"field":"理專員工姓名","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專員工姓名"},{"encrypt":0,"field":"規則編號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"規則編號"},{"encrypt":0,"field":"TIME","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"TIME"},{"encrypt":0,"field":"理專身分證字號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專身分證字號"},{"encrypt":0,"field":"DATE","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"DATE"},{"encrypt":0,"field":"TIMES_OF_1WEEK","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"INTEGER","zh":"TIMES_OF_1WEEK"},{"encrypt":0,"field":"CUR_DT","isPicture":0,"isPrimaryKey":1,"mask":0,"type":"TIMESTAMP","zh":"CUR_DT"},{"encrypt":0,"field":"AMT","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"DOUBLE","zh":"AMT"},{"encrypt":0,"field":"event_id","isPicture":0,"isPrimaryKey":1,"mask":0,"type":"VARCHAR","zh":"event_id"}]},"type":"output","value":{"refreshMode":"overwrite","fields":[{"field":"CIF_ID","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"CIF_ID","mask":0},{"field":"CUST_NAME","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"CUST_NAME","mask":0},{"field":"DATE","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"DATE","mask":0},{"field":"event_id","isPicture":0,"encrypt":0,"isPrimaryKey":1,"type":"VARCHAR","zh":"event_id","mask":0},{"field":"TIME","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"TIME","mask":0},{"field":"TIMES_OF_1WEEK","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"INTEGER","zh":"TIMES_OF_1WEEK","mask":0},{"field":"理專分行代碼","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"INTEGER","zh":"理專分行代碼","mask":0},{"field":"理專身分證字號","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"理專身分證字號","mask":0},{"field":"理專員工姓名","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"理專員工姓名","mask":0},{"field":"理專員工編號","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"理專員工編號","mask":0},{"field":"規則編號","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"VARCHAR","zh":"規則編號","mask":0},{"field":"CUR_DT","isPicture":0,"encrypt":0,"isPrimaryKey":1,"type":"TIMESTAMP","zh":"CUR_DT","mask":0},{"field":"AMT","isPicture":0,"encrypt":0,"isPrimaryKey":0,"type":"DOUBLE","zh":"AMT","mask":0}],"tableName":""},"x":830,"y":200},{"dependencies":[],"error":[],"opId":{"id":"2420cd6c-8596-4999-8269-63e30d22fcc3","name":"表輸入"},"output":{"fields":[{"encrypt":0,"field":"CIF_ID","isPicture":0,"isPrimaryKey":1,"mask":0,"type":"VARCHAR","zh":"CIF_ID"},{"encrypt":0,"field":"CUST_NAME","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"CUST_NAME"},{"encrypt":0,"field":"DATE","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"DATE"},{"encrypt":0,"field":"event_id","isPicture":0,"isPrimaryKey":1,"mask":0,"type":"VARCHAR","zh":"event_id"},{"encrypt":0,"field":"TIME","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"TIME"},{"encrypt":0,"field":"TIMES_OF_1WEEK","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"INTEGER","zh":"TIMES_OF_1WEEK"},{"encrypt":0,"field":"理專分行代碼","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"INTEGER","zh":"理專分行代碼"},{"encrypt":0,"field":"理專身分證字號","isPicture":0,"isPrimaryKey":1,"mask":0,"type":"VARCHAR","zh":"理專身分證字號"},{"encrypt":0,"field":"理專員工姓名","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專員工姓名"},{"encrypt":0,"field":"理專員工編號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專員工編號"},{"encrypt":0,"field":"規則編號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"規則編號"}]},"type":"input","value":{"fields":[{"sortNo":0,"createBy":"admin","field":"CIF_ID","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":1,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"CIF_ID","mask":0},{"sortNo":1,"createBy":"admin","field":"CUST_NAME","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"CUST_NAME","mask":0},{"sortNo":2,"createBy":"admin","field":"DATE","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"DATE","mask":0},{"sortNo":3,"createBy":"admin","field":"event_id","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":1,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"event_id","mask":0},{"sortNo":4,"createBy":"admin","field":"TIME","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"TIME","mask":0},{"sortNo":5,"createBy":"admin","field":"TIMES_OF_1WEEK","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"INTEGER","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"TIMES_OF_1WEEK","mask":0},{"sortNo":6,"createBy":"admin","field":"理專分行代碼","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"INTEGER","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"理專分行代碼","mask":0},{"sortNo":7,"createBy":"admin","field":"理專身分證字號","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":1,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"理專身分證字號","mask":0},{"sortNo":8,"createBy":"admin","field":"理專員工姓名","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"理專員工姓名","mask":0},{"sortNo":9,"createBy":"admin","field":"理專員工編號","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"理專員工編號","mask":0},{"sortNo":10,"createBy":"admin","field":"規則編號","createTime":"2024-06-26 09:37:23","updateBy":"admin","encrypt":0,"updateTime":"2024-06-26 09:37:23","isPrimaryKey":0,"type":"VARCHAR","dataTableBizId":"066c9589-0cd8-440c-8dc7-be582a585211","zh":"規則編號","mask":0}],"tableName":{"name":"tbl_RULE_121","remark":"RULE_121"},"tableNameLabel":""},"x":320,"y":180},{"dependencies":["2420cd6c-8596-4999-8269-63e30d22fcc3"],"error":[],"opId":{"id":"a784d831-5d81-4f58-afe1-e9d414519344","name":"篩選條件"},"output":{"fields":[{"encrypt":0,"field":"CIF_ID","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"CIF_ID"},{"encrypt":0,"field":"CUST_NAME","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"CUST_NAME"},{"encrypt":0,"field":"DATE","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"DATE"},{"encrypt":0,"field":"event_id","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"event_id"},{"encrypt":0,"field":"TIME","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"TIME"},{"encrypt":0,"field":"TIMES_OF_1WEEK","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"INTEGER","zh":"TIMES_OF_1WEEK"},{"encrypt":0,"field":"理專分行代碼","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"INTEGER","zh":"理專分行代碼"},{"encrypt":0,"field":"理專身分證字號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專身分證字號"},{"encrypt":0,"field":"理專員工姓名","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專員工姓名"},{"encrypt":0,"field":"理專員工編號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"理專員工編號"},{"encrypt":0,"field":"規則編號","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"VARCHAR","zh":"規則編號"}]},"type":"filter","value":{"filters":{"type":"filter","value":[{"comparator":"=","rules":[[{"field":"理專分行代碼","tableRemark":"表輸入","fieldRemark":"理專分行代碼","type":"model","table":"2420cd6c-8596-4999-8269-63e30d22fcc3"}],[{"type":"input","value":"11","contentType":"string"}]],"type":"expression"}],"sql":"(表輸入.理專分行代碼 = '11' )"}},"x":520,"y":190},{"dependencies":["a784d831-5d81-4f58-afe1-e9d414519344"],"error":[],"opId":{"id":"38e66737-83a9-4db8-a040-a7d9730c5add","name":"新增欄位"},"output":{"fields":[{"encrypt":0,"field":"CIF_ID","isPicture":0,"mask":0,"type":"VARCHAR","zh":"CIF_ID"},{"encrypt":0,"field":"CUST_NAME","isPicture":0,"mask":0,"type":"VARCHAR","zh":"CUST_NAME"},{"encrypt":0,"field":"DATE","isPicture":0,"mask":0,"type":"VARCHAR","zh":"DATE"},{"encrypt":0,"field":"event_id","isPicture":0,"mask":0,"type":"VARCHAR","zh":"event_id"},{"encrypt":0,"field":"TIME","isPicture":0,"mask":0,"type":"VARCHAR","zh":"TIME"},{"encrypt":0,"field":"TIMES_OF_1WEEK","isPicture":0,"mask":0,"type":"INTEGER","zh":"TIMES_OF_1WEEK"},{"encrypt":0,"field":"理專分行代碼","isPicture":0,"mask":0,"type":"INTEGER","zh":"理專分行代碼"},{"encrypt":0,"field":"理專身分證字號","isPicture":0,"mask":0,"type":"VARCHAR","zh":"理專身分證字號"},{"encrypt":0,"field":"理專員工姓名","isPicture":0,"mask":0,"type":"VARCHAR","zh":"理專員工姓名"},{"encrypt":0,"field":"理專員工編號","isPicture":0,"mask":0,"type":"VARCHAR","zh":"理專員工編號"},{"encrypt":0,"field":"規則編號","isPicture":0,"mask":0,"type":"VARCHAR","zh":"規則編號"},{"encrypt":0,"field":"CUR_DT","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"TIMESTAMP","zh":"CUR_DT"},{"encrypt":0,"field":"AMT","isPicture":0,"isPrimaryKey":0,"mask":0,"type":"DECIMAL","zh":"AMT"}]},"type":"addfield","value":{"addFields":[{"as":{"field":"CUR_DT","type":"VARCHAR","zh":"CUR_DT"},"rules":{"type":"filedEdit","value":[{"name":"curtime","label":"curtime","type":"function","value":[]}]},"tabKey":0.2428769872774066},{"as":{"field":"AMT","type":"VARCHAR","zh":"AMT"},"rules":{"type":"filedEdit","value":[{"type":"input","value":"1000000.08","contentType":"number"}]},"tabKey":0.4101117838078481}]},"x":680,"y":190}]}
    "#;

    // 解析 JSON
    let data: Value = serde_json::from_str(graph_data).expect("JSON 解析失敗");
    let operations_json = data["operations"].clone();

    // 將 JSON 轉換為操作列表
    let operations: Vec<Operation> = serde_json::from_value(operations_json).expect("操作解析失敗");

    // 建立一個 HashMap 來追蹤操作
    let mut operations_map: HashMap<String, Operation> = HashMap::new();
    for op in &operations {
        operations_map.insert(op.opId.id.clone(), op.clone());
    }

    // 建立依賴關係
    let mut visited: HashSet<String> = HashSet::new();
    let mut sorted_operations: Vec<Operation> = Vec::new();

    fn list_dependencies_recursively(
        operation_id: &String,
        operations_map: &HashMap<String, Operation>,
        visited: &mut HashSet<String>,
        sorted_operations: &mut Vec<Operation>,
    ) {
        if visited.contains(operation_id) {
            return;
        }
        visited.insert(operation_id.clone());
        let operation = operations_map.get(operation_id).unwrap();
        for dep in &operation.dependencies {
            list_dependencies_recursively(dep, operations_map, visited, sorted_operations);
        }
        sorted_operations.push(operation.clone());
    }

    // 處理所有操作
    for op in &operations {
        list_dependencies_recursively(&op.opId.id, &operations_map, &mut visited, &mut sorted_operations);
    }

    // 輸出每個步驟及其設置
    for (index, operation) in sorted_operations.iter().enumerate() {
        println!("步驟 {}: {} (ID: {})", index + 1, operation.opId.name, operation.opId.id);
        println!("  設置:");
        for field in &operation.output.fields {
            println!("    - 欄位: {}, 類型: {}", field.field, field.field_type);
        }
        println!();
    }
}