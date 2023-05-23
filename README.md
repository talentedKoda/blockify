# BLOCKIFY

[![Rayon crate](https://img.shields.io/crates/v/blockify.svg)](https://crates.io/crates/blockify)
[![Rayon documentation](https://docs.rs/blockify/badge.svg)](https://docs.rs/blockify)

A Rust blockchain library that provides the building blocks for creating a full-fledged blockchain application or platform, allowing you to focus on the higher-level features of your application without worrying about the low-level details of `block validation`, `data serialization`, `block building and mining`, and `cryptographic operations`.

# FEATURES

- **crypto**
- **record**
- **sqlite**
- **unit**
- **full**

# EXAMPLES

- **Record Trait**

```
fn main() {
    use blockify::{data::MetaData, record::Record};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Record)]
    struct Vote {
        session: i32,
        choice: i32,
    }

    // Generate a new keypair
    let keypair = blockify::generate_ed25519_key_pair();

    // Clone the public key
    let pub_key = keypair.clone().into_public_key();

    // Create a new `Vote` instance
    let my_record = Vote {
        session: 0,
        choice: 2,
    };

    // calculate the hash of my_record
    let my_record_hash = blockify::hash(&my_record);

    // sign my_record with the AuthKeyPair instance and obtain a digital signature
    let signature = my_record.sign(&keypair).unwrap();

    // verify the authencity of the digital signature
    assert!(my_record.verify(&signature, &pub_key).is_ok());

    // record the my_vote (convert it into a SignedRecord instance)
    let signed_record = my_record.record(keypair, MetaData::empty()).unwrap();

    // Compare the signature of `my_record` with that inside the `SignedRecord` instance
    assert_eq!(&signature, signed_record.signature());

    // Compare the public key used to sign my_record with that inside the `SignedRecord` instance.
    assert_eq!(&pub_key, signed_record.signer());

    // Compare the hash of my_record with that inside the `SignedRecord` instance.
    assert_eq!(&my_record_hash, signed_record.hash());

    // Verify the signature within the `SignedRecord` instance.
    assert!(signed_record.verify().is_ok());
}


```

- **Block and Chain using the sqlite feature**

```

use blockify::{
    block::{Block, UnchainedInstance},
    chain::Chain,
    data::Metadata,
    record::{Record, SignedRecord},
    SqliteChain,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Record, Clone, Serialize, Deserialize, PartialEq)]
struct Vote {
    data: String,
}

impl Vote {
    fn new(data: &str) -> Self {
        Vote { data: data.into() }
    }
}

fn main() {
    // Set the URL where the chain will be stored
    let chain_url = "target2/main/sqlite/";

    // Create the directory for the chain if it doesn't exist
    std::fs::create_dir_all(chain_url).expect("could not create chain_url");

    // Prepare data and key pair for the first set of records
    let datas1 = vec!["abcd", "efgh", "ijkl"];
    let keypair = blockify::generate_ed25519_key_pair();

    // Generate signed records for the first set of data
    let records1 = datas1
        .into_iter()
        .map(|w| Vote::new(w).record(keypair.clone(), Metadata::empty()))
        .filter(|r| r.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<SignedRecord<Vote>>>();

    // Prepare data and key pair for the second set of records
    let datas2 = vec!["mnop", "qrst", "uvwx"];

    // Generate signed records for the second set of data
    let records2 = datas2
        .into_iter()
        .map(|w| Vote::new(w).record(keypair.clone(), Metadata::empty()))
        .filter(|r| r.is_ok())
        .map(|v| v.unwrap())
        .collect::<Vec<SignedRecord<Vote>>>();

    // Create an unchained instance for the first set of records
    let mut builder1 = UnchainedInstance::new(Metadata::empty(), 0);

    // Push the first set of records into the unchained instance
    for record in records1 {
        builder1.push(record);
    }

    // Create an unchained instance for the second set of records
    let mut builder2 = UnchainedInstance::new(Metadata::empty(), 1);

    // Push the second set of records into the unchained instance
    for record in records2 {
        builder2.push(record);
    }

    // Create a new SQLite chain with the specified URL
    let mut chain = SqliteChain::new(chain_url).expect("sqlite connection cannot be established");

    // Append the first unchained instance to the chain
    let instance1 = chain.append(&builder1).expect("builder1 append erred");

    // Append the second unchained instance to the chain
    let instance2 = chain.append(&builder2).expect("builder2 append erred");

    // Retrieve block1 from the chain using the position of instance1
    let block1 = chain
        .block_at(instance1.position())
        .expect("couldn't retrieve block1");

    // Retrieve block2 from the chain using the position of instance2
    let block2 = chain
        .block_at(instance2.position())
        .expect("couldn't retrieve block2");

    // Validate block1 against instance1
    assert!(block1.validate(&instance1).is_ok());

    // Validate block2 against instance2
    assert!(block2.validate(&instance2).is_ok());

    // Retrieve records from block1
    let records_from_block1 = block1
        .records()
        .expect("couldn't retrieve records from block1");

    // Check if the retrieved records from block1 match the original records from builder1
    assert_eq!(builder1.records().as_slice(), &*records_from_block1);

    // Retrieve records from block2
    let records_from_block2 = block2
        .records()
        .expect("couldn't retrieve records from block2");

    // Check if the retrieved records from block2 match the original records from builder2
    assert_eq!(builder2.records().as_slice(), &*records_from_block2);
}


```

# DOWNLOADING

```
cargo add blockify
```

# CONTRIBUTING

All forms of contributions are gladly welcome.

# DEPENDENCIES

# LICENSE

**MIT**
