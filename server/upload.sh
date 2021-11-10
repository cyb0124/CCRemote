set -ex
cargo fmt
scp -i ../../lckey/repo src/config.rs ec2-user@leu-235.com:./CC1/src/
ssh -i ../../lckey/repo -t ec2-user@leu-235.com "screen -S CC1 -p 0 -X stuff \"^C^Mcargo run^M\""
