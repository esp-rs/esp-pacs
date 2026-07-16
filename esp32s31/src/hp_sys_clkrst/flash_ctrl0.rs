#[doc = "Register `FLASH_CTRL0` reader"]
pub type R = crate::R<FLASH_CTRL0_SPEC>;
#[doc = "Register `FLASH_CTRL0` writer"]
pub type W = crate::W<FLASH_CTRL0_SPEC>;
#[doc = "Field `FLASH_SYS_CLK_EN` reader - need_des"]
pub type FLASH_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `FLASH_SYS_CLK_EN` writer - need_des"]
pub type FLASH_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_AXI_RST_EN` reader - need_des"]
pub type FLASH_AXI_RST_EN_R = crate::BitReader;
#[doc = "Field `FLASH_AXI_RST_EN` writer - need_des"]
pub type FLASH_AXI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_APB_RST_EN` reader - need_des"]
pub type FLASH_APB_RST_EN_R = crate::BitReader;
#[doc = "Field `FLASH_APB_RST_EN` writer - need_des"]
pub type FLASH_APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_AXI_FORCE_NORST` reader - need_des"]
pub type FLASH_AXI_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FLASH_AXI_FORCE_NORST` writer - need_des"]
pub type FLASH_AXI_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_APB_FORCE_NORST` reader - need_des"]
pub type FLASH_APB_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `FLASH_APB_FORCE_NORST` writer - need_des"]
pub type FLASH_APB_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CLK_SRC_SEL` reader - need_des"]
pub type FLASH_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `FLASH_CLK_SRC_SEL` writer - need_des"]
pub type FLASH_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_PLL_CLK_EN` reader - need_des"]
pub type FLASH_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PLL_CLK_EN` writer - need_des"]
pub type FLASH_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CORE_CLK_EN` reader - need_des"]
pub type FLASH_CORE_CLK_EN_R = crate::BitReader;
#[doc = "Field `FLASH_CORE_CLK_EN` writer - need_des"]
pub type FLASH_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CORE_CLK_DIV_NUM` reader - need_des"]
pub type FLASH_CORE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `FLASH_CORE_CLK_DIV_NUM` writer - need_des"]
pub type FLASH_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn flash_sys_clk_en(&self) -> FLASH_SYS_CLK_EN_R {
        FLASH_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn flash_axi_rst_en(&self) -> FLASH_AXI_RST_EN_R {
        FLASH_AXI_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn flash_apb_rst_en(&self) -> FLASH_APB_RST_EN_R {
        FLASH_APB_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn flash_axi_force_norst(&self) -> FLASH_AXI_FORCE_NORST_R {
        FLASH_AXI_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn flash_apb_force_norst(&self) -> FLASH_APB_FORCE_NORST_R {
        FLASH_APB_FORCE_NORST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn flash_clk_src_sel(&self) -> FLASH_CLK_SRC_SEL_R {
        FLASH_CLK_SRC_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn flash_pll_clk_en(&self) -> FLASH_PLL_CLK_EN_R {
        FLASH_PLL_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn flash_core_clk_en(&self) -> FLASH_CORE_CLK_EN_R {
        FLASH_CORE_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    pub fn flash_core_clk_div_num(&self) -> FLASH_CORE_CLK_DIV_NUM_R {
        FLASH_CORE_CLK_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_CTRL0")
            .field("flash_sys_clk_en", &self.flash_sys_clk_en())
            .field("flash_axi_rst_en", &self.flash_axi_rst_en())
            .field("flash_apb_rst_en", &self.flash_apb_rst_en())
            .field("flash_axi_force_norst", &self.flash_axi_force_norst())
            .field("flash_apb_force_norst", &self.flash_apb_force_norst())
            .field("flash_clk_src_sel", &self.flash_clk_src_sel())
            .field("flash_pll_clk_en", &self.flash_pll_clk_en())
            .field("flash_core_clk_en", &self.flash_core_clk_en())
            .field("flash_core_clk_div_num", &self.flash_core_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn flash_sys_clk_en(&mut self) -> FLASH_SYS_CLK_EN_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn flash_axi_rst_en(&mut self) -> FLASH_AXI_RST_EN_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_AXI_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn flash_apb_rst_en(&mut self) -> FLASH_APB_RST_EN_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_APB_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn flash_axi_force_norst(&mut self) -> FLASH_AXI_FORCE_NORST_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_AXI_FORCE_NORST_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn flash_apb_force_norst(&mut self) -> FLASH_APB_FORCE_NORST_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_APB_FORCE_NORST_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn flash_clk_src_sel(&mut self) -> FLASH_CLK_SRC_SEL_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_CLK_SRC_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn flash_pll_clk_en(&mut self) -> FLASH_PLL_CLK_EN_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_PLL_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn flash_core_clk_en(&mut self) -> FLASH_CORE_CLK_EN_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_CORE_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    pub fn flash_core_clk_div_num(&mut self) -> FLASH_CORE_CLK_DIV_NUM_W<'_, FLASH_CTRL0_SPEC> {
        FLASH_CORE_CLK_DIV_NUM_W::new(self, 9)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLASH_CTRL0_SPEC;
impl crate::RegisterSpec for FLASH_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_ctrl0::R`](R) reader structure"]
impl crate::Readable for FLASH_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flash_ctrl0::W`](W) writer structure"]
impl crate::Writable for FLASH_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASH_CTRL0 to value 0x0781"]
impl crate::Resettable for FLASH_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0781;
}
