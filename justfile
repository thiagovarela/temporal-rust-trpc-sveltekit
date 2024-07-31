
gen_types:
    typeshare ./types -c ./types/typeshare.toml --lang=typescript --output-file=./app/src/lib/types.ts

temporal:
    temporal server start-dev --db-filename temporal.db

worker:
    cargo watch -C worker -x run -p worker