CREATE TABLE currency (
    id uuid PRIMARY KEY,
    iso_num_code int NOT NULL,
    iso_char_code varchar(32) NOT NULL,
    nominal int NOT NULL,
    value float NOT NULL,
    name varchar(128) NOT NULL,
    date timestamptz NOT NULL
); 