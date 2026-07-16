#[doc = "Register `REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC>;
#[doc = "Field `CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register20_currenthosttransmitbufferaddressregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register20_currenthosttransmitbufferaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER20_CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {}
