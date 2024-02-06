#[doc = "Register `COMB_PVT_ERR_HVT_SITE1` reader"]
pub type R = crate::R<COMB_PVT_ERR_HVT_SITE1_SPEC>;
#[doc = "Field `COMB_TIMING_ERR_CNT_HVT_SITE1` reader - reg_comb_timing_err_cnt_hvt_site1"]
pub type COMB_TIMING_ERR_CNT_HVT_SITE1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - reg_comb_timing_err_cnt_hvt_site1"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_hvt_site1(&self) -> COMB_TIMING_ERR_CNT_HVT_SITE1_R {
        COMB_TIMING_ERR_CNT_HVT_SITE1_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_ERR_HVT_SITE1")
            .field(
                "comb_timing_err_cnt_hvt_site1",
                &format_args!("{}", self.comb_timing_err_cnt_hvt_site1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PVT_ERR_HVT_SITE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "mem pvt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_hvt_site1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PVT_ERR_HVT_SITE1_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_HVT_SITE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pvt_err_hvt_site1::R`](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_HVT_SITE1_SPEC {}
#[doc = "`reset()` method sets COMB_PVT_ERR_HVT_SITE1 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_HVT_SITE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
