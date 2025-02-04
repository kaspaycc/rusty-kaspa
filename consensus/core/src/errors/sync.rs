use hashes::Hash;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SyncManagerError {
    #[error("low hash {0} is not in selected parent chain")]
    BlockNotInSelectedParentChain(Hash),

    #[error("low hash {0} is higher than high hash {1}")]
    LowHashHigherThanHighHash(Hash, Hash),
}

pub type SyncManagerResult<T> = std::result::Result<T, SyncManagerError>;
