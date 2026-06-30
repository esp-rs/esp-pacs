#[doc = "Register `REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC>;
#[doc = "Field `CH1_CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub type CH1_CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn ch1_curtbufaptr(&self) -> CH1_CURTBUFAPTR_R {
        CH1_CURTBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER")
            .field("ch1_curtbufaptr", &self.ch1_curtbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register84_channel1currenthosttransmitbufferaddressregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register84_channel1currenthosttransmitbufferaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER84_CHANNEL1CURRENTHOSTTRANSMITBUFFERADDRESSREGISTER_SPEC {}
