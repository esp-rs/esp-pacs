#[doc = "Register `ANA_DCDC` reader"]
pub type R = crate::R<ANA_DCDC_SPEC>;
#[doc = "Register `ANA_DCDC` writer"]
pub type W = crate::W<ANA_DCDC_SPEC>;
#[doc = "Field `RAMPLEVEL_DCDC` reader - "]
pub type RAMPLEVEL_DCDC_R = crate::BitReader;
#[doc = "Field `RAMPLEVEL_DCDC` writer - "]
pub type RAMPLEVEL_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMP_DCDC` reader - "]
pub type RAMP_DCDC_R = crate::BitReader;
#[doc = "Field `RAMP_DCDC` writer - "]
pub type RAMP_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCM2ENB_DCDC` reader - "]
pub type DCM2ENB_DCDC_R = crate::BitReader;
#[doc = "Field `DCM2ENB_DCDC` writer - "]
pub type DCM2ENB_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMLEVEL_DCDC` reader - "]
pub type DCMLEVEL_DCDC_R = crate::FieldReader;
#[doc = "Field `DCMLEVEL_DCDC` writer - "]
pub type DCMLEVEL_DCDC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSW_DCDC` reader - "]
pub type FSW_DCDC_R = crate::FieldReader;
#[doc = "Field `FSW_DCDC` writer - "]
pub type FSW_DCDC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCM_DCDC` reader - "]
pub type CCM_DCDC_R = crate::BitReader;
#[doc = "Field `CCM_DCDC` writer - "]
pub type CCM_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTIME_DCDC` reader - "]
pub type SSTIME_DCDC_R = crate::BitReader;
#[doc = "Field `SSTIME_DCDC` writer - "]
pub type SSTIME_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POCPENB_DCDC` reader - "]
pub type POCPENB_DCDC_R = crate::BitReader;
#[doc = "Field `POCPENB_DCDC` writer - "]
pub type POCPENB_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_TRX_MODE_DCDC` reader - "]
pub type ENABLE_TRX_MODE_DCDC_R = crate::BitReader;
#[doc = "Field `ENABLE_TRX_MODE_DCDC` writer - "]
pub type ENABLE_TRX_MODE_DCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ramplevel_dcdc(&self) -> RAMPLEVEL_DCDC_R {
        RAMPLEVEL_DCDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ramp_dcdc(&self) -> RAMP_DCDC_R {
        RAMP_DCDC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcm2enb_dcdc(&self) -> DCM2ENB_DCDC_R {
        DCM2ENB_DCDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn dcmlevel_dcdc(&self) -> DCMLEVEL_DCDC_R {
        DCMLEVEL_DCDC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn fsw_dcdc(&self) -> FSW_DCDC_R {
        FSW_DCDC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ccm_dcdc(&self) -> CCM_DCDC_R {
        CCM_DCDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sstime_dcdc(&self) -> SSTIME_DCDC_R {
        SSTIME_DCDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pocpenb_dcdc(&self) -> POCPENB_DCDC_R {
        POCPENB_DCDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn enable_trx_mode_dcdc(&self) -> ENABLE_TRX_MODE_DCDC_R {
        ENABLE_TRX_MODE_DCDC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_DCDC")
            .field("ramplevel_dcdc", &self.ramplevel_dcdc())
            .field("ramp_dcdc", &self.ramp_dcdc())
            .field("dcm2enb_dcdc", &self.dcm2enb_dcdc())
            .field("dcmlevel_dcdc", &self.dcmlevel_dcdc())
            .field("fsw_dcdc", &self.fsw_dcdc())
            .field("ccm_dcdc", &self.ccm_dcdc())
            .field("sstime_dcdc", &self.sstime_dcdc())
            .field("pocpenb_dcdc", &self.pocpenb_dcdc())
            .field("enable_trx_mode_dcdc", &self.enable_trx_mode_dcdc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ramplevel_dcdc(&mut self) -> RAMPLEVEL_DCDC_W<'_, ANA_DCDC_SPEC> {
        RAMPLEVEL_DCDC_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ramp_dcdc(&mut self) -> RAMP_DCDC_W<'_, ANA_DCDC_SPEC> {
        RAMP_DCDC_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcm2enb_dcdc(&mut self) -> DCM2ENB_DCDC_W<'_, ANA_DCDC_SPEC> {
        DCM2ENB_DCDC_W::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn dcmlevel_dcdc(&mut self) -> DCMLEVEL_DCDC_W<'_, ANA_DCDC_SPEC> {
        DCMLEVEL_DCDC_W::new(self, 3)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn fsw_dcdc(&mut self) -> FSW_DCDC_W<'_, ANA_DCDC_SPEC> {
        FSW_DCDC_W::new(self, 5)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ccm_dcdc(&mut self) -> CCM_DCDC_W<'_, ANA_DCDC_SPEC> {
        CCM_DCDC_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sstime_dcdc(&mut self) -> SSTIME_DCDC_W<'_, ANA_DCDC_SPEC> {
        SSTIME_DCDC_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pocpenb_dcdc(&mut self) -> POCPENB_DCDC_W<'_, ANA_DCDC_SPEC> {
        POCPENB_DCDC_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn enable_trx_mode_dcdc(&mut self) -> ENABLE_TRX_MODE_DCDC_W<'_, ANA_DCDC_SPEC> {
        ENABLE_TRX_MODE_DCDC_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_dcdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_dcdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_DCDC_SPEC;
impl crate::RegisterSpec for ANA_DCDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_dcdc::R`](R) reader structure"]
impl crate::Readable for ANA_DCDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_dcdc::W`](W) writer structure"]
impl crate::Writable for ANA_DCDC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANA_DCDC to value 0x0800"]
impl crate::Resettable for ANA_DCDC_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
