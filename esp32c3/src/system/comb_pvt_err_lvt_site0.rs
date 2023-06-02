#[doc = "Register `COMB_PVT_ERR_LVT_SITE0` reader"]
pub struct R(crate::R<COMB_PVT_ERR_LVT_SITE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMB_PVT_ERR_LVT_SITE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMB_PVT_ERR_LVT_SITE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMB_PVT_ERR_LVT_SITE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMB_TIMING_ERR_CNT_LVT_SITE0` reader - reg_comb_timing_err_cnt_lvt_site0"]
pub type COMB_TIMING_ERR_CNT_LVT_SITE0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - reg_comb_timing_err_cnt_lvt_site0"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_lvt_site0(&self) -> COMB_TIMING_ERR_CNT_LVT_SITE0_R {
        COMB_TIMING_ERR_CNT_LVT_SITE0_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_ERR_LVT_SITE0")
            .field(
                "comb_timing_err_cnt_lvt_site0",
                &format_args!("{}", self.comb_timing_err_cnt_lvt_site0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PVT_ERR_LVT_SITE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "mem pvt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comb_pvt_err_lvt_site0](index.html) module"]
pub struct COMB_PVT_ERR_LVT_SITE0_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_LVT_SITE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comb_pvt_err_lvt_site0::R](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_LVT_SITE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMB_PVT_ERR_LVT_SITE0 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_LVT_SITE0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
