extern crate diesel;

use crate::models::{InscriptionDeploy, InscriptionMint, NewDeployment, NewMint};
use crate::routes::routes;
use anyhow::Error;
use clap::Parser;
use cli::{Cli, Commands};
use ethers::prelude::*;
use http::Method;
use models::{Deployment, Mint};
use std::{net::SocketAddr, str::FromStr};
use tokio::time::{sleep, Duration};
use tower_http::cors::{Any, CorsLayer};

mod cli;
mod db;
mod handlers;
mod models;
mod routes;
mod schema;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sync {
            endpoint_url,
            start_block,
            end_block,
        }) => {
            sync(endpoint_url, start_block, end_block).await;
        }

        Some(Commands::Serve { port, host }) => {
            serve(host, port).await;
        }

        Some(Commands::All {
            endpoint_url,
            start_block,
            end_block,
            port,
            host,
        }) => {
            tokio::join!(
                sync(endpoint_url, start_block, end_block),
                serve(host, port),
            );
        }

        None => {}
    }
}

async fn sync(endpoint_url: &String, start_block: &u64, end_block: &u64) -> Result<(), Error> {
    let provider =
        Provider::<Http>::try_from(endpoint_url).expect("🚨 could not instantiate HTTP Provider");
    let chain_id = provider
        .get_chainid()
        .await
        .expect("🚨 failed to get Chain ID")
        .as_u32();

    let mut current_block = *start_block;
    println!("🎏 starting synchronization from {} height", current_block);

    while current_block <= *end_block {
        println!(
            "- currently synchronizing block at height {}",
            current_block
        );
        let block = provider.get_block_with_txs(current_block).await;
        let block = match block {
            Ok(blk) => match blk {
                Some(blk) => blk,
                _ => {
                    sleep(Duration::from_secs(30)).await;
                    continue;
                }
            },
            Err(err) => {
                println!(
                    "🚨 failed to get block at {}, error: {}",
                    current_block, err
                );
                sleep(Duration::from_secs(30)).await;
                continue;
            }
        };

        block.transactions.iter().for_each(|trx| {
            if let Some(data) = InscriptionDeploy::from_bytes(&trx.input.to_vec()) {
                let _ = NewDeployment {
                    p: &data.p,
                    op: &data.op,
                    tick: &data.tick,
                    max: &data.max,
                    lim: &data.lim,
                    input_data: &String::from_utf8(trx.input.to_vec()).unwrap(),
                    minted: "0",
                    holders: 0,
                    trx_hash: &bytes_to_hex_str(trx.hash.as_bytes()),
                    chain_id: chain_id.into(),
                    from_address: &bytes_to_hex_str(&trx.from.as_bytes()),
                    to_address: &bytes_to_hex_str(trx.to.unwrap_or_default().as_bytes()),
                    height: block.number.unwrap_or_default().as_u32().into(),
                    timestamp: block.timestamp.as_u32().into(),
                }
                .insert()
                .map_err(|e| println!("🚨 failed to insert NewDeployment: {e}"));
            } else if let Some(data) = InscriptionMint::from_bytes(&trx.input.to_vec()) {
                if is_allow_mint(chain_id.into(), &data) {
                    let _ = Deployment::get(chain_id.into(), &data.tick)
                        .unwrap()
                        .add_minted(
                            &bytes_to_hex_str(&trx.from.as_bytes()),
                            str::parse::<u128>(&data.amt).unwrap(),
                        )
                        .map_err(|e| println!("🚨 failed to add number of mint: {e}"));

                    let _ = NewMint {
                        p: &data.p,
                        op: &data.op,
                        tick: &data.tick,
                        tick_id: &data.id,
                        amt: &data.amt,
                        input_data: &String::from_utf8(trx.input.to_vec()).unwrap(),
                        trx_hash: &bytes_to_hex_str(trx.hash.as_bytes()),
                        chain_id: chain_id.into(),
                        from_address: &bytes_to_hex_str(&trx.from.as_bytes()),
                        to_address: &bytes_to_hex_str(trx.to.unwrap_or_default().as_bytes()),
                        height: block.number.unwrap_or_default().as_u32().into(),
                        timestamp: block.timestamp.as_u32().into(),
                    }
                    .insert()
                    .map_err(|e| println!("🚨 failed to insert NewMint: {e}"));
                };
            }
        });

        current_block += 1;
    }

    Ok(())
}

async fn serve(host: &String, port: &u16) -> Result<(), Error> {
    let app = routes().layer(
        CorsLayer::new()
            .allow_methods([Method::GET])
            .allow_origin(Any)
            .allow_headers(Any),
    );
    let addr = SocketAddr::from_str(&format!("{}:{}", host, port))?;
    println!("🚀 listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

pub fn bytes_to_hex_str(data: &[u8]) -> String {
    format!("0x{}", hex::encode(data))
}

fn is_allow_mint(chain_id: i64, mint: &InscriptionMint) -> bool {
    if let Ok(deployment) = Deployment::get(chain_id, &mint.tick) {
        if mint.op == "mint" && mint.amt == deployment.lim && mint.p == deployment.p {
            let max = match str::parse::<u128>(&deployment.max) {
                Ok(i) => i,
                Err(_) => {
                    return false;
                }
            };
            let minted = match str::parse::<u128>(&deployment.minted) {
                Ok(i) => i,
                Err(_) => {
                    return false;
                }
            };
            if max > minted {
                return true;
            }
        };
    };

    false
}
