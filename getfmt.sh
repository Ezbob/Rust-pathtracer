#!/bin/env bash

echo 'Installing rust code formatter "rustfmt"..'

rustup component add rustfmt-preview

if [ $? == 0 ]; then
    echo 'done. Run the formatter with "cargo fmt"'
fi

