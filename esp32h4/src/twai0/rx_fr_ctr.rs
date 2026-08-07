#[doc = "Register `RX_FR_CTR` reader"]
pub type R = crate::R<RX_FR_CTR_SPEC>;
#[doc = "Field `VAL` reader - Number of received frames by CTU CAN FD."]
pub type VAL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of received frames by CTU CAN FD."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_FR_CTR")
            .field("val", &self.val())
            .finish()
    }
}
#[doc = "TWAI FD received frame counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fr_ctr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_FR_CTR_SPEC;
impl crate::RegisterSpec for RX_FR_CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fr_ctr::R`](R) reader structure"]
impl crate::Readable for RX_FR_CTR_SPEC {}
#[doc = "`reset()` method sets RX_FR_CTR to value 0"]
impl crate::Resettable for RX_FR_CTR_SPEC {}
