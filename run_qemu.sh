#!/bin/bash

set -e

make iso
qemu-system-x86_64 -serial stdio -drive format=raw,file=build/falconos-x86_64.iso
