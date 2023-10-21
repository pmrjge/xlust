#!/bin/sh
[ ! -d "ignore_xla" ] && mkdir -p ignore_xla
wget https://github.com/elixir-nx/xla/releases/download/v0.5.1/xla_extension-x86_64-linux-gnu-cpu.tar.gz -O ignore_xla/xla_extension_cpu.tar.gz
cd ignore_xla || return
tar -xzvf xla_extension_cpu.tar.gz
rm xla_extension_cpu.tar.gz
return 0