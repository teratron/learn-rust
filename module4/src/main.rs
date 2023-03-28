mod cat {
    pub use activities::play::*;
    pub use kitten::meow;

    mod kitten;

    mod activities {
        pub mod play;
    }
}

mod bone;

fn main() {
    cat::meow();

    // let ball = cat::activities::play::CatToy::new("ball");
    // or
    let ball = cat::CatToy::new("ball");
    ball.fetch();

    bone::fetch();
}
