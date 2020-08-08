# install cargo-watch and systemfd
# cargo install systemfd cargo-watch

systemfd --no-pid -s http::5000 -- cargo watch -x run