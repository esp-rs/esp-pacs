#[doc = "Register `BIAS_CONF` reader"]
pub type R = crate::R<BIAS_CONF_SPEC>;
#[doc = "Register `BIAS_CONF` writer"]
pub type W = crate::W<BIAS_CONF_SPEC>;
#[doc = "Field `DG_VDD_DRV_B_SLP` reader - Need add desc"]
pub type DG_VDD_DRV_B_SLP_R = crate::FieldReader;
#[doc = "Field `DG_VDD_DRV_B_SLP` writer - Need add desc"]
pub type DG_VDD_DRV_B_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DG_VDD_DRV_B_SLP_EN` reader - Need add desc"]
pub type DG_VDD_DRV_B_SLP_EN_R = crate::BitReader;
#[doc = "Field `DG_VDD_DRV_B_SLP_EN` writer - Need add desc"]
pub type DG_VDD_DRV_B_SLP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_BUF_IDLE` reader - Need add desc"]
pub type BIAS_BUF_IDLE_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_IDLE` writer - Need add desc"]
pub type BIAS_BUF_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_BUF_WAKE` reader - Need add desc"]
pub type BIAS_BUF_WAKE_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_WAKE` writer - Need add desc"]
pub type BIAS_BUF_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_BUF_DEEP_SLP` reader - Need add desc"]
pub type BIAS_BUF_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_DEEP_SLP` writer - Need add desc"]
pub type BIAS_BUF_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_BUF_MONITOR` reader - Need add desc"]
pub type BIAS_BUF_MONITOR_R = crate::BitReader;
#[doc = "Field `BIAS_BUF_MONITOR` writer - Need add desc"]
pub type BIAS_BUF_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_CUR_DEEP_SLP` reader - xpd cur when rtc in sleep_state"]
pub type PD_CUR_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `PD_CUR_DEEP_SLP` writer - xpd cur when rtc in sleep_state"]
pub type PD_CUR_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_CUR_MONITOR` reader - xpd cur when rtc in monitor state"]
pub type PD_CUR_MONITOR_R = crate::BitReader;
#[doc = "Field `PD_CUR_MONITOR` writer - xpd cur when rtc in monitor state"]
pub type PD_CUR_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` reader - bias_sleep when rtc in sleep_state"]
pub type BIAS_SLEEP_DEEP_SLP_R = crate::BitReader;
#[doc = "Field `BIAS_SLEEP_DEEP_SLP` writer - bias_sleep when rtc in sleep_state"]
pub type BIAS_SLEEP_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIAS_SLEEP_MONITOR` reader - bias_sleep when rtc in monitor state"]
pub type BIAS_SLEEP_MONITOR_R = crate::BitReader;
#[doc = "Field `BIAS_SLEEP_MONITOR` writer - bias_sleep when rtc in monitor state"]
pub type BIAS_SLEEP_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_ATTEN_DEEP_SLP` reader - DBG_ATTEN when rtc in sleep state"]
pub type DBG_ATTEN_DEEP_SLP_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_DEEP_SLP` writer - DBG_ATTEN when rtc in sleep state"]
pub type DBG_ATTEN_DEEP_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DBG_ATTEN_MONITOR` reader - DBG_ATTEN when rtc in active state"]
pub type DBG_ATTEN_MONITOR_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_MONITOR` writer - DBG_ATTEN when rtc in active state"]
pub type DBG_ATTEN_MONITOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DBG_ATTEN_ACTIVE` reader - Need add desc"]
pub type DBG_ATTEN_ACTIVE_R = crate::FieldReader;
#[doc = "Field `DBG_ATTEN_ACTIVE` writer - Need add desc"]
pub type DBG_ATTEN_ACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
            .field("dg_vdd_drv_b_slp", &self.dg_vdd_drv_b_slp())
            .field("dg_vdd_drv_b_slp_en", &self.dg_vdd_drv_b_slp_en())
            .field("bias_buf_idle", &self.bias_buf_idle())
            .field("bias_buf_wake", &self.bias_buf_wake())
            .field("bias_buf_deep_slp", &self.bias_buf_deep_slp())
            .field("bias_buf_monitor", &self.bias_buf_monitor())
            .field("pd_cur_deep_slp", &self.pd_cur_deep_slp())
            .field("pd_cur_monitor", &self.pd_cur_monitor())
            .field("bias_sleep_deep_slp", &self.bias_sleep_deep_slp())
            .field("bias_sleep_monitor", &self.bias_sleep_monitor())
            .field("dbg_atten_deep_slp", &self.dbg_atten_deep_slp())
            .field("dbg_atten_monitor", &self.dbg_atten_monitor())
            .field("dbg_atten_active", &self.dbg_atten_active())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Need add desc"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp(&mut self) -> DG_VDD_DRV_B_SLP_W<BIAS_CONF_SPEC> {
        DG_VDD_DRV_B_SLP_W::new(self, 0)
    }
    #[doc = "Bit 8 - Need add desc"]
    #[inline(always)]
    pub fn dg_vdd_drv_b_slp_en(&mut self) -> DG_VDD_DRV_B_SLP_EN_W<BIAS_CONF_SPEC> {
        DG_VDD_DRV_B_SLP_EN_W::new(self, 8)
    }
    #[doc = "Bit 10 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W<BIAS_CONF_SPEC> {
        BIAS_BUF_IDLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W<BIAS_CONF_SPEC> {
        BIAS_BUF_WAKE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W<BIAS_CONF_SPEC> {
        BIAS_BUF_DEEP_SLP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Need add desc"]
    #[inline(always)]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W<BIAS_CONF_SPEC> {
        BIAS_BUF_MONITOR_W::new(self, 13)
    }
    #[doc = "Bit 14 - xpd cur when rtc in sleep_state"]
    #[inline(always)]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W<BIAS_CONF_SPEC> {
        PD_CUR_DEEP_SLP_W::new(self, 14)
    }
    #[doc = "Bit 15 - xpd cur when rtc in monitor state"]
    #[inline(always)]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W<BIAS_CONF_SPEC> {
        PD_CUR_MONITOR_W::new(self, 15)
    }
    #[doc = "Bit 16 - bias_sleep when rtc in sleep_state"]
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W<BIAS_CONF_SPEC> {
        BIAS_SLEEP_DEEP_SLP_W::new(self, 16)
    }
    #[doc = "Bit 17 - bias_sleep when rtc in monitor state"]
    #[inline(always)]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W<BIAS_CONF_SPEC> {
        BIAS_SLEEP_MONITOR_W::new(self, 17)
    }
    #[doc = "Bits 18:21 - DBG_ATTEN when rtc in sleep state"]
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_DEEP_SLP_W::new(self, 18)
    }
    #[doc = "Bits 22:25 - DBG_ATTEN when rtc in active state"]
    #[inline(always)]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_MONITOR_W::new(self, 22)
    }
    #[doc = "Bits 26:29 - Need add desc"]
    #[inline(always)]
    pub fn dbg_atten_active(&mut self) -> DBG_ATTEN_ACTIVE_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_ACTIVE_W::new(self, 26)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`bias_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bias_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bias_conf::R`](R) reader structure"]
impl crate::Readable for BIAS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bias_conf::W`](W) writer structure"]
impl crate::Writable for BIAS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BIAS_CONF to value 0x0001_0800"]
impl crate::Resettable for BIAS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_0800;
}
