#[doc = "Register `COMB_PVT_ERR_LVT_SITE2` reader"]
pub type R = crate::R<COMB_PVT_ERR_LVT_SITE2_SPEC>;
#[doc = "Field `COMB_TIMING_ERR_CNT_LVT_SITE2` reader - reg_comb_timing_err_cnt_lvt_site2"]
pub type COMB_TIMING_ERR_CNT_LVT_SITE2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - reg_comb_timing_err_cnt_lvt_site2"]
    #[inline(always)]
    pub fn comb_timing_err_cnt_lvt_site2(&self) -> COMB_TIMING_ERR_CNT_LVT_SITE2_R {
        COMB_TIMING_ERR_CNT_LVT_SITE2_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMB_PVT_ERR_LVT_SITE2")
            .field(
                "comb_timing_err_cnt_lvt_site2",
                &format_args!("{}", self.comb_timing_err_cnt_lvt_site2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMB_PVT_ERR_LVT_SITE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "mem pvt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comb_pvt_err_lvt_site2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMB_PVT_ERR_LVT_SITE2_SPEC;
impl crate::RegisterSpec for COMB_PVT_ERR_LVT_SITE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pvt_err_lvt_site2::R`](R) reader structure"]
impl crate::Readable for COMB_PVT_ERR_LVT_SITE2_SPEC {}
#[doc = "`reset()` method sets COMB_PVT_ERR_LVT_SITE2 to value 0"]
impl crate::Resettable for COMB_PVT_ERR_LVT_SITE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
