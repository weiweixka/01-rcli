use crate::{Base64Format, get_reader};
use base64::{Engine as _, engine::general_purpose::*};

pub fn process_encode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    //根据格式选择编码引擎
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> anyhow::Result<()> {
    // Base64解码逻辑
    let mut reader = get_reader(input)?;

    //读取输入数据
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim_end(); //去除末尾的换行符

    //根据格式选择解码引擎
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    //输出解码结果
    //TODO:decoded可能不是utf8格式，需要处理
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}
