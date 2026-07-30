#[doc = "Register `DCM_CTRL` reader"]
pub type R = crate::R<DCM_CTRL_SPEC>;
#[doc = "Register `DCM_CTRL` writer"]
pub type W = crate::W<DCM_CTRL_SPEC>;
#[doc = "Field `DSFMOS_USE_POR` reader - need_des"]
pub type DSFMOS_USE_POR_R = crate::BitReader;
#[doc = "Field `DSFMOS_USE_POR` writer - need_des"]
pub type DSFMOS_USE_POR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_DCM_UPDATE` writer - need_des"]
pub type DCDC_DCM_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_PCUR_LIMIT` reader - need_des"]
pub type DCDC_PCUR_LIMIT_R = crate::FieldReader;
#[doc = "Field `DCDC_PCUR_LIMIT` writer - need_des"]
pub type DCDC_PCUR_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCDC_BIAS_CAL_DONE` reader - need_des"]
pub type DCDC_BIAS_CAL_DONE_R = crate::BitReader;
#[doc = "Field `DCDC_CCM_SW_EN` reader - need_des"]
pub type DCDC_CCM_SW_EN_R = crate::BitReader;
#[doc = "Field `DCDC_CCM_SW_EN` writer - need_des"]
pub type DCDC_CCM_SW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_VCM_ENB` reader - need_des"]
pub type DCDC_VCM_ENB_R = crate::BitReader;
#[doc = "Field `DCDC_VCM_ENB` writer - need_des"]
pub type DCDC_VCM_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_CCM_RDY` reader - need_des"]
pub type DCDC_CCM_RDY_R = crate::BitReader;
#[doc = "Field `DCDC_VCM_RDY` reader - need_des"]
pub type DCDC_VCM_RDY_R = crate::BitReader;
#[doc = "Field `DCDC_RDY_CLR` reader - need_des"]
pub type DCDC_RDY_CLR_R = crate::BitReader;
#[doc = "Field `DCDC_RDY_CLR` writer - need_des"]
pub type DCDC_RDY_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn dsfmos_use_por(&self) -> DSFMOS_USE_POR_R {
        DSFMOS_USE_POR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn dcdc_pcur_limit(&self) -> DCDC_PCUR_LIMIT_R {
        DCDC_PCUR_LIMIT_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn dcdc_bias_cal_done(&self) -> DCDC_BIAS_CAL_DONE_R {
        DCDC_BIAS_CAL_DONE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn dcdc_ccm_sw_en(&self) -> DCDC_CCM_SW_EN_R {
        DCDC_CCM_SW_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn dcdc_vcm_enb(&self) -> DCDC_VCM_ENB_R {
        DCDC_VCM_ENB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn dcdc_ccm_rdy(&self) -> DCDC_CCM_RDY_R {
        DCDC_CCM_RDY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn dcdc_vcm_rdy(&self) -> DCDC_VCM_RDY_R {
        DCDC_VCM_RDY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn dcdc_rdy_clr(&self) -> DCDC_RDY_CLR_R {
        DCDC_RDY_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCM_CTRL")
            .field("dsfmos_use_por", &self.dsfmos_use_por())
            .field("dcdc_pcur_limit", &self.dcdc_pcur_limit())
            .field("dcdc_bias_cal_done", &self.dcdc_bias_cal_done())
            .field("dcdc_ccm_sw_en", &self.dcdc_ccm_sw_en())
            .field("dcdc_vcm_enb", &self.dcdc_vcm_enb())
            .field("dcdc_ccm_rdy", &self.dcdc_ccm_rdy())
            .field("dcdc_vcm_rdy", &self.dcdc_vcm_rdy())
            .field("dcdc_rdy_clr", &self.dcdc_rdy_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn dsfmos_use_por(&mut self) -> DSFMOS_USE_POR_W<'_, DCM_CTRL_SPEC> {
        DSFMOS_USE_POR_W::new(self, 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn dcdc_dcm_update(&mut self) -> DCDC_DCM_UPDATE_W<'_, DCM_CTRL_SPEC> {
        DCDC_DCM_UPDATE_W::new(self, 22)
    }
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn dcdc_pcur_limit(&mut self) -> DCDC_PCUR_LIMIT_W<'_, DCM_CTRL_SPEC> {
        DCDC_PCUR_LIMIT_W::new(self, 23)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn dcdc_ccm_sw_en(&mut self) -> DCDC_CCM_SW_EN_W<'_, DCM_CTRL_SPEC> {
        DCDC_CCM_SW_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn dcdc_vcm_enb(&mut self) -> DCDC_VCM_ENB_W<'_, DCM_CTRL_SPEC> {
        DCDC_VCM_ENB_W::new(self, 28)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn dcdc_rdy_clr(&mut self) -> DCDC_RDY_CLR_W<'_, DCM_CTRL_SPEC> {
        DCDC_RDY_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dcm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCM_CTRL_SPEC;
impl crate::RegisterSpec for DCM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcm_ctrl::R`](R) reader structure"]
impl crate::Readable for DCM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcm_ctrl::W`](W) writer structure"]
impl crate::Writable for DCM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCM_CTRL to value 0x4480_0001"]
impl crate::Resettable for DCM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4480_0001;
}
