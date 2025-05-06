compile using cargo b --release

then navigate cd ./target/release

then use the binary as intended :) (you can omit the --release to have the debug version of the binary, but then you need to go to ./target/debug instead. Note that it will not be optimized)

hint : ./rust-todo --help
