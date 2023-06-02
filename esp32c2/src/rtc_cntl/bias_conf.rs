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
#[doc = "Field `DG_VDD_DRV_B_SLP` reader - Need add desc"]
pub type DG_VDD_DRV_B_SLP_R = crate::FieldReader;
#[doc = "Field `DG_VDD_DRV_B_SLP` writer - Need add desc"]
pub type DG_VDD_DRV_B_SLP_W<'a, const O: u8> = crate::FieldWriter<'a, BIAS_CONF_SPEC, 8, O>;
#[doc = "Field `DG_VDD_DRV_B_SLP_EN` reader - Need add desc"]
pub type DG_VDD_DRV_B_SLP_EN_R = crate::BitReader;
#[doc = "Field `DG_VDD_DRV_B_SLP_EN` writer - Need add desc"]
pub type DG_VDD_DRV_B_SLP_EN_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_IDLE` reader - Need add desc"]
pub type BIAS_BUF_IDLE_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_IDLE` writer - Need add desc"]
pub type BIAS_BUF_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_WAKE` reader - Need add desc"]
pub type BIAS_BUF_WAKE_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_WAKE` writer - Need add desc"]
pub type BIAS_BUF_WAKE_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_DEEP_SLP` reader - Need add desc"]
pub type BIAS_BUF_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_DEEP_SLP` writer - Need add desc"]
pub type BIAS_BUF_DEEP_SLP_W<'a, const O: u8> = crate::BitWriter<'a, BIAS_CONF_SPEC, O>;
#[doc = "Field `BIAS_BUF_MONITOR` reader - Need add desc"]
pub type BIAS_BUF_MONITOR_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_MONITOR` writer - Need add desc"]
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
#[doc = "Field `DBG_ATTEN_MONITOR` reader - DBG_ATTEN when rtc in active state"]
pub type DBG_ATTEN_MONITOR_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_MONITOR` writer - DBG_ATTEN when rtc in active state"]
pub type DBG_ATTEN_MONITOR_W<'a, const O: u8> = crate::FieldWriter<'a, BIAS_CONF_SPEC, 4, O>;
#[doc = "Field `DBG_ATTEN_ACTIVE` reader - Need add desc"]
pub type DBG_ATTEN_ACTIVE_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_ACTIVE` writer - Need add desc"]
pub type DBG_ATTEN_ACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, BIAS_CONF_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Need add desc"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&self) -> DG_VDD_DRV_B_SLP_R {
        DG_VDD_DRV_B_SLP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Need add desc"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp_en(&self) -> DG_VDD_DRV_B_SLP_EN_R {
        DG_VDD_DRV_B_SLP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Need add desc"]
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
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in active state"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - Need add desc"]
    #[inline(always)]
    pub fn dbg_atten_active(&self) -> DBG_ATTEN_ACTIVE_R {
        DBG_ATTEN_ACTIVE_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIAS_CONF")
            .field(
                "dg_vdd_drv_b_slp",
                &format_args!("{}", self.dg_vdd_drv_b_slp().bits()),
            )
            .field(
                "dg_vdd_drv_b_slp_en",
                &format_args!("{}", self.dg_vdd_drv_b_slp_en().bit()),
            )
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
                "dbg_atten_active",
                &format_args!("{}", self.dbg_atten_active().bits()),
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
    #[doc = "Bits 0:7 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W<0> {
        DG_VDD_DRV_B_SLP_W::new(self)
    }
    #[doc = "Bit 8 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dg_vdd_drv_b_slp_en(&mut self) -> DG_VDD_DRV_B_SLP_EN_W<8> {
        DG_VDD_DRV_B_SLP_EN_W::new(self)
    }
    #[doc = "Bit 10 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W<10> {
        BIAS_BUF_IDLE_W::new(self)
    }
    #[doc = "Bit 11 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W<11> {
        BIAS_BUF_WAKE_W::new(self)
    }
    #[doc = "Bit 12 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W<12> {
        BIAS_BUF_DEEP_SLP_W::new(self)
    }
    #[doc = "Bit 13 - Need add desc"]
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
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in active state"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W<22> {
        DBG_ATTEN_MONITOR_W::new(self)
    }
    #[doc = "Bits 26:29 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_active(&mut self) -> DBG_ATTEN_ACTIVE_W<26> {
        DBG_ATTEN_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bias_conf](index.html) module"]
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
