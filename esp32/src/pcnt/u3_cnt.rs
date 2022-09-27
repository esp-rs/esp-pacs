#[doc = "Register `U3_CNT` reader"]
pub struct R(crate::R<U3_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U3_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U3_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U3_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLUS_CNT_U3` reader - This register stores the current pulse count value for unit3."]
pub type PLUS_CNT_U3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit3."]
    #[inline(always)]
    pub fn plus_cnt_u3(&self) -> PLUS_CNT_U3_R {
        PLUS_CNT_U3_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u3_cnt](index.html) module"]
pub struct U3_CNT_SPEC;
impl crate::RegisterSpec for U3_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u3_cnt::R](R) reader structure"]
impl crate::Readable for U3_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U3_CNT to value 0"]
impl crate::Resettable for U3_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
