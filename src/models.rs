use crate::db::get_conn;

use super::schema::{deployments, mints};
use anyhow::Result;
use diesel::{prelude::*, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InscriptionDeploy {
    pub p: String,
    pub op: String,
    pub tick: String,
    pub max: String,
    pub lim: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscriptionMint {
    pub p: String,
    pub op: String,
    pub tick: String,
    pub id: String,
    pub amt: String,
}

impl InscriptionDeploy {
    pub fn from_bytes(data: &Vec<u8>) -> Option<Self> {
        // The minimum input for the Inscription Protocol is "data:,{}"
        if data.len() < 8 || data[..6] != [100, 97, 116, 97, 58, 44] {
            return None;
        }

        match serde_json::from_slice::<Self>(&data[6..]) {
            Ok(data) => Some(data),
            _ => None,
        }
    }

    pub fn from_hex_str(value: &str) -> Option<Self> {
        let buffer = match hex::decode(value.trim_start_matches("0x")) {
            Ok(buf) => Some(buf),
            _ => None,
        }?;

        Self::from_bytes(&buffer)
    }
}

impl InscriptionMint {
    pub fn from_bytes(data: &Vec<u8>) -> Option<Self> {
        // The minimum input for the Inscription Protocol is "data:,{}"
        if data.len() < 8 || data[..6] != [100, 97, 116, 97, 58, 44] {
            return None;
        }

        match serde_json::from_slice::<Self>(&data[6..]) {
            Ok(data) => Some(data),
            _ => None,
        }
    }

    pub fn from_hex_str(value: &str) -> Option<Self> {
        let buffer = match hex::decode(value.trim_start_matches("0x")) {
            Ok(buf) => Some(buf),
            _ => None,
        }?;

        Self::from_bytes(&buffer)
    }
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = deployments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Deployment {
    #[serde(skip)]
    pub id: i32,
    pub p: String,
    pub op: String,
    pub tick: String,
    pub max: String,
    pub lim: String,
    pub input_data: String,
    pub minted: String,
    pub holders: i64,
    pub trx_hash: String,
    pub chain_id: i64,
    #[serde(rename = "owner")]
    pub from_address: String,
    #[serde(skip)]
    pub to_address: String,
    pub height: i64,
    pub timestamp: i64,
}

#[derive(Insertable)]
#[diesel(table_name = deployments)]
pub struct NewDeployment<'a> {
    pub p: &'a str,
    pub op: &'a str,
    pub tick: &'a str,
    pub max: &'a str,
    pub lim: &'a str,
    pub input_data: &'a str,
    pub minted: &'a str,
    pub holders: i64,
    pub trx_hash: &'a str,
    pub chain_id: i64,
    pub from_address: &'a str,
    pub to_address: &'a str,
    pub height: i64,
    pub timestamp: i64,
}

impl Deployment {
    pub fn all() -> Result<Vec<Self>> {
        let result = deployments::table
            .select(Deployment::as_select())
            .order(deployments::timestamp.desc())
            .load(&mut get_conn()?)?;

        Ok(result)
    }

    pub fn get(chain_id: i64, tick: &str) -> Result<Self> {
        let result = deployments::table
            .filter(
                deployments::chain_id
                    .eq(chain_id)
                    .and(deployments::tick.eq(tick)),
            )
            .select(Deployment::as_select())
            .first(&mut get_conn()?)?;

        Ok(result)
    }

    pub fn add_minted(&self, owner: &str, token_number: u128) -> Result<()> {
        let mut add_holder = 0;
        let is_first_mint = Mint::owner(owner)?
            .into_iter()
            .filter(|m| m.chain_id == self.chain_id && m.tick == self.tick)
            .collect::<Vec<Mint>>()
            .is_empty();

        if is_first_mint {
            add_holder = 1;
        }
        diesel::update(deployments::table)
            .filter(deployments::id.eq(self.id))
            .set((
                deployments::minted
                    .eq((str::parse::<u128>(&self.minted)? + token_number).to_string()),
                deployments::holders.eq(deployments::holders + add_holder),
            ))
            .execute(&mut get_conn()?)?;
        Ok(())
    }
}

impl<'a> NewDeployment<'a> {
    pub fn insert(&self) -> Result<()> {
        let conn = &mut get_conn()?;

        diesel::insert_into(deployments::table)
            .values(self)
            .execute(conn)?;

        Ok(())
    }
}

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = mints)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Mint {
    #[serde(skip)]
    pub id: i32,
    pub p: String,
    pub op: String,
    pub tick: String,
    #[serde(rename = "id")]
    pub tick_id: String,
    pub amt: String,
    pub input_data: String,
    pub trx_hash: String,
    pub chain_id: i64,
    #[serde(rename = "owner")]
    pub from_address: String,
    #[serde(skip)]
    pub to_address: String,
    pub height: i64,
    pub timestamp: i64,
}

#[derive(Insertable)]
#[diesel(table_name = mints)]
pub struct NewMint<'a> {
    pub p: &'a str,
    pub op: &'a str,
    pub tick: &'a str,
    pub tick_id: &'a str,
    pub amt: &'a str,
    pub input_data: &'a str,
    pub trx_hash: &'a str,
    pub chain_id: i64,
    pub from_address: &'a str,
    pub to_address: &'a str,
    pub height: i64,
    pub timestamp: i64,
}

impl Mint {
    pub fn find(chain_id: i64, wallet: &str, tick: &str) -> Result<Vec<Mint>> {
        let result = mints::table
            .filter(
                mints::chain_id
                    .eq(chain_id)
                    .and(mints::from_address.eq(wallet))
                    .and(mints::tick.eq(tick)),
            )
            .select(Mint::as_select())
            .load(&mut get_conn()?)?;

        Ok(result)
    }

    pub fn owner(wallet: &str) -> Result<Vec<Mint>> {
        let result = mints::table
            .filter(mints::from_address.eq(wallet))
            .select(Mint::as_select())
            .load(&mut get_conn()?)?;

        Ok(result)
    }
}

impl<'a> NewMint<'a> {
    pub fn insert(&self) -> Result<()> {
        let conn = &mut get_conn()?;

        diesel::insert_into(mints::table)
            .values(self)
            .execute(conn)?;

        Ok(())
    }
}
