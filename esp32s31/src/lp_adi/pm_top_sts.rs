#[doc = "Register `PM_TOP_STS` reader"]
pub type R = crate::R<PM_TOP_STS_SPEC>;
#[doc = "Field `PM_TOP_CAL_DONE` reader - cali done signal from PM TOP"]
pub type PM_TOP_CAL_DONE_R = crate::BitReader;
#[doc = "Field `PM_TOP_DCDC_VCM_RDY` reader - vcm_ready signal from PM TOP"]
pub type PM_TOP_DCDC_VCM_RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cali done signal from PM TOP"]
    #[inline(always)]
    pub fn pm_top_cal_done(&self) -> PM_TOP_CAL_DONE_R {
        PM_TOP_CAL_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - vcm_ready signal from PM TOP"]
    #[inline(always)]
    pub fn pm_top_dcdc_vcm_rdy(&self) -> PM_TOP_DCDC_VCM_RDY_R {
        PM_TOP_DCDC_VCM_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PM_TOP_STS")
            .field("pm_top_cal_done", &self.pm_top_cal_done())
            .field("pm_top_dcdc_vcm_rdy", &self.pm_top_dcdc_vcm_rdy())
            .finish()
    }
}
#[doc = "ANALOG PM TOP related status\n\nYou can [`read`](crate::Reg::read) this register and get [`pm_top_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PM_TOP_STS_SPEC;
impl crate::RegisterSpec for PM_TOP_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pm_top_sts::R`](R) reader structure"]
impl crate::Readable for PM_TOP_STS_SPEC {}
#[doc = "`reset()` method sets PM_TOP_STS to value 0"]
impl crate::Resettable for PM_TOP_STS_SPEC {}
