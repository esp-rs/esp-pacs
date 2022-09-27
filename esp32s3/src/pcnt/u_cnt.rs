#[doc = "Register `U%s_CNT` reader"]
pub struct R(crate::R<U_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PULSE_CNT_U` reader - This register stores the current pulse count value for unit %s."]
pub type PULSE_CNT_U_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit %s."]
    #[inline(always)]
    pub fn pulse_cnt_u(&self) -> PULSE_CNT_U_R {
        PULSE_CNT_U_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Counter value for unit %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_cnt](index.html) module"]
pub struct U_CNT_SPEC;
impl crate::RegisterSpec for U_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_cnt::R](R) reader structure"]
impl crate::Readable for U_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U%s_CNT to value 0"]
impl crate::Resettable for U_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
