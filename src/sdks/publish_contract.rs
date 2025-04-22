use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;
use sui_move_build::BuildConfig;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;

const PACKAGE_PATH: &str = "/data/suiz/new_coins";

pub struct NewCoinParams {
    decimals: u8,
    symbol: String,
    name: String,
    description: String,
    initial_supply: u64,
    freeze_authority: String,
    mint_authority: String,
}

pub struct TokenParams {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub description: String,
    pub initial_supply: u64,
    pub freeze_authority: String,
    pub mint_authority: String,
    pub url: String,
}

pub struct ContractPublisher {
    template_path: PathBuf,
    output_path: PathBuf,
}

impl ContractPublisher {
    pub fn new(template_path_str: &str, output_path_str: &str) -> Self {
        let template_path = PathBuf::from(template_path_str);
        let output_path = PathBuf::from(output_path_str);
        Self {
            template_path,
            output_path,
        }
    }

    pub fn publish_token(&self, params: TokenParams) -> Result<String> {
        fs::create_dir_all(&self.output_path)?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        // 创建新的项目目录
        let project_name = format!("{}_{}", params.symbol.to_lowercase(), timestamp);
        let new_project_path = Path::new(&self.output_path).join(&project_name);
        
        // 复制模板目录
        self.copy_template(&self.template_path, &new_project_path)?;
        
        // 替换文件内容
        self.replace_token_info(&new_project_path, &params)?;
        
        Ok(project_name)
    }

    fn copy_template(&self, src: &Path, dst: &Path) -> Result<()> {
        if dst.exists() {
            fs::remove_dir_all(dst)?;
        }
        
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let new_path = dst.join(path.file_name().unwrap());
                self.copy_template(&path, &new_path)?;
            } else {
                let new_path = dst.join(path.file_name().unwrap());
                fs::copy(&path, &new_path)?;
            }
        }
        
        Ok(())
    }

    fn replace_token_info(&self, project_path: &Path, params: &TokenParams) -> Result<()> {
        // 处理 Move.toml
        let move_toml_path = project_path.join("Move.toml");
        if move_toml_path.exists() {
            let mut content = fs::read_to_string(&move_toml_path)?;
            // 替换包名中的 SYMBOL
            content = content.replace("SYMBOL", &params.symbol);
            // 替换地址部分
            let lower_symbol = params.symbol.to_lowercase();
            content = content.replace("symbol = \"0x0\"", &format!("{} = \"0x0\"", lower_symbol));
            fs::write(&move_toml_path, content)?;
        }

        // 处理 coin.move
        let sources_path = project_path.join("sources");
        let coin_move_path = sources_path.join("coin.move");
        
        if coin_move_path.exists() {
            let mut content = fs::read_to_string(&coin_move_path)?;
            
            // 替换模块名中的 symbol
            let lower_symbol = params.symbol.to_lowercase();
            content = content.replace("symbol::symbol", &format!("{}::{}", lower_symbol, lower_symbol));
            
            // 替换代币结构体名称
            content = content.replace("SYMBOL", &params.symbol);
            
            // 替换代币信息
            content = content.replace("DECIMALS", &params.decimals.to_string());
            content = content.replace("SYMBOL", &params.symbol);
            content = content.replace("NAME", &params.name);
            content = content.replace("DESCRIPTION", &params.description);
            content = content.replace("SUPPLY", &params.initial_supply.to_string());
            
            // 替换权限地址
            content = content.replace("COIN_RECIPIENT", &params.mint_authority);
            content = content.replace("CAP_RECIPIENT", &params.freeze_authority);
            
            // 替换图标 URL
            content = content.replace("ICON", &params.url);
            
            fs::write(&coin_move_path, content)?;
        }
        
        Ok(())
    }
}

pub fn publish_contract(_ptb: &mut ProgrammableTransactionBuilder, path_str: &str) -> Result<(), anyhow::Error> {
    let mut path = PathBuf::from(path_str);
    let compiled_package = BuildConfig::new_for_testing().build(&path)?;
    let compiled_modules_bytes = compiled_package
        .get_package_base64(false)
        .into_iter()
        .map(|b| b.to_vec().unwrap())
        .collect::<Vec<_>>();
    let dependencies = compiled_package.get_dependency_storage_package_ids();

    let mut builder = ProgrammableTransactionBuilder::new();
    builder.publish_immutable(compiled_modules_bytes, dependencies);
    Ok(())
}

// 使用示例
#[cfg(test)]
mod tests {
    use sui_sdk::{SuiClient, SuiClientBuilder};
    use std::str::FromStr;
    use sui_types::{base_types::SuiAddress, transaction::{TransactionData, TransactionKind}};

    use super::*;

    #[tokio::test]
    async fn test_publish_token() {
        let publisher = ContractPublisher::new(
            "/home/ccbond/suiz_data/new_coin",
            "/home/ccbond/suiz_data/published",
        );

        let params = TokenParams {
            name: "My Token".to_string(),
            symbol: "MTK".to_string(),
            decimals: 9,
            description: "MIU, the meme cat coin on the SUI Network".to_string(),
            initial_supply: 1000000000,
            freeze_authority: "0xe5da95de00f8ef23fac3f16528bb23a264ad263fb253f95a95089f50350d97f3".to_string(),
            mint_authority: "0xe5da95de00f8ef23fac3f16528bb23a264ad263fb253f95a95089f50350d97f3".to_string(),
            url: "https://miucoin.io/favicon.ico".to_string(),
        };

        let project_name_result = publisher.publish_token(params);
        println!("project_name: {:?}", project_name_result);
        assert!(project_name_result.is_ok());

        let sui_client = SuiClientBuilder::default()
             .build_testnet()
             .await.unwrap();

        let address = SuiAddress::from_str("0x823e17a9a03e56f26700d8ebf23a3644bd65bfda26272d55c3e7148f77c887c1").unwrap();

        let mut builder = ProgrammableTransactionBuilder::new();
        let project_path = Path::new("/home/ccbond/suiz_data/published").join(project_name_result.unwrap());
        publish_contract(&mut builder, project_path.to_str().unwrap()).unwrap();
        let publish = TransactionKind::programmable(builder.finish());
        let transaction_bytes =
            TransactionData::new_with_gas_coins(publish, address, vec![], 100000000, 1000);
    
        let result = sui_client
            .read_api()
            .dry_run_transaction_block(transaction_bytes)
            .await;
        println!("result: {:?}", result);

    }
}
