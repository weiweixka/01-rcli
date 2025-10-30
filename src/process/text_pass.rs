use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use std::{fs, io::Read};

use crate::{TextSignFormat, get_reader};
use anyhow::{Ok, Result};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use blake3;

/// 文本签名和验证模块
/// 支持 Blake3 和 Ed25519 两种签名算法
/// Blake3: 基于哈希的消息认证码算法
/// Ed25519: 基于椭圆曲线的数字签名算法

/// 文本签名 trait
/// 定义签名操作的统一接口
trait TextSign {
    /// 对输入数据进行签名
    /// # 参数
    /// - reader: 可读数据流
    /// # 返回
    /// - 签名结果的字节向量
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

/// 文本验证 trait
/// 定义验证操作的统一接口
trait TextVerify {
    /// 验证签名是否有效
    /// # 参数
    /// - reader: 可读数据流
    /// - sig: 待验证的签名数据
    /// # 返回
    /// - 验证结果 (true: 有效, false: 无效)
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}
/// Blake3 签名器结构体
/// 使用 Blake3 算法进行签名和验证
struct Blake3 {
    key: [u8; 32], // 32字节的密钥
}

/// Ed25519 签名器结构体
/// 使用 Ed25519 算法进行签名
struct Ed25519Signer {
    key: SigningKey, // Ed25519 签名密钥
}

/// Ed25519 验证器结构体
/// 使用 Ed25519 算法进行验证
struct Ed25519Verifier {
    key: VerifyingKey, // Ed25519 验证密钥
}

/// 处理文本签名的主函数
/// # 参数
/// - input: 输入文件路径或 "-" 表示标准输入
/// - key: 密钥文件路径
/// - format: 签名格式 (Blake3 或 Ed25519)
/// # 返回
/// - 成功时返回 Ok(())，失败时返回错误
pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    // 获取输入读取器
    let mut reader = get_reader(input)?;

    // 根据签名格式选择相应的签名算法
    let signed = match format {
        TextSignFormat::Blake3 => {
            // 读取 Blake3 密钥文件
            let key_data = fs::read(key)?;
            // 将密钥转换为32字节数组
            let key_bytes: [u8; 32] = key_data[..32].try_into()?;
            // 创建 Blake3 签名器
            let signer = Blake3 { key: key_bytes };
            // 执行签名操作
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            // 读取 Ed25519 密钥文件
            let key_data = fs::read(key)?;
            // 将密钥转换为32字节数组
            let key_array: [u8; 32] = key_data
                .try_into()
                .map_err(|_| anyhow::anyhow!("密钥长度必须是32字节"))?;
            // 从字节数组创建签名密钥
            let signing_key = SigningKey::from_bytes(&key_array);
            // 创建 Ed25519 签名器
            let signer = Ed25519Signer { key: signing_key };
            // 执行签名操作
            signer.sign(&mut reader)?
        }
    };

    // 将签名结果进行 Base64 编码并输出
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);
    Ok(())
}

/// Blake3 签名器的 TextSign trait 实现
impl TextSign for Blake3 {
    /// 使用 Blake3 算法对数据进行签名
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // 读取所有输入数据到缓冲区
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 使用 Blake3 密钥哈希算法生成签名
        let hash = blake3::keyed_hash(&self.key, &buf);

        // 返回哈希结果的字节向量
        Ok(hash.as_bytes().to_vec())
    }
}

/// Ed25519 签名器的 TextSign trait 实现
impl TextSign for Ed25519Signer {
    /// 使用 Ed25519 算法对数据进行签名
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // 读取所有输入数据到缓冲区
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 使用 Ed25519 算法生成签名
        let sig = self.key.sign(&buf);

        // 返回签名结果的字节向量
        Ok(sig.to_bytes().to_vec())
    }
}

/// Blake3 验证器的 TextVerify trait 实现
impl TextVerify for Blake3 {
    /// 使用 Blake3 算法验证签名
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        // 读取所有输入数据到缓冲区
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 使用相同的密钥重新计算哈希值
        let hash = blake3::keyed_hash(&self.key, &buf);

        // 比较计算出的哈希值与提供的签名是否一致
        Ok(hash.as_bytes() == sig)
    }
}

/// Ed25519 验证器的 TextVerify trait 实现
impl TextVerify for Ed25519Verifier {
    /// 使用 Ed25519 算法验证签名
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        // 读取所有输入数据到缓冲区
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // 从字节数组创建签名对象
        let signature = Signature::from_bytes(sig.try_into()?);

        // 使用公钥验证签名
        let ret = self.key.verify(&buf, &signature).is_ok();

        // 返回验证结果
        Ok(ret)
    }
}

/// 处理文本验证的主函数
/// # 参数
/// - input: 输入文件路径或 "-" 表示标准输入
/// - key: 密钥文件路径
/// - signature: 签名文件路径
/// - format: 签名格式 (Blake3 或 Ed25519)
/// # 返回
/// - 成功时返回 Ok(())，失败时返回错误
pub fn process_text_verify(
    input: &str,
    key: &str,
    signature: &str,
    format: TextSignFormat,
) -> Result<()> {
    // 获取输入读取器
    let mut reader = get_reader(input)?;

    // 读取签名文件并解码 Base64 编码
    let signature_data = fs::read(signature)?;
    let signature_bytes = URL_SAFE_NO_PAD.decode(&signature_data)?;

    // 根据签名格式选择相应的验证算法
    let verified = match format {
        TextSignFormat::Blake3 => {
            // 读取 Blake3 密钥文件
            let key_data = fs::read(key)?;
            // 将密钥转换为32字节数组
            let key_bytes: [u8; 32] = key_data[..32].try_into()?;
            // 创建 Blake3 验证器
            let verifier = Blake3 { key: key_bytes };
            // 执行验证操作
            verifier.verify(&mut reader, &signature_bytes)?
        }
        TextSignFormat::Ed25519 => {
            // 读取 Ed25519 公钥文件
            let key_data = fs::read(key)?;
            // 将公钥转换为32字节数组
            let key_array: [u8; 32] = key_data
                .try_into()
                .map_err(|_| anyhow::anyhow!("公钥长度必须是32字节"))?;
            // 从字节数组创建验证密钥
            let verifying_key = VerifyingKey::from_bytes(&key_array)?;
            // 创建 Ed25519 验证器
            let verifier = Ed25519Verifier { key: verifying_key };
            // 执行验证操作
            verifier.verify(&mut reader, &signature_bytes)?
        }
    };

    // 输出验证结果
    if verified {
        println!("签名验证成功");
    } else {
        println!("签名验证失败");
    }
    Ok(())
}

impl Blake3 {
    /// 创建一个新的 Blake3 签名器
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key_bytes: [u8; 32] = key[..32].try_into()?;
        Ok(Self { key: key_bytes })
    }
}
