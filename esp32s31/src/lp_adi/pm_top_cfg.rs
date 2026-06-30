#[doc = "Register `PM_TOP_CFG` reader"]
pub type R = crate::R<PM_TOP_CFG_SPEC>;
#[doc = "Register `PM_TOP_CFG` writer"]
pub type W = crate::W<PM_TOP_CFG_SPEC>;
#[doc = "Field `PM_TOP_DREG_MODE` reader - PM TOP dreg mode"]
pub type PM_TOP_DREG_MODE_R = crate::FieldReader;
#[doc = "Field `PM_TOP_DREG_MODE` writer - PM TOP dreg mode"]
pub type PM_TOP_DREG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM_TOP_DCDC_CLR_VCM_RDY` reader - 1: clear vcm rdy signal"]
pub type PM_TOP_DCDC_CLR_VCM_RDY_R = crate::BitReader;
#[doc = "Field `PM_TOP_DCDC_CLR_VCM_RDY` writer - 1: clear vcm rdy signal"]
pub type PM_TOP_DCDC_CLR_VCM_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM_TOP_DIG_REG_DSFMOS` reader - Current limit config"]
pub type PM_TOP_DIG_REG_DSFMOS_R = crate::FieldReader;
#[doc = "Field `PM_TOP_DIG_REG_DSFMOS` writer - Current limit config"]
pub type PM_TOP_DIG_REG_DSFMOS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - PM TOP dreg mode"]
    #[inline(always)]
    pub fn pm_top_dreg_mode(&self) -> PM_TOP_DREG_MODE_R {
        PM_TOP_DREG_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1: clear vcm rdy signal"]
    #[inline(always)]
    pub fn pm_top_dcdc_clr_vcm_rdy(&self) -> PM_TOP_DCDC_CLR_VCM_RDY_R {
        PM_TOP_DCDC_CLR_VCM_RDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Current limit config"]
    #[inline(always)]
    pub fn pm_top_dig_reg_dsfmos(&self) -> PM_TOP_DIG_REG_DSFMOS_R {
        PM_TOP_DIG_REG_DSFMOS_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PM_TOP_CFG")
            .field("pm_top_dreg_mode", &self.pm_top_dreg_mode())
            .field("pm_top_dcdc_clr_vcm_rdy", &self.pm_top_dcdc_clr_vcm_rdy())
            .field("pm_top_dig_reg_dsfmos", &self.pm_top_dig_reg_dsfmos())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - PM TOP dreg mode"]
    #[inline(always)]
    pub fn pm_top_dreg_mode(&mut self) -> PM_TOP_DREG_MODE_W<'_, PM_TOP_CFG_SPEC> {
        PM_TOP_DREG_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - 1: clear vcm rdy signal"]
    #[inline(always)]
    pub fn pm_top_dcdc_clr_vcm_rdy(&mut self) -> PM_TOP_DCDC_CLR_VCM_RDY_W<'_, PM_TOP_CFG_SPEC> {
        PM_TOP_DCDC_CLR_VCM_RDY_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Current limit config"]
    #[inline(always)]
    pub fn pm_top_dig_reg_dsfmos(&mut self) -> PM_TOP_DIG_REG_DSFMOS_W<'_, PM_TOP_CFG_SPEC> {
        PM_TOP_DIG_REG_DSFMOS_W::new(self, 3)
    }
}
#[doc = "ANALOG PM TOP related CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`pm_top_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pm_top_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PM_TOP_CFG_SPEC;
impl crate::RegisterSpec for PM_TOP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pm_top_cfg::R`](R) reader structure"]
impl crate::Readable for PM_TOP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pm_top_cfg::W`](W) writer structure"]
impl crate::Writable for PM_TOP_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PM_TOP_CFG to value 0x03"]
impl crate::Resettable for PM_TOP_CFG_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
