// Copyright 2020 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use sys_util::Result;

use super::{KvmVcpu, KvmVm};
use crate::{VcpuAArch64, VmAArch64};

impl VmAArch64 for KvmVm {
    type Vcpu = KvmVcpu;

    fn create_vcpu(&self, id: usize) -> Result<Self::Vcpu> {
        self.create_kvm_vcpu(id)
    }
}

impl VcpuAArch64 for KvmVcpu {
    fn set_one_reg(&self, _reg_id: u64, _data: u64) -> Result<()> {
        Ok(())
    }
}
