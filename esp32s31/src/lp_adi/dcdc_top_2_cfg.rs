#[doc = "Register `DCDC_TOP_2_CFG` reader"]
pub type R = crate::R<DCDC_TOP_2_CFG_SPEC>;
#[doc = "Register `DCDC_TOP_2_CFG` writer"]
pub type W = crate::W<DCDC_TOP_2_CFG_SPEC>;
#[doc = "Field `DCDC_TOP_2_DREG_MODE` reader - DCDC 2 dreg mode"]
pub type DCDC_TOP_2_DREG_MODE_R = crate::FieldReader;
#[doc = "Field `DCDC_TOP_2_DREG_MODE` writer - DCDC 2 dreg mode"]
pub type DCDC_TOP_2_DREG_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCDC_TOP_2_CLEAR_VCM_RDY` reader - 1: clear vcm rdy signal"]
pub type DCDC_TOP_2_CLEAR_VCM_RDY_R = crate::BitReader;
#[doc = "Field `DCDC_TOP_2_CLEAR_VCM_RDY` writer - 1: clear vcm rdy signal"]
pub type DCDC_TOP_2_CLEAR_VCM_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_TOP_2_IFSM_PCUR_LIMIT` reader - Current limit config"]
pub type DCDC_TOP_2_IFSM_PCUR_LIMIT_R = crate::FieldReader;
#[doc = "Field `DCDC_TOP_2_IFSM_PCUR_LIMIT` writer - Current limit config"]
pub type DCDC_TOP_2_IFSM_PCUR_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - DCDC 2 dreg mode"]
    #[inline(always)]
    pub fn dcdc_top_2_dreg_mode(&self) -> DCDC_TOP_2_DREG_MODE_R {
        DCDC_TOP_2_DREG_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1: clear vcm rdy signal"]
    #[inline(always)]
    pub fn dcdc_top_2_clear_vcm_rdy(&self) -> DCDC_TOP_2_CLEAR_VCM_RDY_R {
        DCDC_TOP_2_CLEAR_VCM_RDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Current limit config"]
    #[inline(always)]
    pub fn dcdc_top_2_ifsm_pcur_limit(&self) -> DCDC_TOP_2_IFSM_PCUR_LIMIT_R {
        DCDC_TOP_2_IFSM_PCUR_LIMIT_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDC_TOP_2_CFG")
            .field("dcdc_top_2_dreg_mode", &self.dcdc_top_2_dreg_mode())
            .field("dcdc_top_2_clear_vcm_rdy", &self.dcdc_top_2_clear_vcm_rdy())
            .field(
                "dcdc_top_2_ifsm_pcur_limit",
                &self.dcdc_top_2_ifsm_pcur_limit(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - DCDC 2 dreg mode"]
    #[inline(always)]
    pub fn dcdc_top_2_dreg_mode(&mut self) -> DCDC_TOP_2_DREG_MODE_W<'_, DCDC_TOP_2_CFG_SPEC> {
        DCDC_TOP_2_DREG_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - 1: clear vcm rdy signal"]
    #[inline(always)]
    pub fn dcdc_top_2_clear_vcm_rdy(
        &mut self,
    ) -> DCDC_TOP_2_CLEAR_VCM_RDY_W<'_, DCDC_TOP_2_CFG_SPEC> {
        DCDC_TOP_2_CLEAR_VCM_RDY_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Current limit config"]
    #[inline(always)]
    pub fn dcdc_top_2_ifsm_pcur_limit(
        &mut self,
    ) -> DCDC_TOP_2_IFSM_PCUR_LIMIT_W<'_, DCDC_TOP_2_CFG_SPEC> {
        DCDC_TOP_2_IFSM_PCUR_LIMIT_W::new(self, 3)
    }
}
#[doc = "ANALOG DCDC 2 related CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_top_2_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_top_2_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDC_TOP_2_CFG_SPEC;
impl crate::RegisterSpec for DCDC_TOP_2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdc_top_2_cfg::R`](R) reader structure"]
impl crate::Readable for DCDC_TOP_2_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcdc_top_2_cfg::W`](W) writer structure"]
impl crate::Writable for DCDC_TOP_2_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDC_TOP_2_CFG to value 0x03"]
impl crate::Resettable for DCDC_TOP_2_CFG_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
