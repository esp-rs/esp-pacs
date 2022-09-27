#[doc = "Register `_0_RXLINK_DSCR_BF0` reader"]
pub struct R(crate::R<_0_RXLINK_DSCR_BF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_RXLINK_DSCR_BF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_RXLINK_DSCR_BF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_RXLINK_DSCR_BF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_RXLINK_DSCR_BF0` reader - "]
pub type SLC0_RXLINK_DSCR_BF0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_rxlink_dscr_bf0(&self) -> SLC0_RXLINK_DSCR_BF0_R {
        SLC0_RXLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_rxlink_dscr_bf0](index.html) module"]
pub struct _0_RXLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for _0_RXLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_rxlink_dscr_bf0::R](R) reader structure"]
impl crate::Readable for _0_RXLINK_DSCR_BF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _0_RXLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for _0_RXLINK_DSCR_BF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
