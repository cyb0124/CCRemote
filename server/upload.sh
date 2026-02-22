set -ex
cargo fmt
scp src/config.rs ec2-user@leu-235.com:./j-cabin/src/
ssh -t ec2-user@leu-235.com "tmux send-keys -t j-cabin C-c C-m"
ssh -t ec2-user@leu-235.com "tmux send-keys -t j-cabin cargo\ run C-m"
