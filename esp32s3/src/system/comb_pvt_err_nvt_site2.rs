#[doc = "Register `COMB_PVT_ERR_NVT_SITE2` reader"]
pub struct R(crate::R<COMB_PVT_ERR_NVT_SITE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMB_PVT_ERR_NVT_SITE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMB_PVT_ERR_NVT_SITE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMB_PVT_ERR_NVT_SITE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COMB_TIMING_ERR_CNT_NVT_SITE2` reader - ******* Description ***********"]
pub type COMB_TIMING_ERR_CNT_NVT_SITE2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_nvt_site2(&self) -> COMB_TIMING_ERR_CNT_NVT_SITE2_R {
        COMB_TIMING_ERR_CNT_NVT_SITE2_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_ERR_NVT_SITE2")
            .field(
                "comb_timing_err_cnt_nvt_site2",
                &format_args!("{}", self.comb_timing_err_cnt_nvt_site2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PVT_ERR_NVT_SITE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comb_pvt_err_nvt_site2](index.html) module"]
pub struct COMB_PVT_ERR_NVT_SITE2_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_NVT_SITE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comb_pvt_err_nvt_site2::R](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_NVT_SITE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMB_PVT_ERR_NVT_SITE2 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_NVT_SITE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
