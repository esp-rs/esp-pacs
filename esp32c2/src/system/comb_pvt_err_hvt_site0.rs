#[doc = "Register `COMB_PVT_ERR_HVT_SITE0` reader"]
pub struct R(crate::R<COMB_PVT_ERR_HVT_SITE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMB_PVT_ERR_HVT_SITE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMB_PVT_ERR_HVT_SITE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMB_PVT_ERR_HVT_SITE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMB_TIMING_ERR_CNT_HVT_SITE0` reader - reg_comb_timing_err_cnt_hvt_site0"]
pub type COMB_TIMING_ERR_CNT_HVT_SITE0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - reg_comb_timing_err_cnt_hvt_site0"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_hvt_site0(&self) -> COMB_TIMING_ERR_CNT_HVT_SITE0_R {
        COMB_TIMING_ERR_CNT_HVT_SITE0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "mem pvt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comb_pvt_err_hvt_site0](index.html) module"]
pub struct COMB_PVT_ERR_HVT_SITE0_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_HVT_SITE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comb_pvt_err_hvt_site0::R](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_HVT_SITE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMB_PVT_ERR_HVT_SITE0 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_HVT_SITE0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
