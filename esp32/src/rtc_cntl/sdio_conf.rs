#[doc = "Register `SDIO_CONF` reader"]
pub type R = crate::R<SDIO_CONF_SPEC>;
#[doc = "Register `SDIO_CONF` writer"]
pub type W = crate::W<SDIO_CONF_SPEC>;
#[doc = "Field `SDIO_PD_EN` reader - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub type SDIO_PD_EN_R = crate::BitReader;
#[doc = "Field `SDIO_PD_EN` writer - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub type SDIO_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_FORCE` reader - 1: use SW option to control SDIO_REG 0: use state machine"]
pub type SDIO_FORCE_R = crate::BitReader;
#[doc = "Field `SDIO_FORCE` writer - 1: use SW option to control SDIO_REG 0: use state machine"]
pub type SDIO_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_TIEH` reader - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub type SDIO_TIEH_R = crate::BitReader;
#[doc = "Field `SDIO_TIEH` writer - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub type SDIO_TIEH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG1P8_READY` reader - read only register for REG1P8_READY"]
pub type REG1P8_READY_R = crate::BitReader;
#[doc = "Field `DREFL_SDIO` reader - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFL_SDIO_R = crate::FieldReader;
#[doc = "Field `DREFL_SDIO` writer - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFL_SDIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DREFM_SDIO` reader - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFM_SDIO_R = crate::FieldReader;
#[doc = "Field `DREFM_SDIO` writer - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFM_SDIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DREFH_SDIO` reader - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFH_SDIO_R = crate::FieldReader;
#[doc = "Field `DREFH_SDIO` writer - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFH_SDIO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XPD_SDIO` reader - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
pub type XPD_SDIO_R = crate::BitReader;
#[doc = "Field `XPD_SDIO` writer - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
pub type XPD_SDIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn sdio_pd_en(&self) -> SDIO_PD_EN_R {
        SDIO_PD_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG 0: use state machine"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - read only register for REG1P8_READY"]
    #[inline(always)]
    pub fn reg1p8_ready(&self) -> REG1P8_READY_R {
        REG1P8_READY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefl_sdio(&self) -> DREFL_SDIO_R {
        DREFL_SDIO_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefm_sdio(&self) -> DREFM_SDIO_R {
        DREFM_SDIO_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn drefh_sdio(&self) -> DREFH_SDIO_R {
        DREFH_SDIO_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CONF")
            .field("sdio_pd_en", &self.sdio_pd_en())
            .field("sdio_force", &self.sdio_force())
            .field("sdio_tieh", &self.sdio_tieh())
            .field("reg1p8_ready", &self.reg1p8_ready())
            .field("drefl_sdio", &self.drefl_sdio())
            .field("drefm_sdio", &self.drefm_sdio())
            .field("drefh_sdio", &self.drefh_sdio())
            .field("xpd_sdio", &self.xpd_sdio())
            .finish()
    }
}
impl W {
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_pd_en(&mut self) -> SDIO_PD_EN_W<SDIO_CONF_SPEC> {
        SDIO_PD_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG 0: use state machine"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W<SDIO_CONF_SPEC> {
        SDIO_FORCE_W::new(self, 22)
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W<SDIO_CONF_SPEC> {
        SDIO_TIEH_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn drefl_sdio(&mut self) -> DREFL_SDIO_W<SDIO_CONF_SPEC> {
        DREFL_SDIO_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn drefm_sdio(&mut self) -> DREFM_SDIO_W<SDIO_CONF_SPEC> {
        DREFM_SDIO_W::new(self, 27)
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn drefh_sdio(&mut self) -> DREFH_SDIO_W<SDIO_CONF_SPEC> {
        DREFH_SDIO_W::new(self, 29)
    }
    #[doc = "Bit 31 - SW option for XPD_SDIO_REG. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sdio(&mut self) -> XPD_SDIO_W<SDIO_CONF_SPEC> {
        XPD_SDIO_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_conf::R`](R) reader structure"]
impl crate::Readable for SDIO_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_conf::W`](W) writer structure"]
impl crate::Writable for SDIO_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_CONF to value 0x02a0_0000"]
impl crate::Resettable for SDIO_CONF_SPEC {
    const RESET_VALUE: u32 = 0x02a0_0000;
}
