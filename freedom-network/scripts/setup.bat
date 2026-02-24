@echo off
echo Setting up Freedom Network...
cd node
cargo build --release
cd ../browser
cargo build --release
cd ..
echo Setup complete!
pause