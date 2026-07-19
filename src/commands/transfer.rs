use alloy::primitives::U256;
use alloy::sol;
use alloy::sol_types::SolCall;
use anyhow::Result;
use clap::Args;

use polymarket_client_sdk_v2::types::Address;

use crate::output::OutputFormat;

use super::proxy;

sol! {
    interface IERC20 {
        function transfer(address to, uint256 amount) external returns (bool);
    }
}

#[derive(Args)]
pub struct TransferArgs {
    #[arg(long)]
    pub to: Address,

    #[arg(long)]
    pub amount: f64,
}

pub async fn execute(
    args: TransferArgs,
    _output: OutputFormat,
    private_key: Option<&str>,
    signature_type: Option<&str>,
) -> Result<()> {

    let use_proxy = proxy::is_proxy_mode(signature_type)?;

    let amount = U256::from((args.amount * 1_000_000.0) as u64);

    let calldata = IERC20::transferCall {
        to: args.to,
        amount,
    }
    .abi_encode();

    let (tx, _) = proxy::send_call(
        private_key,
        use_proxy,
        super::COLLATERAL_ADDRESS,
        calldata,
    )
    .await?;

    println!("Transaction sent:");
    println!("{tx}");

    Ok(())
}
