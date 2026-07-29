#[doc = "Register `IOMUX_CTRL0` reader"]
pub type R = crate::R<IOMUX_CTRL0_SPEC>;
#[doc = "Register `IOMUX_CTRL0` writer"]
pub type W = crate::W<IOMUX_CTRL0_SPEC>;
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
#[doc = "Field `CLK_SRC_SEL` reader - need_des"]
pub type CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `CLK_SRC_SEL` writer - need_des"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - need_des"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - need_des"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DIV_NUM` reader - need_des"]
pub type CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CLK_DIV_NUM` writer - need_des"]
pub type CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - need_des"]
    #[inline(always)]
    pub fn clk_div_num(&self) -> CLK_DIV_NUM_R {
        CLK_DIV_NUM_R::new(((self.bits >> 5) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUX_CTRL0")
            .field("apb_clk_en", &self.apb_clk_en())
            .field("rst_en", &self.rst_en())
            .field("force_norst", &self.force_norst())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("clk_en", &self.clk_en())
            .field("clk_div_num", &self.clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn apb_clk_en(&mut self) -> APB_CLK_EN_W<'_, IOMUX_CTRL0_SPEC> {
        APB_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, IOMUX_CTRL0_SPEC> {
        RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn force_norst(&mut self) -> FORCE_NORST_W<'_, IOMUX_CTRL0_SPEC> {
        FORCE_NORST_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<'_, IOMUX_CTRL0_SPEC> {
        CLK_SRC_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, IOMUX_CTRL0_SPEC> {
        CLK_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:12 - need_des"]
    #[inline(always)]
    pub fn clk_div_num(&mut self) -> CLK_DIV_NUM_W<'_, IOMUX_CTRL0_SPEC> {
        CLK_DIV_NUM_W::new(self, 5)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`iomux_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iomux_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOMUX_CTRL0_SPEC;
impl crate::RegisterSpec for IOMUX_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomux_ctrl0::R`](R) reader structure"]
impl crate::Readable for IOMUX_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iomux_ctrl0::W`](W) writer structure"]
impl crate::Writable for IOMUX_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOMUX_CTRL0 to value 0x11"]
impl crate::Resettable for IOMUX_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
