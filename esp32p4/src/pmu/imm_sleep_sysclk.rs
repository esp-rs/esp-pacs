///Register `IMM_SLEEP_SYSCLK` writer
pub type W = crate::W<IMM_SLEEP_SYSCLK_SPEC>;
///Field `UPDATE_DIG_ICG_SWITCH` writer - need_des
pub type UPDATE_DIG_ICG_SWITCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE_LOW_ICG_SLP_SEL` writer - need_des
pub type TIE_LOW_ICG_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIE_HIGH_ICG_SLP_SEL` writer - need_des
pub type TIE_HIGH_ICG_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UPDATE_DIG_SYS_CLK_SEL` writer - need_des
pub type UPDATE_DIG_SYS_CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IMM_SLEEP_SYSCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 28 - need_des
    #[inline(always)]
    #[must_use]
    pub fn update_dig_icg_switch(&mut self) -> UPDATE_DIG_ICG_SWITCH_W<IMM_SLEEP_SYSCLK_SPEC> {
        UPDATE_DIG_ICG_SWITCH_W::new(self, 28)
    }
    ///Bit 29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tie_low_icg_slp_sel(&mut self) -> TIE_LOW_ICG_SLP_SEL_W<IMM_SLEEP_SYSCLK_SPEC> {
        TIE_LOW_ICG_SLP_SEL_W::new(self, 29)
    }
    ///Bit 30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn tie_high_icg_slp_sel(&mut self) -> TIE_HIGH_ICG_SLP_SEL_W<IMM_SLEEP_SYSCLK_SPEC> {
        TIE_HIGH_ICG_SLP_SEL_W::new(self, 30)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn update_dig_sys_clk_sel(&mut self) -> UPDATE_DIG_SYS_CLK_SEL_W<IMM_SLEEP_SYSCLK_SPEC> {
        UPDATE_DIG_SYS_CLK_SEL_W::new(self, 31)
    }
}
/**need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_sleep_sysclk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IMM_SLEEP_SYSCLK_SPEC;
impl crate::RegisterSpec for IMM_SLEEP_SYSCLK_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`imm_sleep_sysclk::W`](W) writer structure
impl crate::Writable for IMM_SLEEP_SYSCLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IMM_SLEEP_SYSCLK to value 0
impl crate::Resettable for IMM_SLEEP_SYSCLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
