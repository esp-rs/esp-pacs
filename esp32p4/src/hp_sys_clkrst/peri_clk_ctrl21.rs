#[doc = "Register `PERI_CLK_CTRL21` reader"]
pub type R = crate::R<PERI_CLK_CTRL21_SPEC>;
#[doc = "Register `PERI_CLK_CTRL21` writer"]
pub type W = crate::W<PERI_CLK_CTRL21_SPEC>;
#[doc = "Field `TIMERGRP0_TGRT_CLK_SRC_SEL` reader - Reserved"]
pub type TIMERGRP0_TGRT_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP0_TGRT_CLK_SRC_SEL` writer - Reserved"]
pub type TIMERGRP0_TGRT_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIMERGRP0_TGRT_CLK_DIV_NUM` reader - Reserved"]
pub type TIMERGRP0_TGRT_CLK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TIMERGRP0_TGRT_CLK_DIV_NUM` writer - Reserved"]
pub type TIMERGRP0_TGRT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMERGRP1_T0_SRC_SEL` reader - Reserved"]
pub type TIMERGRP1_T0_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP1_T0_SRC_SEL` writer - Reserved"]
pub type TIMERGRP1_T0_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_T0_CLK_EN` reader - Reserved"]
pub type TIMERGRP1_T0_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_T0_CLK_EN` writer - Reserved"]
pub type TIMERGRP1_T0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_T1_SRC_SEL` reader - Reserved"]
pub type TIMERGRP1_T1_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP1_T1_SRC_SEL` writer - Reserved"]
pub type TIMERGRP1_T1_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_T1_CLK_EN` reader - Reserved"]
pub type TIMERGRP1_T1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_T1_CLK_EN` writer - Reserved"]
pub type TIMERGRP1_T1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_WDT_SRC_SEL` reader - Reserved"]
pub type TIMERGRP1_WDT_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP1_WDT_SRC_SEL` writer - Reserved"]
pub type TIMERGRP1_WDT_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_WDT_CLK_EN` reader - Reserved"]
pub type TIMERGRP1_WDT_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_WDT_CLK_EN` writer - Reserved"]
pub type TIMERGRP1_WDT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_SRC_SEL` reader - Reserved"]
pub type SYSTIMER_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_SRC_SEL` writer - Reserved"]
pub type SYSTIMER_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - Reserved"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - Reserved"]
pub type SYSTIMER_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn timergrp0_tgrt_clk_src_sel(&self) -> TIMERGRP0_TGRT_CLK_SRC_SEL_R {
        TIMERGRP0_TGRT_CLK_SRC_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - Reserved"]
    #[inline(always)]
    pub fn timergrp0_tgrt_clk_div_num(&self) -> TIMERGRP0_TGRT_CLK_DIV_NUM_R {
        TIMERGRP0_TGRT_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:21 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t0_src_sel(&self) -> TIMERGRP1_T0_SRC_SEL_R {
        TIMERGRP1_T0_SRC_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t0_clk_en(&self) -> TIMERGRP1_T0_CLK_EN_R {
        TIMERGRP1_T0_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t1_src_sel(&self) -> TIMERGRP1_T1_SRC_SEL_R {
        TIMERGRP1_T1_SRC_SEL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_t1_clk_en(&self) -> TIMERGRP1_T1_CLK_EN_R {
        TIMERGRP1_T1_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_wdt_src_sel(&self) -> TIMERGRP1_WDT_SRC_SEL_R {
        TIMERGRP1_WDT_SRC_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn timergrp1_wdt_clk_en(&self) -> TIMERGRP1_WDT_CLK_EN_R {
        TIMERGRP1_WDT_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn systimer_clk_src_sel(&self) -> SYSTIMER_CLK_SRC_SEL_R {
        SYSTIMER_CLK_SRC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL21")
            .field(
                "timergrp0_tgrt_clk_src_sel",
                &self.timergrp0_tgrt_clk_src_sel().bits(),
            )
            .field(
                "timergrp0_tgrt_clk_div_num",
                &self.timergrp0_tgrt_clk_div_num().bits(),
            )
            .field("timergrp1_t0_src_sel", &self.timergrp1_t0_src_sel().bits())
            .field("timergrp1_t0_clk_en", &self.timergrp1_t0_clk_en().bit())
            .field("timergrp1_t1_src_sel", &self.timergrp1_t1_src_sel().bits())
            .field("timergrp1_t1_clk_en", &self.timergrp1_t1_clk_en().bit())
            .field(
                "timergrp1_wdt_src_sel",
                &self.timergrp1_wdt_src_sel().bits(),
            )
            .field("timergrp1_wdt_clk_en", &self.timergrp1_wdt_clk_en().bit())
            .field("systimer_clk_src_sel", &self.systimer_clk_src_sel().bit())
            .field("systimer_clk_en", &self.systimer_clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL21_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp0_tgrt_clk_src_sel(
        &mut self,
    ) -> TIMERGRP0_TGRT_CLK_SRC_SEL_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP0_TGRT_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bits 4:19 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp0_tgrt_clk_div_num(
        &mut self,
    ) -> TIMERGRP0_TGRT_CLK_DIV_NUM_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP0_TGRT_CLK_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bits 20:21 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp1_t0_src_sel(&mut self) -> TIMERGRP1_T0_SRC_SEL_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP1_T0_SRC_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp1_t0_clk_en(&mut self) -> TIMERGRP1_T0_CLK_EN_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP1_T0_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp1_t1_src_sel(&mut self) -> TIMERGRP1_T1_SRC_SEL_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP1_T1_SRC_SEL_W::new(self, 23)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp1_t1_clk_en(&mut self) -> TIMERGRP1_T1_CLK_EN_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP1_T1_CLK_EN_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp1_wdt_src_sel(&mut self) -> TIMERGRP1_WDT_SRC_SEL_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP1_WDT_SRC_SEL_W::new(self, 26)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn timergrp1_wdt_clk_en(&mut self) -> TIMERGRP1_WDT_CLK_EN_W<PERI_CLK_CTRL21_SPEC> {
        TIMERGRP1_WDT_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_src_sel(&mut self) -> SYSTIMER_CLK_SRC_SEL_W<PERI_CLK_CTRL21_SPEC> {
        SYSTIMER_CLK_SRC_SEL_W::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<PERI_CLK_CTRL21_SPEC> {
        SYSTIMER_CLK_EN_W::new(self, 30)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL21_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl21::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL21_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl21::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL21_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL21 to value 0x5240_0000"]
impl crate::Resettable for PERI_CLK_CTRL21_SPEC {
    const RESET_VALUE: u32 = 0x5240_0000;
}
