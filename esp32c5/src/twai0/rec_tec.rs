#[doc = "Register `REC_TEC` reader"]
pub type R = crate::R<REC_TEC_SPEC>;
#[doc = "Field `REC_VAL` reader - Represents the receiver error counter value."]
pub type REC_VAL_R = crate::FieldReader<u16>;
#[doc = "Field `TEC_VAL` reader - Represents the transmitter error counter value."]
pub type TEC_VAL_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Represents the receiver error counter value."]
    #[inline(always)]
    pub fn rec_val(&self) -> REC_VAL_R {
        REC_VAL_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Represents the transmitter error counter value."]
    #[inline(always)]
    pub fn tec_val(&self) -> TEC_VAL_R {
        TEC_VAL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REC_TEC")
            .field("rec_val", &self.rec_val())
            .field("tec_val", &self.tec_val())
            .finish()
    }
}
#[doc = "TWAI FD error counters status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rec_tec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REC_TEC_SPEC;
impl crate::RegisterSpec for REC_TEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rec_tec::R`](R) reader structure"]
impl crate::Readable for REC_TEC_SPEC {}
#[doc = "`reset()` method sets REC_TEC to value 0"]
impl crate::Resettable for REC_TEC_SPEC {
    const RESET_VALUE: u32 = 0;
}
