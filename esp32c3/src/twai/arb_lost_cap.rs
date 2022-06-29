#[doc = "Register `ARB_LOST_CAP` reader"]
pub struct R(crate::R<ARB_LOST_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_LOST_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_LOST_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_LOST_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ARB_LOST_CAP` reader - This register contains information about the bit position of lost arbitration."]
pub type ARB_LOST_CAP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - This register contains information about the bit position of lost arbitration."]
    #[inline(always)]
    pub fn arb_lost_cap(&self) -> ARB_LOST_CAP_R {
        ARB_LOST_CAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Arbitration Lost Capture Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_lost_cap](index.html) module"]
pub struct ARB_LOST_CAP_SPEC;
impl crate::RegisterSpec for ARB_LOST_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_lost_cap::R](R) reader structure"]
impl crate::Readable for ARB_LOST_CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ARB_LOST_CAP to value 0"]
impl crate::Resettable for ARB_LOST_CAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
