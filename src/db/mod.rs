mod db;

pub use db::*;

pub mod submissions {
    pub mod dsl {
        pub use crate::schema::submissions::dsl::submissions as submissions;
        pub use crate::schema::submissions::*;
    }
}

pub mod clap_history {
    pub mod dsl {
        pub use crate::schema::clap_history::dsl::clap_history as clap_history;
        pub use crate::schema::clap_history::*;
    }
}
