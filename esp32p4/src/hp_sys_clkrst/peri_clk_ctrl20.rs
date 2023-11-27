#[doc = "Register `PERI_CLK_CTRL20` reader"]
pub type R = crate::R<PERI_CLK_CTRL20_SPEC>;
#[doc = "Register `PERI_CLK_CTRL20` writer"]
pub type W = crate::W<PERI_CLK_CTRL20_SPEC>;
#[doc = "Field `REG_MCPWM0_CLK_SRC_SEL` reader - Reserved"]
pub type REG_MCPWM0_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_MCPWM0_CLK_SRC_SEL` writer - Reserved"]
pub type REG_MCPWM0_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_MCPWM0_CLK_EN` reader - Reserved"]
pub type REG_MCPWM0_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_MCPWM0_CLK_EN` writer - Reserved"]
pub type REG_MCPWM0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MCPWM0_CLK_DIV_NUM` reader - Reserved"]
pub type REG_MCPWM0_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_MCPWM0_CLK_DIV_NUM` writer - Reserved"]
pub type REG_MCPWM0_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_MCPWM1_CLK_SRC_SEL` reader - Reserved"]
pub type REG_MCPWM1_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_MCPWM1_CLK_SRC_SEL` writer - Reserved"]
pub type REG_MCPWM1_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_MCPWM1_CLK_EN` reader - Reserved"]
pub type REG_MCPWM1_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_MCPWM1_CLK_EN` writer - Reserved"]
pub type REG_MCPWM1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MCPWM1_CLK_DIV_NUM` reader - Reserved"]
pub type REG_MCPWM1_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_MCPWM1_CLK_DIV_NUM` writer - Reserved"]
pub type REG_MCPWM1_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_TIMERGRP0_T0_SRC_SEL` reader - Reserved"]
pub type REG_TIMERGRP0_T0_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_TIMERGRP0_T0_SRC_SEL` writer - Reserved"]
pub type REG_TIMERGRP0_T0_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TIMERGRP0_T0_CLK_EN` reader - Reserved"]
pub type REG_TIMERGRP0_T0_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TIMERGRP0_T0_CLK_EN` writer - Reserved"]
pub type REG_TIMERGRP0_T0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMERGRP0_T1_SRC_SEL` reader - Reserved"]
pub type REG_TIMERGRP0_T1_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_TIMERGRP0_T1_SRC_SEL` writer - Reserved"]
pub type REG_TIMERGRP0_T1_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TIMERGRP0_T1_CLK_EN` reader - Reserved"]
pub type REG_TIMERGRP0_T1_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TIMERGRP0_T1_CLK_EN` writer - Reserved"]
pub type REG_TIMERGRP0_T1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMERGRP0_WDT_SRC_SEL` reader - Reserved"]
pub type REG_TIMERGRP0_WDT_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_TIMERGRP0_WDT_SRC_SEL` writer - Reserved"]
pub type REG_TIMERGRP0_WDT_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_TIMERGRP0_WDT_CLK_EN` reader - Reserved"]
pub type REG_TIMERGRP0_WDT_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TIMERGRP0_WDT_CLK_EN` writer - Reserved"]
pub type REG_TIMERGRP0_WDT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_EN` reader - Reserved"]
pub type REG_TIMERGRP0_TGRT_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_EN` writer - Reserved"]
pub type REG_TIMERGRP0_TGRT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_mcpwm0_clk_src_sel(&self) -> REG_MCPWM0_CLK_SRC_SEL_R {
        REG_MCPWM0_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_mcpwm0_clk_en(&self) -> REG_MCPWM0_CLK_EN_R {
        REG_MCPWM0_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - Reserved"]
    #[inline(always)]
    pub fn reg_mcpwm0_clk_div_num(&self) -> REG_MCPWM0_CLK_DIV_NUM_R {
        REG_MCPWM0_CLK_DIV_NUM_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    pub fn reg_mcpwm1_clk_src_sel(&self) -> REG_MCPWM1_CLK_SRC_SEL_R {
        REG_MCPWM1_CLK_SRC_SEL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reg_mcpwm1_clk_en(&self) -> REG_MCPWM1_CLK_EN_R {
        REG_MCPWM1_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    pub fn reg_mcpwm1_clk_div_num(&self) -> REG_MCPWM1_CLK_DIV_NUM_R {
        REG_MCPWM1_CLK_DIV_NUM_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_t0_src_sel(&self) -> REG_TIMERGRP0_T0_SRC_SEL_R {
        REG_TIMERGRP0_T0_SRC_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_t0_clk_en(&self) -> REG_TIMERGRP0_T0_CLK_EN_R {
        REG_TIMERGRP0_T0_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_t1_src_sel(&self) -> REG_TIMERGRP0_T1_SRC_SEL_R {
        REG_TIMERGRP0_T1_SRC_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_t1_clk_en(&self) -> REG_TIMERGRP0_T1_CLK_EN_R {
        REG_TIMERGRP0_T1_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_wdt_src_sel(&self) -> REG_TIMERGRP0_WDT_SRC_SEL_R {
        REG_TIMERGRP0_WDT_SRC_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_wdt_clk_en(&self) -> REG_TIMERGRP0_WDT_CLK_EN_R {
        REG_TIMERGRP0_WDT_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_en(&self) -> REG_TIMERGRP0_TGRT_CLK_EN_R {
        REG_TIMERGRP0_TGRT_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL20")
            .field(
                "reg_mcpwm0_clk_src_sel",
                &format_args!("{}", self.reg_mcpwm0_clk_src_sel().bits()),
            )
            .field(
                "reg_mcpwm0_clk_en",
                &format_args!("{}", self.reg_mcpwm0_clk_en().bit()),
            )
            .field(
                "reg_mcpwm0_clk_div_num",
                &format_args!("{}", self.reg_mcpwm0_clk_div_num().bits()),
            )
            .field(
                "reg_mcpwm1_clk_src_sel",
                &format_args!("{}", self.reg_mcpwm1_clk_src_sel().bits()),
            )
            .field(
                "reg_mcpwm1_clk_en",
                &format_args!("{}", self.reg_mcpwm1_clk_en().bit()),
            )
            .field(
                "reg_mcpwm1_clk_div_num",
                &format_args!("{}", self.reg_mcpwm1_clk_div_num().bits()),
            )
            .field(
                "reg_timergrp0_t0_src_sel",
                &format_args!("{}", self.reg_timergrp0_t0_src_sel().bits()),
            )
            .field(
                "reg_timergrp0_t0_clk_en",
                &format_args!("{}", self.reg_timergrp0_t0_clk_en().bit()),
            )
            .field(
                "reg_timergrp0_t1_src_sel",
                &format_args!("{}", self.reg_timergrp0_t1_src_sel().bits()),
            )
            .field(
                "reg_timergrp0_t1_clk_en",
                &format_args!("{}", self.reg_timergrp0_t1_clk_en().bit()),
            )
            .field(
                "reg_timergrp0_wdt_src_sel",
                &format_args!("{}", self.reg_timergrp0_wdt_src_sel().bits()),
            )
            .field(
                "reg_timergrp0_wdt_clk_en",
                &format_args!("{}", self.reg_timergrp0_wdt_clk_en().bit()),
            )
            .field(
                "reg_timergrp0_tgrt_clk_en",
                &format_args!("{}", self.reg_timergrp0_tgrt_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL20_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcpwm0_clk_src_sel(&mut self) -> REG_MCPWM0_CLK_SRC_SEL_W<PERI_CLK_CTRL20_SPEC> {
        REG_MCPWM0_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcpwm0_clk_en(&mut self) -> REG_MCPWM0_CLK_EN_W<PERI_CLK_CTRL20_SPEC> {
        REG_MCPWM0_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcpwm0_clk_div_num(&mut self) -> REG_MCPWM0_CLK_DIV_NUM_W<PERI_CLK_CTRL20_SPEC> {
        REG_MCPWM0_CLK_DIV_NUM_W::new(self, 3)
    }
    #[doc = "Bits 11:12 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcpwm1_clk_src_sel(&mut self) -> REG_MCPWM1_CLK_SRC_SEL_W<PERI_CLK_CTRL20_SPEC> {
        REG_MCPWM1_CLK_SRC_SEL_W::new(self, 11)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcpwm1_clk_en(&mut self) -> REG_MCPWM1_CLK_EN_W<PERI_CLK_CTRL20_SPEC> {
        REG_MCPWM1_CLK_EN_W::new(self, 13)
    }
    #[doc = "Bits 14:21 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcpwm1_clk_div_num(&mut self) -> REG_MCPWM1_CLK_DIV_NUM_W<PERI_CLK_CTRL20_SPEC> {
        REG_MCPWM1_CLK_DIV_NUM_W::new(self, 14)
    }
    #[doc = "Bits 22:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_t0_src_sel(&mut self) -> REG_TIMERGRP0_T0_SRC_SEL_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_T0_SRC_SEL_W::new(self, 22)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_t0_clk_en(&mut self) -> REG_TIMERGRP0_T0_CLK_EN_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_T0_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_t1_src_sel(&mut self) -> REG_TIMERGRP0_T1_SRC_SEL_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_T1_SRC_SEL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_t1_clk_en(&mut self) -> REG_TIMERGRP0_T1_CLK_EN_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_T1_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_wdt_src_sel(
        &mut self,
    ) -> REG_TIMERGRP0_WDT_SRC_SEL_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_WDT_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_wdt_clk_en(&mut self) -> REG_TIMERGRP0_WDT_CLK_EN_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_WDT_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_timergrp0_tgrt_clk_en(
        &mut self,
    ) -> REG_TIMERGRP0_TGRT_CLK_EN_W<PERI_CLK_CTRL20_SPEC> {
        REG_TIMERGRP0_TGRT_CLK_EN_W::new(self, 31)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL20_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl20::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL20_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl20::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL20_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL20 to value 0xc900_0000"]
impl crate::Resettable for PERI_CLK_CTRL20_SPEC {
    const RESET_VALUE: Self::Ux = 0xc900_0000;
}
