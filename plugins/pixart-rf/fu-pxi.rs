// Copyright 2023 Richard Hughes <richard@hughsie.com>
// SPDX-License-Identifier: LGPL-2.1-or-later

#[derive(ToString)]
enum FuPxiOtaSpecCheckResult {
    Ok = 1,
    FwOutOfBounds = 2,
    ProcessIllegal = 3,
    Reconnect = 4,
    FwImgVersionError = 5,
    DeviceLowBattery = 6,
}

#[derive(ToString)]
enum FuPxiWirelessModuleOtaRspCode {
    Ok,
    Finish,
    Fail,
    Error,
    WritePktLenError,
    WritePktTotalSizeError,
    ReadPktLenError,
    NotReady,
}
