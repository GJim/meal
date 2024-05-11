## Backend
```shell
# initialize project
cargo shuttle init

cargo shuttle run
```

## Database
```shell
# show database credential
cargo shuttle resource list --show-secrets

psql -U user-meal -h db.shuttle.rs -d db-meal
```