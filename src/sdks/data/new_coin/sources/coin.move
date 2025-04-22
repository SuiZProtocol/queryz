// SPDX-License-Identifier: Apache-2.0

module symbol::symbol;

use sui::coin;

/// Witness of the coin
public struct SYMBOL has drop {}

/// Register the trusted currency to acquire its `TreasuryCap`. Because
/// this is a module initializer, it ensures the currency only gets
/// registered once.
fun init(witness: SYMBOL, ctx: &mut TxContext) {
    // Get a treasury cap for the coin and give it to the transaction
    // sender
    let (mut treasury_cap, metadata) = coin::create_currency<SYMBOL>(
        witness,
        DECIMALS,
        b"SYMBOL",
        b"NAME",
        b"DESCRIPTION",
        option::some(sui::url::new_unsafe_from_bytes(b"ICON")),
        ctx,
    );
    transfer::public_freeze_object(metadata);
    sui::coin::mint_and_transfer(&mut treasury_cap, SUPPLY, @COIN_RECIPIENT, ctx);
    transfer::public_transfer(treasury_cap, @CAP_RECIPIENT)
}
