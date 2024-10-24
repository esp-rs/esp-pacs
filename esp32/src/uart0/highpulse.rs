#[doc = "Register `HIGHPULSE` reader"]
pub type R = crate::R<HIGHPULSE_SPEC>;
#[doc = "Field `MIN_CNT` reader - This register stores the value of the maxinum duration time for the high level pulse. it is used in baudrate-detect process."]
pub type MIN_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - This register stores the value of the maxinum duration time for the high level pulse. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn min_cnt(&self) -> MIN_CNT_R {
        MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIGHPULSE")
            .field("min_cnt", &self.min_cnt())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`highpulse::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIGHPULSE_SPEC;
impl crate::RegisterSpec for HIGHPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`highpulse::R`](R) reader structure"]
impl crate::Readable for HIGHPULSE_SPEC {}
#[doc = "`reset()` method sets HIGHPULSE to value 0x000f_ffff"]
impl crate::Resettable for HIGHPULSE_SPEC {
    const RESET_VALUE: u32 = 0x000f_ffff;
}
