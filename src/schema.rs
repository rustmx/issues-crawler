table! {
    projects (id) {
        id -> Int8,
        name -> Varchar,
        url -> Varchar,
        watchers -> Nullable<Int8>,
        forks -> Nullable<Int8>,
        stars -> Nullable<Int8>,
    }
}
