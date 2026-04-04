set -ex
cargo fmt
scp src/config.rs ec2-user@leu-235.com:./j-moni/src/
ssh -t ec2-user@leu-235.com "tmux send-keys -t j-moni C-c C-m"
ssh -t ec2-user@leu-235.com "tmux send-keys -t j-moni cargo\ run C-m"
