# rust-tinyusb-example

A simple rust embedded application for the ATSAMG55J19, demonstrating how to
use the [tinyUSB](https://github.com/hathach/tinyusb) C library from Rust.

It creates a CDC device, and echos received characters back to the host.

## Background

I needed USB on an ATSAMG55J19, but couldn't find a rust USB driver that 
supports it -- it's a different peripheral from the samd1/samd5 families.
However, tinyUSB -- a great little USB library used e.g. by [modm](https://github.com/modm-io/modm) --
does support it, and I figured it would be a lot easier to call into it
from Rust than to write my own driver. It was pretty easy to do, but
I put in some effort to figure it out so I'm saving this stripped down
example here for posterity, and in case anyone else might find it useful.

## atsamg5x PAC

When I created this, there wasn't an official rust embedded PAC for the 
samg55j19, so I also had to create that, [here](https://github.com/mcbridejc/atsamg5-hal). 

This example pulls in the HAL to get access to the PAC, but does not actually
use any part of the HAL. As of this writing, it's not published to crates.io,
but maybe it will be at some point in the future.

## Setup/Building

You may need to run `git submodule update --init --recursive` to pull in tinyusb
and all of its submodules.
