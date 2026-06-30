#[doc = "Register `TIMERGRP0_TGRT_CTRL0` reader"]
pub type R = crate::R<TIMERGRP0_TGRT_CTRL0_SPEC>;
#[doc = "Register `TIMERGRP0_TGRT_CTRL0` writer"]
pub type W = crate::W<TIMERGRP0_TGRT_CTRL0_SPEC>;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_EN` reader - need_des"]
pub type REG_TIMERGRP0_TGRT_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_EN` writer - need_des"]
pub type REG_TIMERGRP0_TGRT_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_SRC_SEL` reader - need_des"]
pub type REG_TIMERGRP0_TGRT_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_SRC_SEL` writer - need_des"]
pub type REG_TIMERGRP0_TGRT_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_DIV_NUM` reader - need_des"]
pub type REG_TIMERGRP0_TGRT_CLK_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `REG_TIMERGRP0_TGRT_CLK_DIV_NUM` writer - need_des"]
pub type REG_TIMERGRP0_TGRT_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_en(&self) -> REG_TIMERGRP0_TGRT_CLK_EN_R {
        REG_TIMERGRP0_TGRT_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - need_des"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_src_sel(&self) -> REG_TIMERGRP0_TGRT_CLK_SRC_SEL_R {
        REG_TIMERGRP0_TGRT_CLK_SRC_SEL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:20 - need_des"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_div_num(&self) -> REG_TIMERGRP0_TGRT_CLK_DIV_NUM_R {
        REG_TIMERGRP0_TGRT_CLK_DIV_NUM_R::new(((self.bits >> 5) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGRP0_TGRT_CTRL0")
            .field(
                "reg_timergrp0_tgrt_clk_en",
                &self.reg_timergrp0_tgrt_clk_en(),
            )
            .field(
                "reg_timergrp0_tgrt_clk_src_sel",
                &self.reg_timergrp0_tgrt_clk_src_sel(),
            )
            .field(
                "reg_timergrp0_tgrt_clk_div_num",
                &self.reg_timergrp0_tgrt_clk_div_num(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_en(
        &mut self,
    ) -> REG_TIMERGRP0_TGRT_CLK_EN_W<'_, TIMERGRP0_TGRT_CTRL0_SPEC> {
        REG_TIMERGRP0_TGRT_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - need_des"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_src_sel(
        &mut self,
    ) -> REG_TIMERGRP0_TGRT_CLK_SRC_SEL_W<'_, TIMERGRP0_TGRT_CTRL0_SPEC> {
        REG_TIMERGRP0_TGRT_CLK_SRC_SEL_W::new(self, 1)
    }
    #[doc = "Bits 5:20 - need_des"]
    #[inline(always)]
    pub fn reg_timergrp0_tgrt_clk_div_num(
        &mut self,
    ) -> REG_TIMERGRP0_TGRT_CLK_DIV_NUM_W<'_, TIMERGRP0_TGRT_CTRL0_SPEC> {
        REG_TIMERGRP0_TGRT_CLK_DIV_NUM_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`timergrp0_tgrt_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergrp0_tgrt_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGRP0_TGRT_CTRL0_SPEC;
impl crate::RegisterSpec for TIMERGRP0_TGRT_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergrp0_tgrt_ctrl0::R`](R) reader structure"]
impl crate::Readable for TIMERGRP0_TGRT_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergrp0_tgrt_ctrl0::W`](W) writer structure"]
impl crate::Writable for TIMERGRP0_TGRT_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERGRP0_TGRT_CTRL0 to value 0x11"]
impl crate::Resettable for TIMERGRP0_TGRT_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
