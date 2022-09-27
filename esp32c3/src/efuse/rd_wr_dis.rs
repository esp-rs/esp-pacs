#[doc = "Register `RD_WR_DIS` reader"]
pub struct R(crate::R<RD_WR_DIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_WR_DIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_WR_DIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_WR_DIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WR_DIS` reader - Disable programming of individual eFuses."]
pub type WR_DIS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Disable programming of individual eFuses."]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new(self.bits)
    }
}
#[doc = "BLOCK0 data register 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_wr_dis](index.html) module"]
pub struct RD_WR_DIS_SPEC;
impl crate::RegisterSpec for RD_WR_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_wr_dis::R](R) reader structure"]
impl crate::Readable for RD_WR_DIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_WR_DIS to value 0"]
impl crate::Resettable for RD_WR_DIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
