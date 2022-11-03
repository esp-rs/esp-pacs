#[doc = "Register `IN` reader"]
pub struct R(crate::R<IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LP_GPIO_IN_DATA_NEXT` reader - need des"]
pub type LP_GPIO_IN_DATA_NEXT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - need des"]
    #[inline(always)]
    pub fn lp_gpio_in_data_next(&self) -> LP_GPIO_IN_DATA_NEXT_R {
        LP_GPIO_IN_DATA_NEXT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_](index.html) module"]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_::R](R) reader structure"]
impl crate::Readable for IN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
