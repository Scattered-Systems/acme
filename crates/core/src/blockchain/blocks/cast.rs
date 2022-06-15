pub trait Block {
    type BlockId;
    type Data;
    type Hash;
    type Nonce;
    type Timestamp;
    type Transaction;
}