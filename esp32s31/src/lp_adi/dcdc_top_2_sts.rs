#[doc = "Register `DCDC_TOP_2_STS` reader"]
pub type R = crate::R<DCDC_TOP_2_STS_SPEC>;
#[doc = "Field `DCDC_TOP_2_CAL_DONE` reader - cali done signal from dcdc 2 top"]
pub type DCDC_TOP_2_CAL_DONE_R = crate::BitReader;
#[doc = "Field `DCDC_TOP_2_IN_VCM` reader - in vcm signal from dcdc 2 top"]
pub type DCDC_TOP_2_IN_VCM_R = crate::BitReader;
#[doc = "Field `DCDC_TOP_2_IN_CCM` reader - in ccm signal from dcdc 2 top"]
pub type DCDC_TOP_2_IN_CCM_R = crate::BitReader;
#[doc = "Field `DCDC_TOP_2_VCM_READY` reader - vcm_ready signal from dcdc 2 top"]
pub type DCDC_TOP_2_VCM_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - cali done signal from dcdc 2 top"]
    #[inline(always)]
    pub fn dcdc_top_2_cal_done(&self) -> DCDC_TOP_2_CAL_DONE_R {
        DCDC_TOP_2_CAL_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - in vcm signal from dcdc 2 top"]
    #[inline(always)]
    pub fn dcdc_top_2_in_vcm(&self) -> DCDC_TOP_2_IN_VCM_R {
        DCDC_TOP_2_IN_VCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - in ccm signal from dcdc 2 top"]
    #[inline(always)]
    pub fn dcdc_top_2_in_ccm(&self) -> DCDC_TOP_2_IN_CCM_R {
        DCDC_TOP_2_IN_CCM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - vcm_ready signal from dcdc 2 top"]
    #[inline(always)]
    pub fn dcdc_top_2_vcm_ready(&self) -> DCDC_TOP_2_VCM_READY_R {
        DCDC_TOP_2_VCM_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_TOP_2_STS")
            .field("dcdc_top_2_cal_done", &self.dcdc_top_2_cal_done())
            .field("dcdc_top_2_in_vcm", &self.dcdc_top_2_in_vcm())
            .field("dcdc_top_2_in_ccm", &self.dcdc_top_2_in_ccm())
            .field("dcdc_top_2_vcm_ready", &self.dcdc_top_2_vcm_ready())
            .finish()
    }
}
#[doc = "ANALOG DCDC 2 related status\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_2_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_TOP_2_STS_SPEC;
impl crate::RegisterSpec for DCDC_TOP_2_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_2_sts::R`](R) reader structure"]
impl crate::Readable for DCDC_TOP_2_STS_SPEC {}
#[doc = "`reset()` method sets DCDC_TOP_2_STS to value 0"]
impl crate::Resettable for DCDC_TOP_2_STS_SPEC {}
