#[doc = "Register `CORE1_DBUS_REJECT_VADDR` reader"]
pub type R = crate::R<CORE1_DBUS_REJECT_VADDR_SPEC>;
#[doc = "Field `CORE1_DBUS_VADDR` reader - The bits are used to indicate the virtual address of CPU access dbus when authentication fail."]
pub type CORE1_DBUS_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access dbus when authentication fail."]
    #[inline(always)]
    pub fn core1_dbus_vaddr(&self) -> CORE1_DBUS_VADDR_R {
        CORE1_DBUS_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE1_DBUS_REJECT_VADDR")
            .field("core1_dbus_vaddr", &self.core1_dbus_vaddr())
            .finish()
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_dbus_reject_vaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE1_DBUS_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for CORE1_DBUS_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_dbus_reject_vaddr::R`](R) reader structure"]
impl crate::Readable for CORE1_DBUS_REJECT_VADDR_SPEC {}
#[doc = "`reset()` method sets CORE1_DBUS_REJECT_VADDR to value 0xffff_ffff"]
impl crate::Resettable for CORE1_DBUS_REJECT_VADDR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
