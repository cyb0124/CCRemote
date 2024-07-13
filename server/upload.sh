set -ex
cargo fmt
scp src/config.rs ec2-user@leu-235.com:./CC5/src/
ssh -t ec2-user@leu-235.com "tmux send-keys -t CC5 C-c C-m"
ssh -t ec2-user@leu-235.com "tmux send-keys -t CC5 cargo\ run C-m"
