#[doc = "Register `T0HI` reader"]
pub struct R(crate::R<T0HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T_HI` reader - After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter of timer %s can be read here."]
pub type T_HI_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:21 - After writing to TIMG_T%sUPDATE_REG, the high 22 bits of the time-base counter of timer %s can be read here."]
    #[inline(always)]
    pub fn t_hi(&self) -> T_HI_R {
        T_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Timer %s current value, high 22 bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0hi](index.html) module"]
pub struct T0HI_SPEC;
impl crate::RegisterSpec for T0HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0hi::R](R) reader structure"]
impl crate::Readable for T0HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0HI to value 0"]
impl crate::Resettable for T0HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
