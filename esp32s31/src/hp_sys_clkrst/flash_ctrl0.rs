#[doc = "Register `FLASH_CTRL0` reader"]
pub type R = crate::R<FLASH_CTRL0_SPEC>;
#[doc = "Register `FLASH_CTRL0` writer"]
pub type W = crate::W<FLASH_CTRL0_SPEC>;
#[doc = "Field `SYS_CLK_EN` reader - need_des"]
pub type SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_CLK_EN` writer - need_des"]
pub type SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RST_EN` reader - need_des"]
pub type AXI_RST_EN_R = crate::BitReader;
#[doc = "Field `AXI_RST_EN` writer - need_des"]
pub type AXI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_RST_EN` reader - need_des"]
pub type APB_RST_EN_R = crate::BitReader;
#[doc = "Field `APB_RST_EN` writer - need_des"]
pub type APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_FORCE_NORST` reader - need_des"]
pub type AXI_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `AXI_FORCE_NORST` writer - need_des"]
pub type AXI_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_FORCE_NORST` reader - need_des"]
pub type APB_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `APB_FORCE_NORST` writer - need_des"]
pub type APB_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SRC_SEL` reader - need_des"]
pub type CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SRC_SEL` writer - need_des"]
pub type CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL_CLK_EN` reader - need_des"]
pub type PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `PLL_CLK_EN` writer - need_des"]
pub type PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_CLK_EN` reader - need_des"]
pub type CORE_CLK_EN_R = crate::BitReader;
#[doc = "Field `CORE_CLK_EN` writer - need_des"]
pub type CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_CLK_DIV_NUM` reader - need_des"]
pub type CORE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `CORE_CLK_DIV_NUM` writer - need_des"]
pub type CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&self) -> SYS_CLK_EN_R {
        SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn axi_rst_en(&self) -> AXI_RST_EN_R {
        AXI_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn apb_rst_en(&self) -> APB_RST_EN_R {
        APB_RST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn axi_force_norst(&self) -> AXI_FORCE_NORST_R {
        AXI_FORCE_NORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn apb_force_norst(&self) -> APB_FORCE_NORST_R {
        APB_FORCE_NORST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn pll_clk_en(&self) -> PLL_CLK_EN_R {
        PLL_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn core_clk_en(&self) -> CORE_CLK_EN_R {
        CORE_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    pub fn core_clk_div_num(&self) -> CORE_CLK_DIV_NUM_R {
        CORE_CLK_DIV_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_CTRL0")
            .field("sys_clk_en", &self.sys_clk_en())
            .field("axi_rst_en", &self.axi_rst_en())
            .field("apb_rst_en", &self.apb_rst_en())
            .field("axi_force_norst", &self.axi_force_norst())
            .field("apb_force_norst", &self.apb_force_norst())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("pll_clk_en", &self.pll_clk_en())
            .field("core_clk_en", &self.core_clk_en())
            .field("core_clk_div_num", &self.core_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&mut self) -> SYS_CLK_EN_W<'_, FLASH_CTRL0_SPEC> {
        SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn axi_rst_en(&mut self) -> AXI_RST_EN_W<'_, FLASH_CTRL0_SPEC> {
        AXI_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn apb_rst_en(&mut self) -> APB_RST_EN_W<'_, FLASH_CTRL0_SPEC> {
        APB_RST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn axi_force_norst(&mut self) -> AXI_FORCE_NORST_W<'_, FLASH_CTRL0_SPEC> {
        AXI_FORCE_NORST_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn apb_force_norst(&mut self) -> APB_FORCE_NORST_W<'_, FLASH_CTRL0_SPEC> {
        APB_FORCE_NORST_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - need_des"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<'_, FLASH_CTRL0_SPEC> {
        CLK_SRC_SEL_W::new(self, 5)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn pll_clk_en(&mut self) -> PLL_CLK_EN_W<'_, FLASH_CTRL0_SPEC> {
        PLL_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn core_clk_en(&mut self) -> CORE_CLK_EN_W<'_, FLASH_CTRL0_SPEC> {
        CORE_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:16 - need_des"]
    #[inline(always)]
    pub fn core_clk_div_num(&mut self) -> CORE_CLK_DIV_NUM_W<'_, FLASH_CTRL0_SPEC> {
        CORE_CLK_DIV_NUM_W::new(self, 9)
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
