#[doc = "Register `SYSTIMER_CTRL0` reader"]
pub type R = crate::R<SYSTIMER_CTRL0_SPEC>;
#[doc = "Register `SYSTIMER_CTRL0` writer"]
pub type W = crate::W<SYSTIMER_CTRL0_SPEC>;
#[doc = "Field `SYSTIMER_APB_CLK_EN` reader - need_des"]
pub type SYSTIMER_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_APB_CLK_EN` writer - need_des"]
pub type SYSTIMER_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_RST_EN` reader - need_des"]
pub type SYSTIMER_RST_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_RST_EN` writer - need_des"]
pub type SYSTIMER_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_FORCE_NORST` reader - need_des"]
pub type SYSTIMER_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `SYSTIMER_FORCE_NORST` writer - need_des"]
pub type SYSTIMER_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_SRC_SEL` reader - need_des"]
pub type SYSTIMER_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_SRC_SEL` writer - need_des"]
pub type SYSTIMER_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSTIMER_CLK_EN` reader - need_des"]
pub type SYSTIMER_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYSTIMER_CLK_EN` writer - need_des"]
pub type SYSTIMER_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn systimer_apb_clk_en(&self) -> SYSTIMER_APB_CLK_EN_R {
        SYSTIMER_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn systimer_rst_en(&self) -> SYSTIMER_RST_EN_R {
        SYSTIMER_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn systimer_force_norst(&self) -> SYSTIMER_FORCE_NORST_R {
        SYSTIMER_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn systimer_clk_src_sel(&self) -> SYSTIMER_CLK_SRC_SEL_R {
        SYSTIMER_CLK_SRC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn systimer_clk_en(&self) -> SYSTIMER_CLK_EN_R {
        SYSTIMER_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTIMER_CTRL0")
            .field("systimer_apb_clk_en", &self.systimer_apb_clk_en())
            .field("systimer_rst_en", &self.systimer_rst_en())
            .field("systimer_force_norst", &self.systimer_force_norst())
            .field("systimer_clk_src_sel", &self.systimer_clk_src_sel())
            .field("systimer_clk_en", &self.systimer_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn systimer_apb_clk_en(&mut self) -> SYSTIMER_APB_CLK_EN_W<'_, SYSTIMER_CTRL0_SPEC> {
        SYSTIMER_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn systimer_rst_en(&mut self) -> SYSTIMER_RST_EN_W<'_, SYSTIMER_CTRL0_SPEC> {
        SYSTIMER_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn systimer_force_norst(&mut self) -> SYSTIMER_FORCE_NORST_W<'_, SYSTIMER_CTRL0_SPEC> {
        SYSTIMER_FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn systimer_clk_src_sel(&mut self) -> SYSTIMER_CLK_SRC_SEL_W<'_, SYSTIMER_CTRL0_SPEC> {
        SYSTIMER_CLK_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn systimer_clk_en(&mut self) -> SYSTIMER_CLK_EN_W<'_, SYSTIMER_CTRL0_SPEC> {
        SYSTIMER_CLK_EN_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`systimer_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systimer_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSTIMER_CTRL0_SPEC;
impl crate::RegisterSpec for SYSTIMER_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systimer_ctrl0::R`](R) reader structure"]
impl crate::Readable for SYSTIMER_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`systimer_ctrl0::W`](W) writer structure"]
impl crate::Writable for SYSTIMER_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSTIMER_CTRL0 to value 0"]
impl crate::Resettable for SYSTIMER_CTRL0_SPEC {}
