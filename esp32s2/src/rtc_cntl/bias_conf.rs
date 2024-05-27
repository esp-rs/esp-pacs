///Register `BIAS_CONF` reader
pub type R = crate::R<BIAS_CONF_SPEC>;
///Register `BIAS_CONF` writer
pub type W = crate::W<BIAS_CONF_SPEC>;
///Field `BIAS_BUF_IDLE` reader - open bias buf when system in active
pub type BIAS_BUF_IDLE_R = crate::BitReader;
///Field `BIAS_BUF_IDLE` writer - open bias buf when system in active
pub type BIAS_BUF_IDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_BUF_WAKE` reader - open bias buf when rtc in wakeup
pub type BIAS_BUF_WAKE_R = crate::BitReader;
///Field `BIAS_BUF_WAKE` writer - open bias buf when rtc in wakeup
pub type BIAS_BUF_WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_BUF_DEEP_SLP` reader - open bias buf when rtc in deep sleep
pub type BIAS_BUF_DEEP_SLP_R = crate::BitReader;
///Field `BIAS_BUF_DEEP_SLP` writer - open bias buf when rtc in deep sleep
pub type BIAS_BUF_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_BUF_MONITOR` reader - open bias buf when rtc in monitor state
pub type BIAS_BUF_MONITOR_R = crate::BitReader;
///Field `BIAS_BUF_MONITOR` writer - open bias buf when rtc in monitor state
pub type BIAS_BUF_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_CUR_DEEP_SLP` reader - xpd cur when rtc in sleep_state
pub type PD_CUR_DEEP_SLP_R = crate::BitReader;
///Field `PD_CUR_DEEP_SLP` writer - xpd cur when rtc in sleep_state
pub type PD_CUR_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_CUR_MONITOR` reader - xpd cur when rtc in monitor state
pub type PD_CUR_MONITOR_R = crate::BitReader;
///Field `PD_CUR_MONITOR` writer - xpd cur when rtc in monitor state
pub type PD_CUR_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_SLEEP_DEEP_SLP` reader - bias_sleep when rtc in sleep_state
pub type BIAS_SLEEP_DEEP_SLP_R = crate::BitReader;
///Field `BIAS_SLEEP_DEEP_SLP` writer - bias_sleep when rtc in sleep_state
pub type BIAS_SLEEP_DEEP_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIAS_SLEEP_MONITOR` reader - bias_sleep when rtc in monitor state
pub type BIAS_SLEEP_MONITOR_R = crate::BitReader;
///Field `BIAS_SLEEP_MONITOR` writer - bias_sleep when rtc in monitor state
pub type BIAS_SLEEP_MONITOR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBG_ATTEN_DEEP_SLP` reader - DBG_ATTEN when rtc in sleep state
pub type DBG_ATTEN_DEEP_SLP_R = crate::FieldReader;
///Field `DBG_ATTEN_DEEP_SLP` writer - DBG_ATTEN when rtc in sleep state
pub type DBG_ATTEN_DEEP_SLP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `DBG_ATTEN_MONITOR` reader - DBG_ATTEN when rtc in monitor state
pub type DBG_ATTEN_MONITOR_R = crate::FieldReader;
///Field `DBG_ATTEN_MONITOR` writer - DBG_ATTEN when rtc in monitor state
pub type DBG_ATTEN_MONITOR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ENB_SCK_XTAL` reader - ENB_SCK_XTAL
pub type ENB_SCK_XTAL_R = crate::BitReader;
///Field `ENB_SCK_XTAL` writer - ENB_SCK_XTAL
pub type ENB_SCK_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INC_HEARTBEAT_REFRESH` reader - INC_HEARTBEAT_REFRESH
pub type INC_HEARTBEAT_REFRESH_R = crate::BitReader;
///Field `INC_HEARTBEAT_REFRESH` writer - INC_HEARTBEAT_REFRESH
pub type INC_HEARTBEAT_REFRESH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEC_HEARTBEAT_PERIOD` reader - DEC_HEARTBEAT_PERIOD
pub type DEC_HEARTBEAT_PERIOD_R = crate::BitReader;
///Field `DEC_HEARTBEAT_PERIOD` writer - DEC_HEARTBEAT_PERIOD
pub type DEC_HEARTBEAT_PERIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INC_HEARTBEAT_PERIOD` reader - INC_HEARTBEAT_PERIOD
pub type INC_HEARTBEAT_PERIOD_R = crate::BitReader;
///Field `INC_HEARTBEAT_PERIOD` writer - INC_HEARTBEAT_PERIOD
pub type INC_HEARTBEAT_PERIOD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEC_HEARTBEAT_WIDTH` reader - DEC_HEARTBEAT_WIDTH
pub type DEC_HEARTBEAT_WIDTH_R = crate::BitReader;
///Field `DEC_HEARTBEAT_WIDTH` writer - DEC_HEARTBEAT_WIDTH
pub type DEC_HEARTBEAT_WIDTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_BIAS_I2C` reader -
pub type RST_BIAS_I2C_R = crate::BitReader;
///Field `RST_BIAS_I2C` writer -
pub type RST_BIAS_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 10 - open bias buf when system in active
    #[inline(always)]
    pub fn bias_buf_idle(&self) -> BIAS_BUF_IDLE_R {
        BIAS_BUF_IDLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - open bias buf when rtc in wakeup
    #[inline(always)]
    pub fn bias_buf_wake(&self) -> BIAS_BUF_WAKE_R {
        BIAS_BUF_WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - open bias buf when rtc in deep sleep
    #[inline(always)]
    pub fn bias_buf_deep_slp(&self) -> BIAS_BUF_DEEP_SLP_R {
        BIAS_BUF_DEEP_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - open bias buf when rtc in monitor state
    #[inline(always)]
    pub fn bias_buf_monitor(&self) -> BIAS_BUF_MONITOR_R {
        BIAS_BUF_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - xpd cur when rtc in sleep_state
    #[inline(always)]
    pub fn pd_cur_deep_slp(&self) -> PD_CUR_DEEP_SLP_R {
        PD_CUR_DEEP_SLP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - xpd cur when rtc in monitor state
    #[inline(always)]
    pub fn pd_cur_monitor(&self) -> PD_CUR_MONITOR_R {
        PD_CUR_MONITOR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - bias_sleep when rtc in sleep_state
    #[inline(always)]
    pub fn bias_sleep_deep_slp(&self) -> BIAS_SLEEP_DEEP_SLP_R {
        BIAS_SLEEP_DEEP_SLP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - bias_sleep when rtc in monitor state
    #[inline(always)]
    pub fn bias_sleep_monitor(&self) -> BIAS_SLEEP_MONITOR_R {
        BIAS_SLEEP_MONITOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:21 - DBG_ATTEN when rtc in sleep state
    #[inline(always)]
    pub fn dbg_atten_deep_slp(&self) -> DBG_ATTEN_DEEP_SLP_R {
        DBG_ATTEN_DEEP_SLP_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:25 - DBG_ATTEN when rtc in monitor state
    #[inline(always)]
    pub fn dbg_atten_monitor(&self) -> DBG_ATTEN_MONITOR_R {
        DBG_ATTEN_MONITOR_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 26 - ENB_SCK_XTAL
    #[inline(always)]
    pub fn enb_sck_xtal(&self) -> ENB_SCK_XTAL_R {
        ENB_SCK_XTAL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - INC_HEARTBEAT_REFRESH
    #[inline(always)]
    pub fn inc_heartbeat_refresh(&self) -> INC_HEARTBEAT_REFRESH_R {
        INC_HEARTBEAT_REFRESH_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DEC_HEARTBEAT_PERIOD
    #[inline(always)]
    pub fn dec_heartbeat_period(&self) -> DEC_HEARTBEAT_PERIOD_R {
        DEC_HEARTBEAT_PERIOD_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - INC_HEARTBEAT_PERIOD
    #[inline(always)]
    pub fn inc_heartbeat_period(&self) -> INC_HEARTBEAT_PERIOD_R {
        INC_HEARTBEAT_PERIOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DEC_HEARTBEAT_WIDTH
    #[inline(always)]
    pub fn dec_heartbeat_width(&self) -> DEC_HEARTBEAT_WIDTH_R {
        DEC_HEARTBEAT_WIDTH_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn rst_bias_i2c(&self) -> RST_BIAS_I2C_R {
        RST_BIAS_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIAS_CONF")
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
            .field("enb_sck_xtal", &self.enb_sck_xtal())
            .field("inc_heartbeat_refresh", &self.inc_heartbeat_refresh())
            .field("dec_heartbeat_period", &self.dec_heartbeat_period())
            .field("inc_heartbeat_period", &self.inc_heartbeat_period())
            .field("dec_heartbeat_width", &self.dec_heartbeat_width())
            .field("rst_bias_i2c", &self.rst_bias_i2c())
            .finish()
    }
}
impl W {
    ///Bit 10 - open bias buf when system in active
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_idle(&mut self) -> BIAS_BUF_IDLE_W<BIAS_CONF_SPEC> {
        BIAS_BUF_IDLE_W::new(self, 10)
    }
    ///Bit 11 - open bias buf when rtc in wakeup
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_wake(&mut self) -> BIAS_BUF_WAKE_W<BIAS_CONF_SPEC> {
        BIAS_BUF_WAKE_W::new(self, 11)
    }
    ///Bit 12 - open bias buf when rtc in deep sleep
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_deep_slp(&mut self) -> BIAS_BUF_DEEP_SLP_W<BIAS_CONF_SPEC> {
        BIAS_BUF_DEEP_SLP_W::new(self, 12)
    }
    ///Bit 13 - open bias buf when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn bias_buf_monitor(&mut self) -> BIAS_BUF_MONITOR_W<BIAS_CONF_SPEC> {
        BIAS_BUF_MONITOR_W::new(self, 13)
    }
    ///Bit 14 - xpd cur when rtc in sleep_state
    #[inline(always)]
    #[must_use]
    pub fn pd_cur_deep_slp(&mut self) -> PD_CUR_DEEP_SLP_W<BIAS_CONF_SPEC> {
        PD_CUR_DEEP_SLP_W::new(self, 14)
    }
    ///Bit 15 - xpd cur when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn pd_cur_monitor(&mut self) -> PD_CUR_MONITOR_W<BIAS_CONF_SPEC> {
        PD_CUR_MONITOR_W::new(self, 15)
    }
    ///Bit 16 - bias_sleep when rtc in sleep_state
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_deep_slp(&mut self) -> BIAS_SLEEP_DEEP_SLP_W<BIAS_CONF_SPEC> {
        BIAS_SLEEP_DEEP_SLP_W::new(self, 16)
    }
    ///Bit 17 - bias_sleep when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn bias_sleep_monitor(&mut self) -> BIAS_SLEEP_MONITOR_W<BIAS_CONF_SPEC> {
        BIAS_SLEEP_MONITOR_W::new(self, 17)
    }
    ///Bits 18:21 - DBG_ATTEN when rtc in sleep state
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_deep_slp(&mut self) -> DBG_ATTEN_DEEP_SLP_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_DEEP_SLP_W::new(self, 18)
    }
    ///Bits 22:25 - DBG_ATTEN when rtc in monitor state
    #[inline(always)]
    #[must_use]
    pub fn dbg_atten_monitor(&mut self) -> DBG_ATTEN_MONITOR_W<BIAS_CONF_SPEC> {
        DBG_ATTEN_MONITOR_W::new(self, 22)
    }
    ///Bit 26 - ENB_SCK_XTAL
    #[inline(always)]
    #[must_use]
    pub fn enb_sck_xtal(&mut self) -> ENB_SCK_XTAL_W<BIAS_CONF_SPEC> {
        ENB_SCK_XTAL_W::new(self, 26)
    }
    ///Bit 27 - INC_HEARTBEAT_REFRESH
    #[inline(always)]
    #[must_use]
    pub fn inc_heartbeat_refresh(&mut self) -> INC_HEARTBEAT_REFRESH_W<BIAS_CONF_SPEC> {
        INC_HEARTBEAT_REFRESH_W::new(self, 27)
    }
    ///Bit 28 - DEC_HEARTBEAT_PERIOD
    #[inline(always)]
    #[must_use]
    pub fn dec_heartbeat_period(&mut self) -> DEC_HEARTBEAT_PERIOD_W<BIAS_CONF_SPEC> {
        DEC_HEARTBEAT_PERIOD_W::new(self, 28)
    }
    ///Bit 29 - INC_HEARTBEAT_PERIOD
    #[inline(always)]
    #[must_use]
    pub fn inc_heartbeat_period(&mut self) -> INC_HEARTBEAT_PERIOD_W<BIAS_CONF_SPEC> {
        INC_HEARTBEAT_PERIOD_W::new(self, 29)
    }
    ///Bit 30 - DEC_HEARTBEAT_WIDTH
    #[inline(always)]
    #[must_use]
    pub fn dec_heartbeat_width(&mut self) -> DEC_HEARTBEAT_WIDTH_W<BIAS_CONF_SPEC> {
        DEC_HEARTBEAT_WIDTH_W::new(self, 30)
    }
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn rst_bias_i2c(&mut self) -> RST_BIAS_I2C_W<BIAS_CONF_SPEC> {
        RST_BIAS_I2C_W::new(self, 31)
    }
}
/**configure power register

You can [`read`](crate::generic::Reg::read) this register and get [`bias_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bias_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BIAS_CONF_SPEC;
impl crate::RegisterSpec for BIAS_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bias_conf::R`](R) reader structure
impl crate::Readable for BIAS_CONF_SPEC {}
///`write(|w| ..)` method takes [`bias_conf::W`](W) writer structure
impl crate::Writable for BIAS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BIAS_CONF to value 0x0001_0800
impl crate::Resettable for BIAS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_0800;
}
