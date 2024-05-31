#!/bin/sh

curl -LO https://github.com/neovim/neovim/releases/download/stable/nvim-linux64.tar.gz
tar xzvf nvim-linux64.tar.gz
mkdir -p $HOME/.local/share/nvim-linux64
mv nvim-linux64 $HOME/.local/share/
ln -s $HOME/.local/share/nvim-linux64/bin/nvim /usr/local/bin/nvim
rm nvim-linux64.tar.gz

# Config
mkdir -p $HOME/.config/nvim
git clone https://github.com/xRetry/nvim.git $HOME/.config/nvim
nvim --headless +'Lazy sync' +'sleep 20' +qall
nvim --headless +'TSInstall rust' +'MasonInstall rust-analyzer' +'sleep 20' +qall

apt-get install -y tmux
# Config
mkdir -p $HOME/.config/tmux
git clone https://github.com/xRetry/tmux.git $HOME/.config/tmux
echo "alias tmux='TERM=xterm-256color tmux'" >> $HOME/.bashrc
