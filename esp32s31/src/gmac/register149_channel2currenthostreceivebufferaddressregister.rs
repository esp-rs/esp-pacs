#[doc = "Register `REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC>;
#[doc = "Field `CH2_CURRBUFAPTR` reader - Host Receive Buffer Address Pointer"]
pub type CH2_CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn ch2_currbufaptr(&self) -> CH2_CURRBUFAPTR_R {
        CH2_CURRBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER")
            .field("ch2_currbufaptr", &self.ch2_currbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register149_channel2currenthostreceivebufferaddressregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register149_channel2currenthostreceivebufferaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER149_CHANNEL2CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {}
