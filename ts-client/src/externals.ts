/**
 * This module is to guide TypeDoc towards documenting types
 * from dependencies for convenience, particularly Polkadot.js
 *
 * @module
 */

export * as api              from "@polkadot/api"
export * as apiTypes         from "@polkadot/api/types"
export * as types            from "@polkadot/types"
export * as typesTypes       from "@polkadot/types/types"
export * as typesInterfaces  from "@polkadot/types/interfaces"
export * as typesExtrinsic   from "@polkadot/types/extrinsic"
export * as typesCodec       from "@polkadot/types-codec"

export { default as BN }     from "bn.js"
