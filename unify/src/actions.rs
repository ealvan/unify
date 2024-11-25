pub mod operations{
    use std::fs::Metadata;
    pub struct FileInfo{
        pub metadata: Metadata,
        pub filename: String
    }
    pub enum Operation{
        DoVersions,
        DoMove,
        DoRemove,
        FindUniques
    }
    pub struct Paths{
        pub intersection: String,
        pub r1_r2: String,
        pub r2_r1: String
    }
    //in result -> OK or Err
    //in Option -> Some or None
    
}
