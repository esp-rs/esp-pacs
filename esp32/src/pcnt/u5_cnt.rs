#[doc = "Register `U5_CNT` reader"]
pub struct R(crate::R<U5_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U5_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U5_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U5_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLUS_CNT_U5` reader - This register stores the current pulse count value for unit5."]
pub type PLUS_CNT_U5_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit5."]
    #[inline(always)]
    pub fn plus_cnt_u5(&self) -> PLUS_CNT_U5_R {
        PLUS_CNT_U5_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u5_cnt](index.html) module"]
pub struct U5_CNT_SPEC;
impl crate::RegisterSpec for U5_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u5_cnt::R](R) reader structure"]
impl crate::Readable for U5_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U5_CNT to value 0"]
impl crate::Resettable for U5_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
