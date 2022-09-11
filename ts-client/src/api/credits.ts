import { ApiPromise } from "@polkadot/api";

/**
 * Credit-specific information that is available on the blockchain.
 */
export interface Credit {
  /**
   * Numeric id of the credit. Can be used to refer to the credit throughout the system.
   */
  id: string,
  /**
   * Credit name.
   */
  name: string,
  /**
   * Credit kind.
   */
  creditKind: string,
  /**
   * A Blake2 hash of the credit's photo, if present.
   * The actual photo isn't stored on the chain, just the hash.
   */
  photo?: string,
}

export enum CardClass {
  Gold,
  Platinum,
  Diamond,
}

export interface CreditCard {
  creditId: string,
  cardId: string,
  class: CardClass,
  hash: string,
}

/**
 * Get all credits from the chain.
 *
 * @param api Handle to the Substrate-based API entrypoint
 * @returns A list of all credits
 */
export async function getAllCredits(api: ApiPromise): Promise<Credit[]> {
  return null as any
}

/**
 * Get all approved credits from the chain.
 * This doesn't include credits that have submitted applications but weren't approved yet.
 *
 * @param api Handle to the Substrate-based API entrypoint
 * @returns A list of all approved credits
 */
export async function getApprovedCredits(api: ApiPromise): Promise<Credit[]> {
  return null as any
}

/**
 * Get all applicant credits from the chain.
 * This only includes credits that have submitted applications, but weren't approved yet.
 *
 * @param api Handle to the Substrate-based API entrypoint
 * @returns A list of all approved credits
 */
export async function getApplicantCredits(api: ApiPromise): Promise<Credit[]> {
  return null as any
}

/**
 * Get all cards minted for a specific credit.
 *
 * @param api
 * @param creditId
 * @returns
 */
export async function getCreditCards(api: ApiPromise, creditId: string): Promise<CreditCard[]> {
  return null as any
}
