#[doc = "Register `SDIO_CONF` reader"]
pub struct R(crate::R<SDIO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CONF` writer"]
pub struct W(crate::W<SDIO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDIO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_TIMER_TARGET` reader - timer count to apply reg_sdio_dcap after sdio power on"]
pub type SDIO_TIMER_TARGET_R = crate::FieldReader;
#[doc = "Field `SDIO_TIMER_TARGET` writer - timer count to apply reg_sdio_dcap after sdio power on"]
pub type SDIO_TIMER_TARGET_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 8, O>;
#[doc = "Field `SDIO_DTHDRV` reader - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current, set to 3 after several us."]
pub type SDIO_DTHDRV_R = crate::FieldReader;
#[doc = "Field `SDIO_DTHDRV` writer - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current, set to 3 after several us."]
pub type SDIO_DTHDRV_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 2, O>;
#[doc = "Field `SDIO_DCAP` reader - ability to prevent LDO from overshoot"]
pub type SDIO_DCAP_R = crate::FieldReader;
#[doc = "Field `SDIO_DCAP` writer - ability to prevent LDO from overshoot"]
pub type SDIO_DCAP_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 2, O>;
#[doc = "Field `SDIO_INITI` reader - add resistor from ldo output to ground. 0: no res, 1: 6k,2:4k,3:2k"]
pub type SDIO_INITI_R = crate::FieldReader;
#[doc = "Field `SDIO_INITI` writer - add resistor from ldo output to ground. 0: no res, 1: 6k,2:4k,3:2k"]
pub type SDIO_INITI_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 2, O>;
#[doc = "Field `SDIO_EN_INITI` reader - 0 to set init\\[1:0\\]=0"]
pub type SDIO_EN_INITI_R = crate::BitReader;
#[doc = "Field `SDIO_EN_INITI` writer - 0 to set init\\[1:0\\]=0"]
pub type SDIO_EN_INITI_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
#[doc = "Field `SDIO_DCURLIM` reader - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
pub type SDIO_DCURLIM_R = crate::FieldReader;
#[doc = "Field `SDIO_DCURLIM` writer - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
pub type SDIO_DCURLIM_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 3, O>;
#[doc = "Field `SDIO_MODECURLIM` reader - select current limit mode"]
pub type SDIO_MODECURLIM_R = crate::BitReader;
#[doc = "Field `SDIO_MODECURLIM` writer - select current limit mode"]
pub type SDIO_MODECURLIM_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
#[doc = "Field `SDIO_ENCURLIM` reader - enable current limit"]
pub type SDIO_ENCURLIM_R = crate::BitReader;
#[doc = "Field `SDIO_ENCURLIM` writer - enable current limit"]
pub type SDIO_ENCURLIM_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
#[doc = "Field `SDIO_REG_PD_EN` reader - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub type SDIO_REG_PD_EN_R = crate::BitReader;
#[doc = "Field `SDIO_REG_PD_EN` writer - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
pub type SDIO_REG_PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
#[doc = "Field `SDIO_FORCE` reader - 1: use SW option to control SDIO_REG, 0: use state machine"]
pub type SDIO_FORCE_R = crate::BitReader;
#[doc = "Field `SDIO_FORCE` writer - 1: use SW option to control SDIO_REG, 0: use state machine"]
pub type SDIO_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
#[doc = "Field `SDIO_TIEH` reader - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub type SDIO_TIEH_R = crate::BitReader;
#[doc = "Field `SDIO_TIEH` writer - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
pub type SDIO_TIEH_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
#[doc = "Field `REG1P8_READY` reader - read only register for REG1P8_READY"]
pub type REG1P8_READY_R = crate::BitReader;
#[doc = "Field `DREFL_SDIO` reader - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFL_SDIO_R = crate::FieldReader;
#[doc = "Field `DREFL_SDIO` writer - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFL_SDIO_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 2, O>;
#[doc = "Field `DREFM_SDIO` reader - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFM_SDIO_R = crate::FieldReader;
#[doc = "Field `DREFM_SDIO` writer - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFM_SDIO_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 2, O>;
#[doc = "Field `DREFH_SDIO` reader - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFH_SDIO_R = crate::FieldReader;
#[doc = "Field `DREFH_SDIO` writer - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
pub type DREFH_SDIO_W<'a, const O: u8> = crate::FieldWriter<'a, SDIO_CONF_SPEC, 2, O>;
#[doc = "Field `XPD_SDIO` reader - power on flash regulator"]
pub type XPD_SDIO_R = crate::BitReader;
#[doc = "Field `XPD_SDIO` writer - power on flash regulator"]
pub type XPD_SDIO_W<'a, const O: u8> = crate::BitWriter<'a, SDIO_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - timer count to apply reg_sdio_dcap after sdio power on"]
    #[inline(always)]
    pub fn sdio_timer_target(&self) -> SDIO_TIMER_TARGET_R {
        SDIO_TIMER_TARGET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:10 - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current, set to 3 after several us."]
    #[inline(always)]
    pub fn sdio_dthdrv(&self) -> SDIO_DTHDRV_R {
        SDIO_DTHDRV_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - ability to prevent LDO from overshoot"]
    #[inline(always)]
    pub fn sdio_dcap(&self) -> SDIO_DCAP_R {
        SDIO_DCAP_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - add resistor from ldo output to ground. 0: no res, 1: 6k,2:4k,3:2k"]
    #[inline(always)]
    pub fn sdio_initi(&self) -> SDIO_INITI_R {
        SDIO_INITI_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - 0 to set init\\[1:0\\]=0"]
    #[inline(always)]
    pub fn sdio_en_initi(&self) -> SDIO_EN_INITI_R {
        SDIO_EN_INITI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
    #[inline(always)]
    pub fn sdio_dcurlim(&self) -> SDIO_DCURLIM_R {
        SDIO_DCURLIM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - select current limit mode"]
    #[inline(always)]
    pub fn sdio_modecurlim(&self) -> SDIO_MODECURLIM_R {
        SDIO_MODECURLIM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable current limit"]
    #[inline(always)]
    pub fn sdio_encurlim(&self) -> SDIO_ENCURLIM_R {
        SDIO_ENCURLIM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    pub fn sdio_reg_pd_en(&self) -> SDIO_REG_PD_EN_R {
        SDIO_REG_PD_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG, 0: use state machine"]
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
    #[doc = "Bit 31 - power on flash regulator"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_CONF")
            .field(
                "sdio_timer_target",
                &format_args!("{}", self.sdio_timer_target().bits()),
            )
            .field(
                "sdio_dthdrv",
                &format_args!("{}", self.sdio_dthdrv().bits()),
            )
            .field("sdio_dcap", &format_args!("{}", self.sdio_dcap().bits()))
            .field("sdio_initi", &format_args!("{}", self.sdio_initi().bits()))
            .field(
                "sdio_en_initi",
                &format_args!("{}", self.sdio_en_initi().bit()),
            )
            .field(
                "sdio_dcurlim",
                &format_args!("{}", self.sdio_dcurlim().bits()),
            )
            .field(
                "sdio_modecurlim",
                &format_args!("{}", self.sdio_modecurlim().bit()),
            )
            .field(
                "sdio_encurlim",
                &format_args!("{}", self.sdio_encurlim().bit()),
            )
            .field(
                "sdio_reg_pd_en",
                &format_args!("{}", self.sdio_reg_pd_en().bit()),
            )
            .field("sdio_force", &format_args!("{}", self.sdio_force().bit()))
            .field("sdio_tieh", &format_args!("{}", self.sdio_tieh().bit()))
            .field(
                "reg1p8_ready",
                &format_args!("{}", self.reg1p8_ready().bit()),
            )
            .field("drefl_sdio", &format_args!("{}", self.drefl_sdio().bits()))
            .field("drefm_sdio", &format_args!("{}", self.drefm_sdio().bits()))
            .field("drefh_sdio", &format_args!("{}", self.drefh_sdio().bits()))
            .field("xpd_sdio", &format_args!("{}", self.xpd_sdio().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - timer count to apply reg_sdio_dcap after sdio power on"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_timer_target(&mut self) -> SDIO_TIMER_TARGET_W<0> {
        SDIO_TIMER_TARGET_W::new(self)
    }
    #[doc = "Bits 9:10 - Tieh = 1 mode drive ability. Initially set to 0 to limit charge current, set to 3 after several us."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_dthdrv(&mut self) -> SDIO_DTHDRV_W<9> {
        SDIO_DTHDRV_W::new(self)
    }
    #[doc = "Bits 11:12 - ability to prevent LDO from overshoot"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_dcap(&mut self) -> SDIO_DCAP_W<11> {
        SDIO_DCAP_W::new(self)
    }
    #[doc = "Bits 13:14 - add resistor from ldo output to ground. 0: no res, 1: 6k,2:4k,3:2k"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_initi(&mut self) -> SDIO_INITI_W<13> {
        SDIO_INITI_W::new(self)
    }
    #[doc = "Bit 15 - 0 to set init\\[1:0\\]=0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_en_initi(&mut self) -> SDIO_EN_INITI_W<15> {
        SDIO_EN_INITI_W::new(self)
    }
    #[doc = "Bits 16:18 - tune current limit threshold when tieh = 0. About 800mA/(8+d)"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_dcurlim(&mut self) -> SDIO_DCURLIM_W<16> {
        SDIO_DCURLIM_W::new(self)
    }
    #[doc = "Bit 19 - select current limit mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_modecurlim(&mut self) -> SDIO_MODECURLIM_W<19> {
        SDIO_MODECURLIM_W::new(self)
    }
    #[doc = "Bit 20 - enable current limit"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_encurlim(&mut self) -> SDIO_ENCURLIM_W<20> {
        SDIO_ENCURLIM_W::new(self)
    }
    #[doc = "Bit 21 - power down SDIO_REG in sleep. Only active when reg_sdio_force = 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_reg_pd_en(&mut self) -> SDIO_REG_PD_EN_W<21> {
        SDIO_REG_PD_EN_W::new(self)
    }
    #[doc = "Bit 22 - 1: use SW option to control SDIO_REG, 0: use state machine"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W<22> {
        SDIO_FORCE_W::new(self)
    }
    #[doc = "Bit 23 - SW option for SDIO_TIEH. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W<23> {
        SDIO_TIEH_W::new(self)
    }
    #[doc = "Bits 25:26 - SW option for DREFL_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn drefl_sdio(&mut self) -> DREFL_SDIO_W<25> {
        DREFL_SDIO_W::new(self)
    }
    #[doc = "Bits 27:28 - SW option for DREFM_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn drefm_sdio(&mut self) -> DREFM_SDIO_W<27> {
        DREFM_SDIO_W::new(self)
    }
    #[doc = "Bits 29:30 - SW option for DREFH_SDIO. Only active when reg_sdio_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn drefh_sdio(&mut self) -> DREFH_SDIO_W<29> {
        DREFH_SDIO_W::new(self)
    }
    #[doc = "Bit 31 - power on flash regulator"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sdio(&mut self) -> XPD_SDIO_W<31> {
        XPD_SDIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure flash power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_conf](index.html) module"]
pub struct SDIO_CONF_SPEC;
impl crate::RegisterSpec for SDIO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_conf::R](R) reader structure"]
impl crate::Readable for SDIO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_conf::W](W) writer structure"]
impl crate::Writable for SDIO_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_CONF to value 0x0ab0_be0a"]
impl crate::Resettable for SDIO_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0ab0_be0a;
}
