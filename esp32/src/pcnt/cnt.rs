#[doc = "Register `CNT%s` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLUS_CNT` reader - This register stores the current pulse count value for unit0."]
pub type PLUS_CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit0."]
    #[inline(always)]
    pub fn plus_cnt(&self) -> PLUS_CNT_R {
        PLUS_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNT%s to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
