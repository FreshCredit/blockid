use jsonrpc_core::serde_json::{Map, Value};
use fresh_credit_runtime::{
  AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
  SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

const BASE: u128 = 1_000_000_000_000;

pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

type AccountPublic = <Signature as Verify>::Signer;

pub(crate) fn development_config() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary is not available".to_string())?;

  Ok(ChainSpec::from_genesis(
    "Development",
    "dev",
    ChainType::Development,
    move || {
      let initial_authorities = vec![authority_keys_from_seed("Alice")];
      let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");
      GenesisConfig {
        system: SystemConfig { code: wasm_binary.to_vec() },
        balances: BalancesConfig {
          balances: vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
          ]
          .into_iter()
          .map(|k| (k, 1 << 64))
          .collect(),
        },
        aura: AuraConfig {
          authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
          authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
        },
        sudo: SudoConfig { key: Some(root_key) },
        transaction_payment: Default::default(),
      }
    },
    vec![
      "/ip4/198.51.100.19/tcp/30333/p2p/QmSk5HQbn6LhUwDiNMseVUjuRYhEtYj4aUZ6WfWoGURpdV"
        .parse()
        .unwrap(),
    ],
    None,
    None,
    Some(token_properties()),
    None,
  ))
}

pub(crate) fn johnny_quick() -> Result<ChainSpec, String> {
  let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

  Ok(ChainSpec::from_genesis(
    "Fresh Credit - Johnny Quick",
    "fresh_credit_johnny_quick",
    ChainType::Live,
    move || {
      let initial_authorities: Vec<(AuraId, GrandpaId)> = vec![(
        hex_literal::hex!["70abc20aca1f03b178c62f37af225b772a725041b0b4cf36fdce39e04a05207d"]
          .unchecked_into(),
        hex_literal::hex!["b01c60192a4fe833180e3db2bfa82a6acc4d08309db559738e42d0a19584748c"]
          .unchecked_into(),
      )];
      let root_key =
        hex_literal::hex!["70abc20aca1f03b178c62f37af225b772a725041b0b4cf36fdce39e04a05207d"]
          .into();
      GenesisConfig {
        system: SystemConfig { code: wasm_binary.to_vec() },
        balances: BalancesConfig {
          balances: vec![
            hex_literal::hex!["70abc20aca1f03b178c62f37af225b772a725041b0b4cf36fdce39e04a05207d"]
              .into(),
          ]
          .into_iter()
          .map(|k| (k, 1_000_000_000 * BASE))
          .collect(),
        },
        aura: AuraConfig {
          authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
          authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
        },
        sudo: SudoConfig { key: Some(root_key) },
        transaction_payment: Default::default(),
      }
    },
    vec![],
    telemetry_endpoints(),
    Some("fresh_credit_johnny_quick_protocol_id"),
    Some(token_properties()),
    None,
  ))
}

fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
  (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
  AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
  AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
  TPublic::Pair::from_string(&format!("//{}", seed), None)
    .expect("static values are valid; qed")
    .public()
}

fn telemetry_endpoints() -> Option<TelemetryEndpoints> {
  TelemetryEndpoints::new(vec![("wss://telemetry.polkadot.io/submit/".into(), 0)]).ok()
}

fn token_properties() -> Map<String, Value> {
  let mut properties = Map::new();
  properties.insert("tokenSymbol".into(), "ATHL".into());
  properties.insert("tokenDecimals".into(), "12".into());
  properties
}
