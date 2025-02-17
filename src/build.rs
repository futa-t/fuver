struct Build {
    identifier: BuildIdentifiers,
}

enum BuildIdentifiers {
    BuildNumber { number: usize },
    CommitHash { hash: String },
    DateTime {},
}
