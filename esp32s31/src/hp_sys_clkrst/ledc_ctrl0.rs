#[doc = "Register `LEDC_CTRL0` reader"]
pub type R = crate::R<LEDC_CTRL0_SPEC>;
#[doc = "Register `LEDC_CTRL0` writer"]
pub type W = crate::W<LEDC_CTRL0_SPEC>;
#[doc = "Field `LEDC0_APB_CLK_EN` reader - need_des"]
pub type LEDC0_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC0_APB_CLK_EN` writer - need_des"]
pub type LEDC0_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC0_RST_EN` reader - need_des"]
pub type LEDC0_RST_EN_R = crate::BitReader;
#[doc = "Field `LEDC0_RST_EN` writer - need_des"]
pub type LEDC0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC0_FORCE_NORST` reader - need_des"]
pub type LEDC0_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `LEDC0_FORCE_NORST` writer - need_des"]
pub type LEDC0_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC0_CLK_SRC_SEL` reader - need_des"]
pub type LEDC0_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `LEDC0_CLK_SRC_SEL` writer - need_des"]
pub type LEDC0_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEDC0_CLK_EN` reader - need_des"]
pub type LEDC0_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC0_CLK_EN` writer - need_des"]
pub type LEDC0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC1_APB_CLK_EN` reader - need_des"]
pub type LEDC1_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC1_APB_CLK_EN` writer - need_des"]
pub type LEDC1_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC1_RST_EN` reader - need_des"]
pub type LEDC1_RST_EN_R = crate::BitReader;
#[doc = "Field `LEDC1_RST_EN` writer - need_des"]
pub type LEDC1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC1_FORCE_NORST` reader - need_des"]
pub type LEDC1_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `LEDC1_FORCE_NORST` writer - need_des"]
pub type LEDC1_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEDC1_CLK_SRC_SEL` reader - need_des"]
pub type LEDC1_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `LEDC1_CLK_SRC_SEL` writer - need_des"]
pub type LEDC1_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEDC1_CLK_EN` reader - need_des"]
pub type LEDC1_CLK_EN_R = crate::BitReader;
#[doc = "Field `LEDC1_CLK_EN` writer - need_des"]
pub type LEDC1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ledc0_apb_clk_en(&self) -> LEDC0_APB_CLK_EN_R {
        LEDC0_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ledc0_rst_en(&self) -> LEDC0_RST_EN_R {
        LEDC0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ledc0_force_norst(&self) -> LEDC0_FORCE_NORST_R {
        LEDC0_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn ledc0_clk_src_sel(&self) -> LEDC0_CLK_SRC_SEL_R {
        LEDC0_CLK_SRC_SEL_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn ledc0_clk_en(&self) -> LEDC0_CLK_EN_R {
        LEDC0_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn ledc1_apb_clk_en(&self) -> LEDC1_APB_CLK_EN_R {
        LEDC1_APB_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn ledc1_rst_en(&self) -> LEDC1_RST_EN_R {
        LEDC1_RST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn ledc1_force_norst(&self) -> LEDC1_FORCE_NORST_R {
        LEDC1_FORCE_NORST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - need_des"]
    #[inline(always)]
    pub fn ledc1_clk_src_sel(&self) -> LEDC1_CLK_SRC_SEL_R {
        LEDC1_CLK_SRC_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn ledc1_clk_en(&self) -> LEDC1_CLK_EN_R {
        LEDC1_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LEDC_CTRL0")
            .field("ledc0_apb_clk_en", &self.ledc0_apb_clk_en())
            .field("ledc0_rst_en", &self.ledc0_rst_en())
            .field("ledc0_force_norst", &self.ledc0_force_norst())
            .field("ledc0_clk_src_sel", &self.ledc0_clk_src_sel())
            .field("ledc0_clk_en", &self.ledc0_clk_en())
            .field("ledc1_apb_clk_en", &self.ledc1_apb_clk_en())
            .field("ledc1_rst_en", &self.ledc1_rst_en())
            .field("ledc1_force_norst", &self.ledc1_force_norst())
            .field("ledc1_clk_src_sel", &self.ledc1_clk_src_sel())
            .field("ledc1_clk_en", &self.ledc1_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ledc0_apb_clk_en(&mut self) -> LEDC0_APB_CLK_EN_W<'_, LEDC_CTRL0_SPEC> {
        LEDC0_APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ledc0_rst_en(&mut self) -> LEDC0_RST_EN_W<'_, LEDC_CTRL0_SPEC> {
        LEDC0_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ledc0_force_norst(&mut self) -> LEDC0_FORCE_NORST_W<'_, LEDC_CTRL0_SPEC> {
        LEDC0_FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - need_des"]
    #[inline(always)]
    pub fn ledc0_clk_src_sel(&mut self) -> LEDC0_CLK_SRC_SEL_W<'_, LEDC_CTRL0_SPEC> {
        LEDC0_CLK_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn ledc0_clk_en(&mut self) -> LEDC0_CLK_EN_W<'_, LEDC_CTRL0_SPEC> {
        LEDC0_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn ledc1_apb_clk_en(&mut self) -> LEDC1_APB_CLK_EN_W<'_, LEDC_CTRL0_SPEC> {
        LEDC1_APB_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn ledc1_rst_en(&mut self) -> LEDC1_RST_EN_W<'_, LEDC_CTRL0_SPEC> {
        LEDC1_RST_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn ledc1_force_norst(&mut self) -> LEDC1_FORCE_NORST_W<'_, LEDC_CTRL0_SPEC> {
        LEDC1_FORCE_NORST_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - need_des"]
    #[inline(always)]
    pub fn ledc1_clk_src_sel(&mut self) -> LEDC1_CLK_SRC_SEL_W<'_, LEDC_CTRL0_SPEC> {
        LEDC1_CLK_SRC_SEL_W::new(self, 9)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn ledc1_clk_en(&mut self) -> LEDC1_CLK_EN_W<'_, LEDC_CTRL0_SPEC> {
        LEDC1_CLK_EN_W::new(self, 11)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ledc_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ledc_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LEDC_CTRL0_SPEC;
impl crate::RegisterSpec for LEDC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ledc_ctrl0::R`](R) reader structure"]
impl crate::Readable for LEDC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ledc_ctrl0::W`](W) writer structure"]
impl crate::Writable for LEDC_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEDC_CTRL0 to value 0"]
impl crate::Resettable for LEDC_CTRL0_SPEC {}
