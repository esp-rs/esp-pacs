#[doc = "Register `REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC>;
#[doc = "Field `CURRBUFAPTR` reader - Host Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register21_currenthostreceivebufferaddressregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register21_currenthostreceivebufferaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER21_CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {}
