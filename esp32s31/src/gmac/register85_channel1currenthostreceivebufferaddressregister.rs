#[doc = "Register `REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER` reader"]
pub type R = crate::R<REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC>;
#[doc = "Field `CH1_CURRBUFAPTR` reader - Host Receive Buffer Address Pointer"]
pub type CH1_CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn ch1_currbufaptr(&self) -> CH1_CURRBUFAPTR_R {
        CH1_CURRBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER")
            .field("ch1_currbufaptr", &self.ch1_currbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register85_channel1currenthostreceivebufferaddressregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register85_channel1currenthostreceivebufferaddressregister::R`](R) reader structure"]
impl crate::Readable for REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER to value 0"]
impl crate::Resettable for REGISTER85_CHANNEL1CURRENTHOSTRECEIVEBUFFERADDRESSREGISTER_SPEC {}
