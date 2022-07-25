table! {
    currency (id) {
        id -> Uuid,
        iso_num_code -> Int4,
        iso_char_code -> Varchar,
        nominal -> Int4,
        value -> Float8,
        name -> Varchar,
        date -> Timestamptz,
    }
}
