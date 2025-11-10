pub fn run() {
    let scores = vec![
        super::test_ex::TestEx { score: 90 },
        super::test_ex::TestEx { score: 88 },
        super::test_ex::TestEx { score: 77 },
        super::test_ex::TestEx { score: 93 },
    ];

    for score in scores {
        println!("score: {:?}", score.score);
    }
}