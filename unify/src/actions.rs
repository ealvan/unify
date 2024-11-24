pub mod operations{
    use std::fs::Metadata;
    pub struct FileInfo{
        metadata: Metadata,
        filename: String
    }
    pub enum Operation{
        DO_VERSIONS,
        DO_MOVE,
        DO_REMOVE
    }
    
    
}
