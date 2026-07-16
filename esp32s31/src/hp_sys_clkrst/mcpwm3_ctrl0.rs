#[doc = "Register `MCPWM3_CTRL0` reader"]
pub type R = crate::R<MCPWM3_CTRL0_SPEC>;
#[doc = "Register `MCPWM3_CTRL0` writer"]
pub type W = crate::W<MCPWM3_CTRL0_SPEC>;
#[doc = "Field `MCPWM3_APB_CLK_EN` reader - need_des"]
pub type MCPWM3_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `MCPWM3_APB_CLK_EN` writer - need_des"]
pub type MCPWM3_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_RST_EN` reader - need_des"]
pub type MCPWM3_RST_EN_R = crate::BitReader;
#[doc = "Field `MCPWM3_RST_EN` writer - need_des"]
pub type MCPWM3_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_FORCE_NORST` reader - need_des"]
pub type MCPWM3_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `MCPWM3_FORCE_NORST` writer - need_des"]
pub type MCPWM3_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_CLK_SRC_SEL` reader - need_des"]
pub type MCPWM3_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `MCPWM3_CLK_SRC_SEL` writer - need_des"]
pub type MCPWM3_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCPWM3_CLK_EN` reader - need_des"]
pub type MCPWM3_CLK_EN_R = crate::BitReader;
#[doc = "Field `MCPWM3_CLK_EN` writer - need_des"]
pub type MCPWM3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM3_CLK_DIV_NUM` reader - need_des"]
pub type MCPWM3_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MCPWM3_CLK_DIV_NUM` writer - need_des"]
pub type MCPWM3_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_apb_clk_en(&self) -> MCPWM3_APB_CLK_EN_R {
        MCPWM3_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_rst_en(&self) -> MCPWM3_RST_EN_R {
        MCPWM3_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_force_norst(&self) -> MCPWM3_FORCE_NORST_R {
        MCPWM3_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_clk_src_sel(&self) -> MCPWM3_CLK_SRC_SEL_R {
        MCPWM3_CLK_SRC_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_clk_en(&self) -> MCPWM3_CLK_EN_R {
        MCPWM3_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_clk_div_num(&self) -> MCPWM3_CLK_DIV_NUM_R {
        MCPWM3_CLK_DIV_NUM_R::new(((self.bits >> 6) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCPWM3_CTRL0")
            .field("mcpwm3_apb_clk_en", &self.mcpwm3_apb_clk_en())
            .field("mcpwm3_rst_en", &self.mcpwm3_rst_en())
            .field("mcpwm3_force_norst", &self.mcpwm3_force_norst())
            .field("mcpwm3_clk_src_sel", &self.mcpwm3_clk_src_sel())
            .field("mcpwm3_clk_en", &self.mcpwm3_clk_en())
            .field("mcpwm3_clk_div_num", &self.mcpwm3_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_apb_clk_en(&mut self) -> MCPWM3_APB_CLK_EN_W<'_, MCPWM3_CTRL0_SPEC> {
        MCPWM3_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_rst_en(&mut self) -> MCPWM3_RST_EN_W<'_, MCPWM3_CTRL0_SPEC> {
        MCPWM3_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_force_norst(&mut self) -> MCPWM3_FORCE_NORST_W<'_, MCPWM3_CTRL0_SPEC> {
        MCPWM3_FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_clk_src_sel(&mut self) -> MCPWM3_CLK_SRC_SEL_W<'_, MCPWM3_CTRL0_SPEC> {
        MCPWM3_CLK_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_clk_en(&mut self) -> MCPWM3_CLK_EN_W<'_, MCPWM3_CTRL0_SPEC> {
        MCPWM3_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:13 - need_des"]
    #[inline(always)]
    pub fn mcpwm3_clk_div_num(&mut self) -> MCPWM3_CLK_DIV_NUM_W<'_, MCPWM3_CTRL0_SPEC> {
        MCPWM3_CLK_DIV_NUM_W::new(self, 6)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mcpwm3_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcpwm3_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCPWM3_CTRL0_SPEC;
impl crate::RegisterSpec for MCPWM3_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcpwm3_ctrl0::R`](R) reader structure"]
impl crate::Readable for MCPWM3_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcpwm3_ctrl0::W`](W) writer structure"]
impl crate::Writable for MCPWM3_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCPWM3_CTRL0 to value 0"]
impl crate::Resettable for MCPWM3_CTRL0_SPEC {}
