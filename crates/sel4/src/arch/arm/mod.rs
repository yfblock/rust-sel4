//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: MIT
//

use crate::{const_helpers::u32_into_usize, sys};

mod arch;
mod invocations;
mod object;
mod vm_attributes;
mod vspace;

pub(crate) mod fault;

pub(crate) mod top_level {
    pub use super::{
        arch::top_level::*,
        object::{ObjectBlueprintArch, ObjectBlueprintArm, ObjectTypeArch, ObjectTypeArm},
        vm_attributes::VmAttributes,
        vspace::{FrameObjectType, TranslationTableObjectType},
        NUM_FAST_MESSAGE_REGISTERS,
    };
}

pub(crate) use vspace::vspace_levels;

/// The number of message registers which are passed in architectural registers.
pub const NUM_FAST_MESSAGE_REGISTERS: usize = u32_into_usize(sys::seL4_FastMessageRegisters);

pub(crate) mod cap_type_arch {
    use sel4_config::sel4_cfg_if;

    use crate::{declare_cap_type, declare_cap_type_for_object_of_fixed_size, sel4_cfg};

    #[sel4_cfg(ARM_HYPERVISOR_SUPPORT)]
    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_VCPU`.
        VCpu { ObjectTypeArch, ObjectBlueprintArch }
    }

    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_Page` with `size_bits = 12`.
        SmallPage { ObjectTypeArch, ObjectBlueprintArch }
    }

    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_Page` with `size_bits = 21`.
        LargePage { ObjectTypeArch, ObjectBlueprintArch }
    }

    #[sel4_cfg(ARCH_AARCH64)]
    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_Page` with `size_bits = 30`.
        HugePage { ObjectTypeSeL4Arch, ObjectBlueprintSeL4Arch }
    }

    #[sel4_cfg(ARCH_AARCH32)]
    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_Page` with `size_bits = 16`.
        Section { ObjectTypeSeL4Arch, ObjectBlueprintSeL4Arch }
    }

    #[sel4_cfg(ARCH_AARCH64)]
    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_VSpace`.
        VSpace { ObjectTypeSeL4Arch, ObjectBlueprintSeL4Arch }
    }

    #[sel4_cfg(ARCH_AARCH32)]
    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_PD`.
        PD { ObjectTypeSeL4Arch, ObjectBlueprintSeL4Arch }
    }

    declare_cap_type_for_object_of_fixed_size! {
        /// Corresponds to `seL4_ARM_PageTable`.
        PT { ObjectTypeArch, ObjectBlueprintArch }
    }

    sel4_cfg_if! {
        if #[sel4_cfg(ALLOW_SMC_CALLS)] {
            declare_cap_type! {
                /// Corresponds to `sel4_ARM_SMC`
                Smc
            }
        }
    }

    /// Alias for [`cap_type::SmallPage`](SmallPage).
    pub type Granule = SmallPage;

    #[sel4_cfg(ARCH_AARCH32)]
    /// Alias for [`cap_type::PD`](PD).
    pub type VSpace = PD;
}

pub(crate) mod cap_arch {
    use crate::{declare_cap_alias, sel4_cfg};

    #[sel4_cfg(ARM_HYPERVISOR_SUPPORT)]
    declare_cap_alias!(VCpu);

    declare_cap_alias!(SmallPage);
    declare_cap_alias!(LargePage);

    #[sel4_cfg(ARCH_AARCH64)]
    declare_cap_alias!(HugePage);

    #[sel4_cfg(ARCH_AARCH32)]
    declare_cap_alias!(PD);

    declare_cap_alias!(PT);

    #[sel4_cfg(ALLOW_SMC_CALLS)]
    declare_cap_alias!(Smc);
}
