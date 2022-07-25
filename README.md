# currency-checker-rs

Learn deasel + rocket

For start:

* up docker-compose.yml file (up postgres 14)
* cargo run

## Sync request

Method: Post

Api: `http://127.0.0.1:8000/sync`

Sync currency with cb by current day. Return currency count

## Sync by date request

Method: Post

Api: `http://127.0.0.1:8000/sync_by_date/yyyy-MM-dd`

Sync currency with cb by requested day. Return currency count

## Todat request

Methiod: Get

Api: `http://127.0.0.1:8000/today`

Return currency list by current day

## By date request

Methiod: Get

Api: `http://127.0.0.1:8000/by_date/yyyy-MM-dd`

Return currency list by requested day
