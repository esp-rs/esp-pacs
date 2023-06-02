#[doc = "Register `CONF_STATUS` reader"]
pub struct R(crate::R<CONF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FUNC0_CONFIG0` reader - func0 config0 (addr: 0x20f0 ) status"]
pub type FUNC0_CONFIG0_R = crate::FieldReader;
#[doc = "Field `SDR25_ST` reader - sdr25 status"]
pub type SDR25_ST_R = crate::BitReader;
#[doc = "Field `SDR50_ST` reader - sdr50 status"]
pub type SDR50_ST_R = crate::BitReader;
#[doc = "Field `SDR104_ST` reader - sdr104 status"]
pub type SDR104_ST_R = crate::BitReader;
#[doc = "Field `DDR50_ST` reader - ddr50 status"]
pub type DDR50_ST_R = crate::BitReader;
#[doc = "Field `TUNE_ST` reader - tune_st fsm status"]
pub type TUNE_ST_R = crate::FieldReader;
#[doc = "Field `SDIO_SWITCH_VOLT_ST` reader - sdio switch voltage status:0-3.3V, 1-1.8V."]
pub type SDIO_SWITCH_VOLT_ST_R = crate::BitReader;
#[doc = "Field `SDIO_SWITCH_END` reader - sdio switch voltage ldo ready"]
pub type SDIO_SWITCH_END_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - func0 config0 (addr: 0x20f0 ) status"]
    #[inline(always)]
    pub fn func0_config0(&self) -> FUNC0_CONFIG0_R {
        FUNC0_CONFIG0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - sdr25 status"]
    #[inline(always)]
    pub fn sdr25_st(&self) -> SDR25_ST_R {
        SDR25_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - sdr50 status"]
    #[inline(always)]
    pub fn sdr50_st(&self) -> SDR50_ST_R {
        SDR50_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - sdr104 status"]
    #[inline(always)]
    pub fn sdr104_st(&self) -> SDR104_ST_R {
        SDR104_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ddr50 status"]
    #[inline(always)]
    pub fn ddr50_st(&self) -> DDR50_ST_R {
        DDR50_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - tune_st fsm status"]
    #[inline(always)]
    pub fn tune_st(&self) -> TUNE_ST_R {
        TUNE_ST_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - sdio switch voltage status:0-3.3V, 1-1.8V."]
    #[inline(always)]
    pub fn sdio_switch_volt_st(&self) -> SDIO_SWITCH_VOLT_ST_R {
        SDIO_SWITCH_VOLT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - sdio switch voltage ldo ready"]
    #[inline(always)]
    pub fn sdio_switch_end(&self) -> SDIO_SWITCH_END_R {
        SDIO_SWITCH_END_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_STATUS")
            .field(
                "func0_config0",
                &format_args!("{}", self.func0_config0().bits()),
            )
            .field("sdr25_st", &format_args!("{}", self.sdr25_st().bit()))
            .field("sdr50_st", &format_args!("{}", self.sdr50_st().bit()))
            .field("sdr104_st", &format_args!("{}", self.sdr104_st().bit()))
            .field("ddr50_st", &format_args!("{}", self.ddr50_st().bit()))
            .field("tune_st", &format_args!("{}", self.tune_st().bits()))
            .field(
                "sdio_switch_volt_st",
                &format_args!("{}", self.sdio_switch_volt_st().bit()),
            )
            .field(
                "sdio_switch_end",
                &format_args!("{}", self.sdio_switch_end().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "func0 config0 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_status](index.html) module"]
pub struct CONF_STATUS_SPEC;
impl crate::RegisterSpec for CONF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_status::R](R) reader structure"]
impl crate::Readable for CONF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONF_STATUS to value 0"]
impl crate::Resettable for CONF_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
