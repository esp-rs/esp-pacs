#[doc = "Register `SAR_TOUCH_DENOISE` reader"]
pub type R = crate::R<SAR_TOUCH_DENOISE_SPEC>;
#[doc = "Field `DATA` reader - configure touch controller"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - configure touch controller"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_DENOISE")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "configure touch controller\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_denoise::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_DENOISE_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_DENOISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_denoise::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_DENOISE_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_DENOISE to value 0"]
impl crate::Resettable for SAR_TOUCH_DENOISE_SPEC {
    const RESET_VALUE: u32 = 0;
}
