up:
  docker compose up -d
  sleep 2

migrate: up
  diesel migration run

reset:
  diesel migration redo

run *ARGS:
  cargo run -- {{ARGS}}

backup:
  docker compose exec db pg_dumpall -U postgres | lz4 | backup.pg.lz4

