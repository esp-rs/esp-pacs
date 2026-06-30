#[doc = "Register `REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC>;
#[doc = "Field `CH2_CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub type CH2_CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn ch2_curtbufaptr(&self) -> CH2_CURTBUFAPTR_R {
        CH2_CURTBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER")
            .field("ch2_curtbufaptr", &self.ch2_curtbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register148_channel2currenthosttransmitbufferaddressregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register148_channel2currenthosttransmitbufferaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER148_CHANNEL2CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {}
