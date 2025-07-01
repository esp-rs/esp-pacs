#[doc = "Register `HP_MODEM_HP_SYS_CNTL` writer"]
pub type W = crate::W<HP_MODEM_HP_SYS_CNTL_SPEC>;
#[doc = "Field `HP_MODEM_HP_POWER_DET_BYPASS` writer - need_des"]
pub type HP_MODEM_HP_POWER_DET_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_UART_WAKEUP_EN` writer - need_des"]
pub type HP_MODEM_UART_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_LP_PAD_HOLD_ALL` writer - need_des"]
pub type HP_MODEM_LP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_HP_PAD_HOLD_ALL` writer - need_des"]
pub type HP_MODEM_HP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DIG_PAD_SLP_SEL` writer - need_des"]
pub type HP_MODEM_DIG_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DIG_PAUSE_WDT` writer - need_des"]
pub type HP_MODEM_DIG_PAUSE_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DIG_CPU_STALL` writer - need_des"]
pub type HP_MODEM_DIG_CPU_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_HP_SYS_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_power_det_bypass(
        &mut self,
    ) -> HP_MODEM_HP_POWER_DET_BYPASS_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_HP_POWER_DET_BYPASS_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_modem_uart_wakeup_en(
        &mut self,
    ) -> HP_MODEM_UART_WAKEUP_EN_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_UART_WAKEUP_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_modem_lp_pad_hold_all(
        &mut self,
    ) -> HP_MODEM_LP_PAD_HOLD_ALL_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_LP_PAD_HOLD_ALL_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_pad_hold_all(
        &mut self,
    ) -> HP_MODEM_HP_PAD_HOLD_ALL_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_HP_PAD_HOLD_ALL_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_pad_slp_sel(
        &mut self,
    ) -> HP_MODEM_DIG_PAD_SLP_SEL_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_DIG_PAD_SLP_SEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_pause_wdt(
        &mut self,
    ) -> HP_MODEM_DIG_PAUSE_WDT_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_DIG_PAUSE_WDT_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dig_cpu_stall(
        &mut self,
    ) -> HP_MODEM_DIG_CPU_STALL_W<HP_MODEM_HP_SYS_CNTL_SPEC> {
        HP_MODEM_DIG_CPU_STALL_W::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_sys_cntl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_HP_SYS_CNTL_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_SYS_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_hp_sys_cntl::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_HP_SYS_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_HP_SYS_CNTL to value 0"]
impl crate::Resettable for HP_MODEM_HP_SYS_CNTL_SPEC {}
