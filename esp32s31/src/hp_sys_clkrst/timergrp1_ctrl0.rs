#[doc = "Register `TIMERGRP1_CTRL0` reader"]
pub type R = crate::R<TIMERGRP1_CTRL0_SPEC>;
#[doc = "Register `TIMERGRP1_CTRL0` writer"]
pub type W = crate::W<TIMERGRP1_CTRL0_SPEC>;
#[doc = "Field `APB_CLK_EN` reader - need_des"]
pub type APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `APB_CLK_EN` writer - need_des"]
pub type APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN` reader - need_des"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - need_des"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_NORST` reader - need_des"]
pub type FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FORCE_NORST` writer - need_des"]
pub type FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0_SRC_SEL` reader - need_des"]
pub type T0_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `T0_SRC_SEL` writer - need_des"]
pub type T0_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `T0_CLK_EN` reader - need_des"]
pub type T0_CLK_EN_R = crate::BitReader;
#[doc = "Field `T0_CLK_EN` writer - need_des"]
pub type T0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1_SRC_SEL` reader - need_des"]
pub type T1_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `T1_SRC_SEL` writer - need_des"]
pub type T1_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `T1_CLK_EN` reader - need_des"]
pub type T1_CLK_EN_R = crate::BitReader;
#[doc = "Field `T1_CLK_EN` writer - need_des"]
pub type T1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_SRC_SEL` reader - need_des"]
pub type WDT_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `WDT_SRC_SEL` writer - need_des"]
pub type WDT_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDT_CLK_EN` reader - need_des"]
pub type WDT_CLK_EN_R = crate::BitReader;
#[doc = "Field `WDT_CLK_EN` writer - need_des"]
pub type WDT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&self) -> APB_CLK_EN_R {
        APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&self) -> FORCE_NORST_R {
        FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn t0_src_sel(&self) -> T0_SRC_SEL_R {
        T0_SRC_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn t0_clk_en(&self) -> T0_CLK_EN_R {
        T0_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    pub fn t1_src_sel(&self) -> T1_SRC_SEL_R {
        T1_SRC_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn t1_clk_en(&self) -> T1_CLK_EN_R {
        T1_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - need_des"]
    #[inline(always)]
    pub fn wdt_src_sel(&self) -> WDT_SRC_SEL_R {
        WDT_SRC_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn wdt_clk_en(&self) -> WDT_CLK_EN_R {
        WDT_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGRP1_CTRL0")
            .field("apb_clk_en", &self.apb_clk_en())
            .field("rst_en", &self.rst_en())
            .field("force_norst", &self.force_norst())
            .field("t0_src_sel", &self.t0_src_sel())
            .field("t0_clk_en", &self.t0_clk_en())
            .field("t1_src_sel", &self.t1_src_sel())
            .field("t1_clk_en", &self.t1_clk_en())
            .field("wdt_src_sel", &self.wdt_src_sel())
            .field("wdt_clk_en", &self.wdt_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&mut self) -> APB_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&mut self) -> FORCE_NORST_W<'_, TIMERGRP1_CTRL0_SPEC> {
        FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn t0_src_sel(&mut self) -> T0_SRC_SEL_W<'_, TIMERGRP1_CTRL0_SPEC> {
        T0_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn t0_clk_en(&mut self) -> T0_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        T0_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - need_des"]
    #[inline(always)]
    pub fn t1_src_sel(&mut self) -> T1_SRC_SEL_W<'_, TIMERGRP1_CTRL0_SPEC> {
        T1_SRC_SEL_W::new(self, 6)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn t1_clk_en(&mut self) -> T1_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        T1_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - need_des"]
    #[inline(always)]
    pub fn wdt_src_sel(&mut self) -> WDT_SRC_SEL_W<'_, TIMERGRP1_CTRL0_SPEC> {
        WDT_SRC_SEL_W::new(self, 9)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn wdt_clk_en(&mut self) -> WDT_CLK_EN_W<'_, TIMERGRP1_CTRL0_SPEC> {
        WDT_CLK_EN_W::new(self, 11)
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
