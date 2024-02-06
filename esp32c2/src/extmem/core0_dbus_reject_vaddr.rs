#[doc = "Register `CORE0_DBUS_REJECT_VADDR` reader"]
pub type R = crate::R<CORE0_DBUS_REJECT_VADDR_SPEC>;
#[doc = "Field `CORE0_DBUS_VADDR` reader - The bits are used to indicate the virtual address of CPU access dbus when authentication fail."]
pub type CORE0_DBUS_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access dbus when authentication fail."]
    #[inline(always)]
    pub fn core0_dbus_vaddr(&self) -> CORE0_DBUS_VADDR_R {
        CORE0_DBUS_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_DBUS_REJECT_VADDR")
            .field(
                "core0_dbus_vaddr",
                &format_args!("{}", self.core0_dbus_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE0_DBUS_REJECT_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core0_dbus_reject_vaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE0_DBUS_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for CORE0_DBUS_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core0_dbus_reject_vaddr::R`](R) reader structure"]
impl crate::Readable for CORE0_DBUS_REJECT_VADDR_SPEC {}
#[doc = "`reset()` method sets CORE0_DBUS_REJECT_VADDR to value 0xffff_ffff"]
impl crate::Resettable for CORE0_DBUS_REJECT_VADDR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
