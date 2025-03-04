//
// Copyright 2023, Colias Group, LLC
//
// SPDX-License-Identifier: MIT
//

use sel4_config::sel4_cfg;

use crate::{
    cap::*, cap_type, AbsoluteCPtr, Cap, CapRights, CapTypeForFrameObject, Error,
    InvocationContext, Result, TranslationTableObjectType, VmAttributes, Word,
};

#[sel4_cfg(ARM_HYPERVISOR_SUPPORT)]
use crate::VCpuReg;

#[sel4_cfg(ARM_HYPERVISOR_SUPPORT)]
impl<C: InvocationContext> VCpu<C> {
    /// Corresponds to `seL4_ARM_VCPU_SetTCB`.
    pub fn vcpu_set_tcb(self, tcb: Tcb) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_ARM_VCPU_SetTCB(cptr.bits(), tcb.bits())
        }))
    }

    /// Corresponds to `seL4_ARM_VCPU_ReadRegs`.
    pub fn vcpu_read_regs(self, field: VCpuReg) -> Result<Word> {
        let res = self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_ARM_VCPU_ReadRegs(cptr.bits(), field.into_sys())
        });
        Error::or(res.error, res.value)
    }

    /// Corresponds to `seL4_ARM_VCPU_WriteRegs`.
    pub fn vcpu_write_regs(self, field: VCpuReg, value: Word) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_ARM_VCPU_WriteRegs(cptr.bits(), field.into_sys(), value)
        }))
    }

    /// Corresponds to `seL4_ARM_VCPU_AckVPPI`.
    pub fn vcpu_ack_vppi(self, irq: Word) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_ARM_VCPU_AckVPPI(cptr.bits(), irq)
        }))
    }

    /// Corresponds to `seL4_ARM_VCPU_InjectIRQ`.
    pub fn vcpu_inject_irq(self, virq: u16, priority: u8, group: u8, index: u8) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_ARM_VCPU_InjectIRQ(
                cptr.bits(),
                virq,
                priority,
                group,
                index,
            )
        }))
    }
}

#[sel4_cfg(ALLOW_SMC_CALLS)]
impl<C: InvocationContext> Smc<C> {
    pub fn smc_call(
        self,
        args: &sel4_sys::seL4_ARM_SMCContext,
    ) -> Result<sel4_sys::seL4_ARM_SMCContext> {
        let mut ctx = sel4_sys::seL4_ARM_SMCContext::default();
        let ret = self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_ARM_SMC_Call(cptr.bits() as _, args, &mut ctx)
        });
        match Error::from_sys(ret) {
            None => Ok(ctx),
            Some(err) => Err(err),
        }
    }
}

impl<T: CapTypeForFrameObject, C: InvocationContext> Cap<T, C> {
    /// Corresponds to `seL4_ARM_Page_Map`.
    pub fn frame_map(
        self,
        vspace: VSpace,
        vaddr: usize,
        rights: CapRights,
        attrs: VmAttributes,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_ARM_Page_Map(
                cptr.bits(),
                vspace.bits(),
                vaddr.try_into().unwrap(),
                rights.into_inner(),
                attrs.into_inner(),
            )
        }))
    }

    /// Corresponds to `seL4_ARM_Page_Unmap`.
    pub fn frame_unmap(self) -> Result<()> {
        Error::wrap(
            self.invoke(|cptr, ipc_buffer| ipc_buffer.inner_mut().seL4_ARM_Page_Unmap(cptr.bits())),
        )
    }

    /// Corresponds to `seL4_ARM_Page_GetAddress`.
    pub fn frame_get_address(self) -> Result<usize> {
        let ret = self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_ARM_Page_GetAddress(cptr.bits())
        });
        match Error::from_sys(ret.error) {
            None => Ok(ret.paddr.try_into().unwrap()),
            Some(err) => Err(err),
        }
    }
}

impl<C: InvocationContext> PT<C> {
    pub fn pt_map(self, vspace: VSpace, vaddr: usize, attr: VmAttributes) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_ARM_PageTable_Map(
                cptr.bits(),
                vspace.bits(),
                vaddr.try_into().unwrap(),
                attr.into_inner(),
            )
        }))
    }
}

impl<C: InvocationContext> UnspecifiedIntermediateTranslationTable<C> {
    pub fn generic_intermediate_translation_table_map(
        self,
        ty: TranslationTableObjectType,
        vspace: VSpace,
        vaddr: usize,
        attr: VmAttributes,
    ) -> Result<()> {
        match ty {
            TranslationTableObjectType::PT => {
                self.cast::<cap_type::PT>().pt_map(vspace, vaddr, attr)
            }
            _ => panic!(),
        }
    }
}

// TODO structured trigger type
impl<C: InvocationContext> IrqControl<C> {
    /// Corresponds to `seL4_IRQControl_GetTriggerCore`.
    #[sel4_cfg(not(MAX_NUM_NODES = "1"))]
    pub fn irq_control_get_trigger_core(
        self,
        irq: Word,
        edge_triggered: bool,
        target: Word,
        dst: &AbsoluteCPtr,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_IRQControl_GetTriggerCore(
                cptr.bits(),
                irq,
                edge_triggered.into(),
                dst.root().bits(),
                dst.path().bits(),
                dst.path().depth_for_kernel(),
                target,
            )
        }))
    }

    /// Corresponds to `seL4_IRQControl_GetTrigger`.
    pub fn irq_control_get_trigger(
        self,
        irq: Word,
        edge_triggered: bool,
        dst: &AbsoluteCPtr,
    ) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_IRQControl_GetTrigger(
                cptr.bits(),
                irq,
                edge_triggered.into(),
                dst.root().bits(),
                dst.path().bits(),
                dst.path().depth_for_kernel(),
            )
        }))
    }
}

impl<C: InvocationContext> AsidControl<C> {
    /// Corresponds to `seL4_ARM_ASIDControl_MakePool`.
    pub fn asid_control_make_pool(self, untyped: Untyped, dst: &AbsoluteCPtr) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer.inner_mut().seL4_ARM_ASIDControl_MakePool(
                cptr.bits(),
                untyped.bits(),
                dst.root().bits(),
                dst.path().bits(),
                dst.path().depth_for_kernel(),
            )
        }))
    }
}

impl<C: InvocationContext> AsidPool<C> {
    /// Corresponds to `seL4_ARM_ASIDPool_Assign`.
    pub fn asid_pool_assign(self, vspace: VSpace) -> Result<()> {
        Error::wrap(self.invoke(|cptr, ipc_buffer| {
            ipc_buffer
                .inner_mut()
                .seL4_ARM_ASIDPool_Assign(cptr.bits(), vspace.bits())
        }))
    }
}
