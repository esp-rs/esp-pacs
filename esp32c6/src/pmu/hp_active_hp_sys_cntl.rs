#[doc = "Register `HP_ACTIVE_HP_SYS_CNTL` reader"]
pub type R = crate::R<HP_ACTIVE_HP_SYS_CNTL_SPEC>;
#[doc = "Register `HP_ACTIVE_HP_SYS_CNTL` writer"]
pub type W = crate::W<HP_ACTIVE_HP_SYS_CNTL_SPEC>;
#[doc = "Field `HP_ACTIVE_UART_WAKEUP_EN` reader - need_des"]
pub type HP_ACTIVE_UART_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_UART_WAKEUP_EN` writer - need_des"]
pub type HP_ACTIVE_UART_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_LP_PAD_HOLD_ALL` reader - need_des"]
pub type HP_ACTIVE_LP_PAD_HOLD_ALL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_LP_PAD_HOLD_ALL` writer - need_des"]
pub type HP_ACTIVE_LP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_HP_PAD_HOLD_ALL` reader - need_des"]
pub type HP_ACTIVE_HP_PAD_HOLD_ALL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_PAD_HOLD_ALL` writer - need_des"]
pub type HP_ACTIVE_HP_PAD_HOLD_ALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_DIG_PAD_SLP_SEL` reader - need_des"]
pub type HP_ACTIVE_DIG_PAD_SLP_SEL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_DIG_PAD_SLP_SEL` writer - need_des"]
pub type HP_ACTIVE_DIG_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_DIG_PAUSE_WDT` reader - need_des"]
pub type HP_ACTIVE_DIG_PAUSE_WDT_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_DIG_PAUSE_WDT` writer - need_des"]
pub type HP_ACTIVE_DIG_PAUSE_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_DIG_CPU_STALL` reader - need_des"]
pub type HP_ACTIVE_DIG_CPU_STALL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_DIG_CPU_STALL` writer - need_des"]
pub type HP_ACTIVE_DIG_CPU_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_active_uart_wakeup_en(&self) -> HP_ACTIVE_UART_WAKEUP_EN_R {
        HP_ACTIVE_UART_WAKEUP_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_active_lp_pad_hold_all(&self) -> HP_ACTIVE_LP_PAD_HOLD_ALL_R {
        HP_ACTIVE_LP_PAD_HOLD_ALL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_active_hp_pad_hold_all(&self) -> HP_ACTIVE_HP_PAD_HOLD_ALL_R {
        HP_ACTIVE_HP_PAD_HOLD_ALL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_pad_slp_sel(&self) -> HP_ACTIVE_DIG_PAD_SLP_SEL_R {
        HP_ACTIVE_DIG_PAD_SLP_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_pause_wdt(&self) -> HP_ACTIVE_DIG_PAUSE_WDT_R {
        HP_ACTIVE_DIG_PAUSE_WDT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_active_dig_cpu_stall(&self) -> HP_ACTIVE_DIG_CPU_STALL_R {
        HP_ACTIVE_DIG_CPU_STALL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_HP_SYS_CNTL")
            .field(
                "hp_active_uart_wakeup_en",
                &format_args!("{}", self.hp_active_uart_wakeup_en().bit()),
            )
            .field(
                "hp_active_lp_pad_hold_all",
                &format_args!("{}", self.hp_active_lp_pad_hold_all().bit()),
            )
            .field(
                "hp_active_hp_pad_hold_all",
                &format_args!("{}", self.hp_active_hp_pad_hold_all().bit()),
            )
            .field(
                "hp_active_dig_pad_slp_sel",
                &format_args!("{}", self.hp_active_dig_pad_slp_sel().bit()),
            )
            .field(
                "hp_active_dig_pause_wdt",
                &format_args!("{}", self.hp_active_dig_pause_wdt().bit()),
            )
            .field(
                "hp_active_dig_cpu_stall",
                &format_args!("{}", self.hp_active_dig_cpu_stall().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_uart_wakeup_en(
        &mut self,
    ) -> HP_ACTIVE_UART_WAKEUP_EN_W<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
        HP_ACTIVE_UART_WAKEUP_EN_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_lp_pad_hold_all(
        &mut self,
    ) -> HP_ACTIVE_LP_PAD_HOLD_ALL_W<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
        HP_ACTIVE_LP_PAD_HOLD_ALL_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_hp_pad_hold_all(
        &mut self,
    ) -> HP_ACTIVE_HP_PAD_HOLD_ALL_W<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
        HP_ACTIVE_HP_PAD_HOLD_ALL_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_pad_slp_sel(
        &mut self,
    ) -> HP_ACTIVE_DIG_PAD_SLP_SEL_W<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
        HP_ACTIVE_DIG_PAD_SLP_SEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_pause_wdt(
        &mut self,
    ) -> HP_ACTIVE_DIG_PAUSE_WDT_W<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
        HP_ACTIVE_DIG_PAUSE_WDT_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_dig_cpu_stall(
        &mut self,
    ) -> HP_ACTIVE_DIG_CPU_STALL_W<HP_ACTIVE_HP_SYS_CNTL_SPEC> {
        HP_ACTIVE_DIG_CPU_STALL_W::new(self, 29)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_active_hp_sys_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_hp_sys_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_HP_SYS_CNTL_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_hp_sys_cntl::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_SYS_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_hp_sys_cntl::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_SYS_CNTL to value 0"]
impl crate::Resettable for HP_ACTIVE_HP_SYS_CNTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
