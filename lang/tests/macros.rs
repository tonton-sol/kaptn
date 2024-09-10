use core::str::FromStr;
use kaptn_lang::solana_program::pubkey::Pubkey;

mod id {
    kaptn_lang::declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
}

#[test]
fn test_declare_id() {
    let good = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let bad = Pubkey::from_str("A7yUYJNEVYRLE4QWsnc9rE9JRsm7DfqEmLscQVwkffAk").unwrap();
    assert_eq!(good, id::ID);
    assert_eq!(good, id::id());
    assert!(id::check_id(&good));
    assert!(!id::check_id(&bad));
}
