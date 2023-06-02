#[doc = "Register `SAR_TOUCH_DENOISE` reader"]
pub struct R(crate::R<SAR_TOUCH_DENOISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_DENOISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_DENOISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_DENOISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - configure touch controller"]
pub type DATA_R = crate::FieldReader<u32, u32>;
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
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_DENOISE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "configure touch controller\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_denoise](index.html) module"]
pub struct SAR_TOUCH_DENOISE_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_DENOISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_denoise::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_DENOISE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_DENOISE to value 0"]
impl crate::Resettable for SAR_TOUCH_DENOISE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
