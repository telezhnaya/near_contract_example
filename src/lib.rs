/*
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();
// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Duck {
}

impl Default for Duck {
    fn default() -> Self {
        Self {}
    }
}

#[near_bindgen]
impl Duck {
    pub fn fake_move_nft() {
        env::log(r#"
            EVENT_JSON:
              {
                "standard": "nep171",
                "version": "1.0.0",
                "event": "nft_mint",
                "data": [
                  {
                    "owner_id": "frol.near",
                    "token_ids": [
                      "awesome_token",
                      "nice_token"
                    ],
                    "memo": "some tokens for frol"
                  },
                  {
                    "owner_id": "olga.near",
                    "token_ids": [
                      "cool_token",
                      "wow_token"
                    ]
                  }
                ]
              }
        "#.as_bytes());

        env::log(r#"
            EVENT_JSON:
              {
                "standard": "nep171",
                "version": "1.0.0",
                "event": "nft_transfer",
                "data": [
                  {
                    "authorized_id": "bohdan.near",
                    "old_owner_id": "frol.near",
                    "new_owner_id": "olga.near",
                    "token_ids": [
                      "nice_token"
                    ],
                    "memo": "because we can"
                  },
                  {
                    "old_owner_id": "olga.near",
                    "new_owner_id": "bohdan.near",
                    "token_ids": [
                      "cool_token",
                      "wow_token"
                    ]
                  }
                ]
              }
        "#.as_bytes());

        env::log(r#"
            EVENT_JSON:
              {
                "standard": "nep171",
                "version": "1.0.0",
                "event": "nft_burn",
                "data": [
                  {
                    "authorized_id": "olga.near",
                    "owner_id": "bohdan.near",
                    "token_ids": [
                      "cool_token"
                    ],
                    "memo": "good bye"
                  },
                  {
                    "owner_id": "frol.near",
                    "token_ids": [
                      "awesome_token"
                    ]
                  }
                ]
              }
        "#.as_bytes());
    }

    pub fn fake_move_ft() {
        env::log(r#"
            EVENT_JSON:
              {
                "standard": "nep141",
                "version": "1.0.0",
                "event": "ft_mint",
                "data": [
                  {
                    "owner_id": "frol.near",
                    "amount": "100500",
                    "memo": "some tokens for frol"
                  },
                  {
                    "owner_id": "olga.near",
                    "amount": "42424242"
                  }
                ]
              }
        "#.as_bytes());

        env::log(r#"
            EVENT_JSON:
              {
                "standard": "nep141",
                "version": "1.0.0",
                "event": "ft_transfer",
                "data": [
                  {
                    "old_owner_id": "frol.near",
                    "new_owner_id": "bohdan.near",
                    "amount": "666",
                    "memo": "some tokens for bohdan"
                  },
                  {
                    "old_owner_id": "olga.near",
                    "new_owner_id": "frol.near",
                    "amount": "43"
                  }
                ]
              }
        "#.as_bytes());

        env::log(r#"
            EVENT_JSON:
              {
                "standard": "nep141",
                "version": "1.0.0",
                "event": "ft_burn",
                "data": [
                  {
                    "owner_id": "frol.near",
                    "amount": "500",
                    "memo": "burn some tokens from frol"
                  },
                  {
                    "owner_id": "olga.near",
                    "amount": "42"
                  }
                ]
              }
        "#.as_bytes());
    }
}
