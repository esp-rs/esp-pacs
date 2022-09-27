#[doc = "Register `T%sHI` reader"]
pub struct R(crate::R<THI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HI` reader - After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter of timer %s can be read here."]
pub type HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
#[doc = "Timer %s current value, high 22 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thi](index.html) module"]
pub struct THI_SPEC;
impl crate::RegisterSpec for THI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thi::R](R) reader structure"]
impl crate::Readable for THI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T%sHI to value 0"]
impl crate::Resettable for THI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
