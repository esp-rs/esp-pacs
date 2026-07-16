#[doc = "Register `TIMERGRP1_CTRL0` reader"]
pub type R = crate::R<TIMERGRP1_CTRL0_SPEC>;
#[doc = "Register `TIMERGRP1_CTRL0` writer"]
pub type W = crate::W<TIMERGRP1_CTRL0_SPEC>;
#[doc = "Field `TIMERGRP1_APB_CLK_EN` reader - need_des"]
pub type TIMERGRP1_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_APB_CLK_EN` writer - need_des"]
pub type TIMERGRP1_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_RST_EN` reader - need_des"]
pub type TIMERGRP1_RST_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_RST_EN` writer - need_des"]
pub type TIMERGRP1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_FORCE_NORST` reader - need_des"]
pub type TIMERGRP1_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_FORCE_NORST` writer - need_des"]
pub type TIMERGRP1_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_T0_SRC_SEL` reader - need_des"]
pub type TIMERGRP1_T0_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP1_T0_SRC_SEL` writer - need_des"]
pub type TIMERGRP1_T0_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_T0_CLK_EN` reader - need_des"]
pub type TIMERGRP1_T0_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_T0_CLK_EN` writer - need_des"]
pub type TIMERGRP1_T0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_T1_SRC_SEL` reader - need_des"]
pub type TIMERGRP1_T1_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP1_T1_SRC_SEL` writer - need_des"]
pub type TIMERGRP1_T1_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_T1_CLK_EN` reader - need_des"]
pub type TIMERGRP1_T1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_T1_CLK_EN` writer - need_des"]
pub type TIMERGRP1_T1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMERGRP1_WDT_SRC_SEL` reader - need_des"]
pub type TIMERGRP1_WDT_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `TIMERGRP1_WDT_SRC_SEL` writer - need_des"]
pub type TIMERGRP1_WDT_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERGRP1_WDT_CLK_EN` reader - need_des"]
pub type TIMERGRP1_WDT_CLK_EN_R = crate::BitReader;
#[doc = "Field `TIMERGRP1_WDT_CLK_EN` writer - need_des"]
pub type TIMERGRP1_WDT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn timergrp1_apb_clk_en(&self) -> TIMERGRP1_APB_CLK_EN_R {
        TIMERGRP1_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn timergrp1_rst_en(&self) -> TIMERGRP1_RST_EN_R {
        TIMERGRP1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn timergrp1_force_norst(&self) -> TIMERGRP1_FORCE_NORST_R {
        TIMERGRP1_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t0_src_sel(&self) -> TIMERGRP1_T0_SRC_SEL_R {
        TIMERGRP1_T0_SRC_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t0_clk_en(&self) -> TIMERGRP1_T0_CLK_EN_R {
        TIMERGRP1_T0_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t1_src_sel(&self) -> TIMERGRP1_T1_SRC_SEL_R {
        TIMERGRP1_T1_SRC_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t1_clk_en(&self) -> TIMERGRP1_T1_CLK_EN_R {
        TIMERGRP1_T1_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - need_des"]
    #[inline(always)]
    pub fn timergrp1_wdt_src_sel(&self) -> TIMERGRP1_WDT_SRC_SEL_R {
        TIMERGRP1_WDT_SRC_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn timergrp1_wdt_clk_en(&self) -> TIMERGRP1_WDT_CLK_EN_R {
        TIMERGRP1_WDT_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGRP1_CTRL0")
            .field("timergrp1_apb_clk_en", &self.timergrp1_apb_clk_en())
            .field("timergrp1_rst_en", &self.timergrp1_rst_en())
            .field("timergrp1_force_norst", &self.timergrp1_force_norst())
            .field("timergrp1_t0_src_sel", &self.timergrp1_t0_src_sel())
            .field("timergrp1_t0_clk_en", &self.timergrp1_t0_clk_en())
            .field("timergrp1_t1_src_sel", &self.timergrp1_t1_src_sel())
            .field("timergrp1_t1_clk_en", &self.timergrp1_t1_clk_en())
            .field("timergrp1_wdt_src_sel", &self.timergrp1_wdt_src_sel())
            .field("timergrp1_wdt_clk_en", &self.timergrp1_wdt_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn timergrp1_apb_clk_en(&mut self) -> TIMERGRP1_APB_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn timergrp1_rst_en(&mut self) -> TIMERGRP1_RST_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn timergrp1_force_norst(&mut self) -> TIMERGRP1_FORCE_NORST_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t0_src_sel(&mut self) -> TIMERGRP1_T0_SRC_SEL_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_T0_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t0_clk_en(&mut self) -> TIMERGRP1_T0_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_T0_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t1_src_sel(&mut self) -> TIMERGRP1_T1_SRC_SEL_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_T1_SRC_SEL_W::new(self, 6)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn timergrp1_t1_clk_en(&mut self) -> TIMERGRP1_T1_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_T1_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - need_des"]
    #[inline(always)]
    pub fn timergrp1_wdt_src_sel(&mut self) -> TIMERGRP1_WDT_SRC_SEL_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_WDT_SRC_SEL_W::new(self, 9)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn timergrp1_wdt_clk_en(&mut self) -> TIMERGRP1_WDT_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        TIMERGRP1_WDT_CLK_EN_W::new(self, 11)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp1_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp1_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGRP1_CTRL0_SPEC;
impl crate::RegisterSpec for TIMERGRP1_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergrp1_ctrl0::R`](R) reader structure"]
impl crate::Readable for TIMERGRP1_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergrp1_ctrl0::W`](W) writer structure"]
impl crate::Writable for TIMERGRP1_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERGRP1_CTRL0 to value 0x0921"]
impl crate::Resettable for TIMERGRP1_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0921;
}
