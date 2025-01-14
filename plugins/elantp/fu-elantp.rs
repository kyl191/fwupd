// Copyright 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1-or-later

#[derive(ValidateStream, Default)]
struct FuStructElantpFirmwareHdr {
    magic: [u8; 6] == 0xAA55CC33FFFF,
}

#[derive(ValidateStream, Default)]
struct FuStructElantpHapticFirmwareHdr {
    magic: [u8; 4] == 0xFF40A25B,
}
