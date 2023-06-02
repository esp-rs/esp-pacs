#[doc = "Register `BIAS_CONF` reader"]
pub struct R(crate::R<BIAS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIAS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIAS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIAS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIAS_CONF` writer"]
pub struct W(crate::W<BIAS_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIAS_CONF_SPEC>;
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
impl From<crate::W<BIAS_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIAS_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS_BUF_IDLE` reader - open bias buf when system in active"]
pub type BIAS_BUF_IDLE_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_IDLE` writer - open bias buf when system in active"]
pub type BIAS_BUF_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_WAKE` reader - open bias buf when rtc in wakeup"]
pub type BIAS_BUF_WAKE_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_WAKE` writer - open bias buf when rtc in wakeup"]
pub type BIAS_BUF_WAKE_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_DEEP_SLP` reader - open bias buf when rtc in deep sleep"]
pub type BIAS_BUF_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_DEEP_SLP` writer - open bias buf when rtc in deep sleep"]
pub type BIAS_BUF_DEEP_SLP_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_MONITOR` reader - open bias buf when rtc in monitor state"]
pub type BIAS_BUF_MONITOR_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_MONITOR` writer - open bias buf when rtc in monitor state"]
pub type BIAS_BUF_MONITOR_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `PD_CUR_DEEP_SLP` reader - xpd cur when rtc in sleep_state"]
pub type PD_CUR_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `PD_CUR_DEEP_SLP` writer - xpd cur when rtc in sleep_state"]
pub type PD_CUR_DEEP_SLP_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `PD_CUR_MONITOR` reader - xpd cur when rtc in monitor state"]
pub type PD_CUR_MONITOR_R = crate::BitReader;
#[doc = "Field `PD_CUR_MONITOR` writer - xpd cur when rtc in monitor state"]
pub type PD_CUR_MONITOR_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` reader - bias_sleep when rtc in sleep_state"]
pub type BIAS_SLEEP_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` writer - bias_sleep when rtc in sleep_state"]
pub type BIAS_SLEEP_DEEP_SLP_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_SLEEP_MONITOR` reader - bias_sleep when rtc in monitor state"]
pub type BIAS_SLEEP_MONITOR_R = crate::BitReader;
#[doc = "Field `BIAS_SLEEP_MONITOR` writer - bias_sleep when rtc in monitor state"]
pub type BIAS_SLEEP_MONITOR_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `DBG_ATTEN_DEEP_SLP` reader - DBG_ATTEN when rtc in sleep state"]
pub type DBG_ATTEN_DEEP_SLP_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_DEEP_SLP` writer - DBG_ATTEN when rtc in sleep state"]
pub type DBG_ATTEN_DEEP_SLP_W<'a, const O: u8> = crate::FieldWriter<'a, BIAS_CONF_SPEC, 4, O>;
#[doc = "Field `DBG_ATTEN_MONITOR` reader - DBG_ATTEN when rtc in monitor state"]
pub type DBG_ATTEN_MONITOR_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_MONITOR` writer - DBG_ATTEN when rtc in monitor state"]
pub type DBG_ATTEN_MONITOR_W<'a, const O: u8> = crate::FieldWriter<'a, BIAS_CONF_SPEC, 4, O>;
#[doc = "Field `ENB_SCK_XTAL` reader - ENB_SCK_XTAL"]
pub type ENB_SCK_XTAL_R = crate::BitReader;
#[doc = "Field `ENB_SCK_XTAL` writer - ENB_SCK_XTAL"]
pub type ENB_SCK_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `INC_HEARTBEAT_REFRESH` reader - INC_HEARTBEAT_REFRESH"]
pub type INC_HEARTBEAT_REFRESH_R = crate::BitReader;
#[doc = "Field `INC_HEARTBEAT_REFRESH` writer - INC_HEARTBEAT_REFRESH"]
pub type INC_HEARTBEAT_REFRESH_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `DEC_HEARTBEAT_PERIOD` reader - DEC_HEARTBEAT_PERIOD"]
pub type DEC_HEARTBEAT_PERIOD_R = crate::BitReader;
#[doc = "Field `DEC_HEARTBEAT_PERIOD` writer - DEC_HEARTBEAT_PERIOD"]
pub type DEC_HEARTBEAT_PERIOD_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `INC_HEARTBEAT_PERIOD` reader - INC_HEARTBEAT_PERIOD"]
pub type INC_HEARTBEAT_PERIOD_R = crate::BitReader;
#[doc = "Field `INC_HEARTBEAT_PERIOD` writer - INC_HEARTBEAT_PERIOD"]
pub type INC_HEARTBEAT_PERIOD_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `DEC_HEARTBEAT_WIDTH` reader - DEC_HEARTBEAT_WIDTH"]
pub type DEC_HEARTBEAT_WIDTH_R = crate::BitReader;
#[doc = "Field `DEC_HEARTBEAT_WIDTH` writer - DEC_HEARTBEAT_WIDTH"]
pub type DEC_HEARTBEAT_WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `RST_BIAS_I2C` reader - "]
pub type RST_BIAS_I2C_R = crate::BitReader;
#[doc = "Field `RST_BIAS_I2C` writer - "]
pub type RST_BIAS_I2C_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 10 - open bias buf when system in active"]
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - open bias buf when rtc in wakeup"]
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - open bias buf when rtc in deep sleep"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - open bias buf when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_buf_monitor(&self) -> BIAS_BUF_MONITOR_R {
        BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - xpd cur when rtc in sleep_state"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&self) -> PD_CUR_DEEP_SLP_R {
        PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - xpd cur when rtc in monitor state"]
    #[inline(always)]
    pub fn pd_cur_monitor(&self) -> PD_CUR_MONITOR_R {
        PD_CUR_MONITOR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - bias_sleep when rtc in sleep_state"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&self) -> BIAS_SLEEP_DEEP_SLP_R {
        BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - bias_sleep when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&self) -> BIAS_SLEEP_MONITOR_R {
        BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - DBG_ATTEN when rtc in sleep state"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&self) -> DBG_ATTEN_DEEP_SLP_R {
        DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in monitor state"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    pub fn enb_sck_xtal(&self) -> ENB_SCK_XTAL_R {
        ENB_SCK_XTAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&self) -> INC_HEARTBEAT_REFRESH_R {
        INC_HEARTBEAT_REFRESH_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn dec_heartbeat_period(&self) -> DEC_HEARTBEAT_PERIOD_R {
        DEC_HEARTBEAT_PERIOD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    pub fn inc_heartbeat_period(&self) -> INC_HEARTBEAT_PERIOD_R {
        INC_HEARTBEAT_PERIOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    pub fn dec_heartbeat_width(&self) -> DEC_HEARTBEAT_WIDTH_R {
        DEC_HEARTBEAT_WIDTH_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rst_bias_i2c(&self) -> RST_BIAS_I2C_R {
        RST_BIAS_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIAS_CONF")
            .field(
                "bias_buf_idle",
                &format_args!("{}", self.bias_buf_idle().bit()),
            )
            .field(
                "bias_buf_wake",
                &format_args!("{}", self.bias_buf_wake().bit()),
            )
            .field(
                "bias_buf_deep_slp",
                &format_args!("{}", self.bias_buf_deep_slp().bit()),
            )
            .field(
                "bias_buf_monitor",
                &format_args!("{}", self.bias_buf_monitor().bit()),
            )
            .field(
                "pd_cur_deep_slp",
                &format_args!("{}", self.pd_cur_deep_slp().bit()),
            )
            .field(
                "pd_cur_monitor",
                &format_args!("{}", self.pd_cur_monitor().bit()),
            )
            .field(
                "bias_sleep_deep_slp",
                &format_args!("{}", self.bias_sleep_deep_slp().bit()),
            )
            .field(
                "bias_sleep_monitor",
                &format_args!("{}", self.bias_sleep_monitor().bit()),
            )
            .field(
                "dbg_atten_deep_slp",
                &format_args!("{}", self.dbg_atten_deep_slp().bits()),
            )
            .field(
                "dbg_atten_monitor",
                &format_args!("{}", self.dbg_atten_monitor().bits()),
            )
            .field(
                "enb_sck_xtal",
                &format_args!("{}", self.enb_sck_xtal().bit()),
            )
            .field(
                "inc_heartbeat_refresh",
                &format_args!("{}", self.inc_heartbeat_refresh().bit()),
            )
            .field(
                "dec_heartbeat_period",
                &format_args!("{}", self.dec_heartbeat_period().bit()),
            )
            .field(
                "inc_heartbeat_period",
                &format_args!("{}", self.inc_heartbeat_period().bit()),
            )
            .field(
                "dec_heartbeat_width",
                &format_args!("{}", self.dec_heartbeat_width().bit()),
            )
            .field(
                "rst_bias_i2c",
                &format_args!("{}", self.rst_bias_i2c().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BIAS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 10 - open bias buf when system in active"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W<10> {
        BIAS_BUF_IDLE_W::new(self)
    }
    #[doc = "Bit 11 - open bias buf when rtc in wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W<11> {
        BIAS_BUF_WAKE_W::new(self)
    }
    #[doc = "Bit 12 - open bias buf when rtc in deep sleep"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W<12> {
        BIAS_BUF_DEEP_SLP_W::new(self)
    }
    #[doc = "Bit 13 - open bias buf when rtc in monitor state"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W<13> {
        BIAS_BUF_MONITOR_W::new(self)
    }
    #[doc = "Bit 14 - xpd cur when rtc in sleep_state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W<14> {
        PD_CUR_DEEP_SLP_W::new(self)
    }
    #[doc = "Bit 15 - xpd cur when rtc in monitor state"]
    #[inline(always)]
    #[must_use]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W<15> {
        PD_CUR_MONITOR_W::new(self)
    }
    #[doc = "Bit 16 - bias_sleep when rtc in sleep_state"]
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W<16> {
        BIAS_SLEEP_DEEP_SLP_W::new(self)
    }
    #[doc = "Bit 17 - bias_sleep when rtc in monitor state"]
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W<17> {
        BIAS_SLEEP_MONITOR_W::new(self)
    }
    #[doc = "Bits 18:21 - DBG_ATTEN when rtc in sleep state"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W<18> {
        DBG_ATTEN_DEEP_SLP_W::new(self)
    }
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in monitor state"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W<22> {
        DBG_ATTEN_MONITOR_W::new(self)
    }
    #[doc = "Bit 26 - ENB_SCK_XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn enb_sck_xtal(&mut self) -> ENB_SCK_XTAL_W<26> {
        ENB_SCK_XTAL_W::new(self)
    }
    #[doc = "Bit 27 - INC_HEARTBEAT_REFRESH"]
    #[inline(always)]
    #[must_use]
    pub fn inc_heartbeat_refresh(&mut self) -> INC_HEARTBEAT_REFRESH_W<27> {
        INC_HEARTBEAT_REFRESH_W::new(self)
    }
    #[doc = "Bit 28 - DEC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn dec_heartbeat_period(&mut self) -> DEC_HEARTBEAT_PERIOD_W<28> {
        DEC_HEARTBEAT_PERIOD_W::new(self)
    }
    #[doc = "Bit 29 - INC_HEARTBEAT_PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn inc_heartbeat_period(&mut self) -> INC_HEARTBEAT_PERIOD_W<29> {
        INC_HEARTBEAT_PERIOD_W::new(self)
    }
    #[doc = "Bit 30 - DEC_HEARTBEAT_WIDTH"]
    #[inline(always)]
    #[must_use]
    pub fn dec_heartbeat_width(&mut self) -> DEC_HEARTBEAT_WIDTH_W<30> {
        DEC_HEARTBEAT_WIDTH_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rst_bias_i2c(&mut self) -> RST_BIAS_I2C_W<31> {
        RST_BIAS_I2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure power register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bias_conf](index.html) module"]
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bias_conf::R](R) reader structure"]
impl crate::Readable for BIAS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bias_conf::W](W) writer structure"]
impl crate::Writable for BIAS_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIAS_CONF to value 0x0001_0800"]
impl crate::Resettable for BIAS_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0800;
}
