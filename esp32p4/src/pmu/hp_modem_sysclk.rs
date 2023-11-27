#[doc = "Register `HP_MODEM_SYSCLK` writer"]
pub type W = crate::W<HP_MODEM_SYSCLK_SPEC>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_NO_DIV` writer - need_des"]
pub type HP_MODEM_DIG_SYS_CLK_NO_DIV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_ICG_SYS_CLOCK_EN` writer - need_des"]
pub type HP_MODEM_ICG_SYS_CLOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_SYS_CLK_SLP_SEL` writer - need_des"]
pub type HP_MODEM_SYS_CLK_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_ICG_SLP_SEL` writer - need_des"]
pub type HP_MODEM_ICG_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DIG_SYS_CLK_SEL` writer - need_des"]
pub type HP_MODEM_DIG_SYS_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_SYSCLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_sys_clk_no_div(
        &mut self,
    ) -> HP_MODEM_DIG_SYS_CLK_NO_DIV_W<HP_MODEM_SYSCLK_SPEC> {
        HP_MODEM_DIG_SYS_CLK_NO_DIV_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_icg_sys_clock_en(
        &mut self,
    ) -> HP_MODEM_ICG_SYS_CLOCK_EN_W<HP_MODEM_SYSCLK_SPEC> {
        HP_MODEM_ICG_SYS_CLOCK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_sys_clk_slp_sel(&mut self) -> HP_MODEM_SYS_CLK_SLP_SEL_W<HP_MODEM_SYSCLK_SPEC> {
        HP_MODEM_SYS_CLK_SLP_SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_icg_slp_sel(&mut self) -> HP_MODEM_ICG_SLP_SEL_W<HP_MODEM_SYSCLK_SPEC> {
        HP_MODEM_ICG_SLP_SEL_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_dig_sys_clk_sel(&mut self) -> HP_MODEM_DIG_SYS_CLK_SEL_W<HP_MODEM_SYSCLK_SPEC> {
        HP_MODEM_DIG_SYS_CLK_SEL_W::new(self, 30)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_sysclk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_SYSCLK_SPEC;
impl crate::RegisterSpec for HP_MODEM_SYSCLK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_sysclk::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_SYSCLK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_SYSCLK to value 0"]
impl crate::Resettable for HP_MODEM_SYSCLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
