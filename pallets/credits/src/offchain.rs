use crate::{Call, Config, Pallet};
use alloc::{string::String, vec::Vec};
use core::str;
use frame_system::offchain::{SendUnsignedTransaction, SignedPayload, Signer, SigningTypes};
use fresh_credit_primitives::{CreditId, InstanceId};
use sp_runtime::offchain::{http, Duration};

pub const ORACLE_URL: &str = "http://www.foo.com";

#[derive(
  Debug, Eq, Ord, parity_scale_codec::Decode, parity_scale_codec::Encode, PartialEq, PartialOrd,
)]
pub struct OffchainPayload<P> {
  pub(crate) class_id: CreditId,
  pub(crate) instance_id: InstanceId,
  pub(crate) public: P,
  pub(crate) views: u32,
  pub(crate) votes: u32,
}

impl<T> SignedPayload<T> for OffchainPayload<T::Public>
where
  T: SigningTypes,
{
  fn public(&self) -> T::Public {
    self.public.clone()
  }
}

impl<T> Pallet<T>
where
  T: Config,
{
  pub(crate) fn fetch_and_parse_credit_info()
  -> Result<(CreditId, InstanceId, u32, u32), http::Error> {
    let res = Self::request_response(ORACLE_URL)?;
    Self::parse_credit_info(&res).ok_or(http::Error::Unknown)
  }

  fn parse_credit_info(s: &str) -> Option<(CreditId, InstanceId, u32, u32)> {
    let mut iter = s.split(',');
    let class_id = iter.next()?.parse().ok()?;
    let instance_id = iter.next()?.parse().ok()?;
    let views = iter.next()?.parse().ok()?;
    let votes = iter.next()?.parse().ok()?;
    Some((class_id, instance_id, views, votes))
  }

  fn request_response(url: &str) -> Result<String, http::Error> {
    let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
    let request = http::Request::get(url);
    let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
    let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;

    if response.code != 200 {
      return Err(http::Error::Unknown);
    }

    let body = response.body().collect::<Vec<u8>>();
    let body_string = String::from_utf8(body).map_err(|_| http::Error::Unknown)?;

    Ok(body_string)
  }
}
