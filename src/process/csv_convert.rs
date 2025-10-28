use crate::cliopts::OutputFormat;
use csv::Reader;
//use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

// //定义与CSV文件对应的结构体(数据表头信息)
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// struct Player {
//     name: String,
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }

//处理CSV文件并转换为JSON格式
pub fn process_csv(input: &str, output: String, _format: OutputFormat) -> anyhow::Result<()> {
    //读取CSV文件
    let mut reader = Reader::from_path(input)?;
    //反序列化CSV记录为Player结构体
    let mut ret = Vec::with_capacity(128);
    //获取表头
    let headers = reader.headers()?.clone();
    //遍历每一条记录
    for result in reader.records() {
        //将记录转换为Player结构体
        let record = result?;
        //创建JSON对象
        //header.iter().zip(record.iter()) 创建一个迭代器，将表头和记录对应起来
        //zip 方法将两个迭代器组合成一个新的迭代器，生成 (header, record) 对
        //collect 方法将这些对收集到一个 serde_json::Value 对象中
        let json_value: Value = headers.iter().zip(record.iter()).collect::<Value>();
        //将记录添加到结果向量中
        ret.push(json_value);
    }

    //将结果向量序列化为JSON格式并写入输出文件
    let content = match _format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    //写入文件
    fs::write(output, content)?;
    Ok(())
}
