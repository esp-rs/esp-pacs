#[doc = "Register `PERI_CLK_CTRL00` reader"]
pub type R = crate::R<PERI_CLK_CTRL00_SPEC>;
#[doc = "Register `PERI_CLK_CTRL00` writer"]
pub type W = crate::W<PERI_CLK_CTRL00_SPEC>;
#[doc = "Field `FLASH_CLK_SRC_SEL` reader - Reserved"]
pub type FLASH_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `FLASH_CLK_SRC_SEL` writer - Reserved"]
pub type FLASH_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLASH_PLL_CLK_EN` reader - Reserved"]
pub type FLASH_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PLL_CLK_EN` writer - Reserved"]
pub type FLASH_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CORE_CLK_EN` reader - Reserved"]
pub type FLASH_CORE_CLK_EN_R = crate::BitReader;
#[doc = "Field `FLASH_CORE_CLK_EN` writer - Reserved"]
pub type FLASH_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_CORE_CLK_DIV_NUM` reader - Reserved"]
pub type FLASH_CORE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `FLASH_CORE_CLK_DIV_NUM` writer - Reserved"]
pub type FLASH_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PSRAM_CLK_SRC_SEL` reader - Reserved"]
pub type PSRAM_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `PSRAM_CLK_SRC_SEL` writer - Reserved"]
pub type PSRAM_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSRAM_PLL_CLK_EN` reader - Reserved"]
pub type PSRAM_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_PLL_CLK_EN` writer - Reserved"]
pub type PSRAM_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_CORE_CLK_EN` reader - Reserved"]
pub type PSRAM_CORE_CLK_EN_R = crate::BitReader;
#[doc = "Field `PSRAM_CORE_CLK_EN` writer - Reserved"]
pub type PSRAM_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_CORE_CLK_DIV_NUM` reader - Reserved"]
pub type PSRAM_CORE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PSRAM_CORE_CLK_DIV_NUM` writer - Reserved"]
pub type PSRAM_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PAD_EMAC_REF_CLK_EN` reader - Reserved"]
pub type PAD_EMAC_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `PAD_EMAC_REF_CLK_EN` writer - Reserved"]
pub type PAD_EMAC_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RMII_CLK_SRC_SEL` reader - Reserved"]
pub type EMAC_RMII_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `EMAC_RMII_CLK_SRC_SEL` writer - Reserved"]
pub type EMAC_RMII_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMAC_RMII_CLK_EN` reader - Reserved"]
pub type EMAC_RMII_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_RMII_CLK_EN` writer - Reserved"]
pub type EMAC_RMII_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RX_CLK_SRC_SEL` reader - Reserved"]
pub type EMAC_RX_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `EMAC_RX_CLK_SRC_SEL` writer - Reserved"]
pub type EMAC_RX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_RX_CLK_EN` reader - Reserved"]
pub type EMAC_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_RX_CLK_EN` writer - Reserved"]
pub type EMAC_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn flash_clk_src_sel(&self) -> FLASH_CLK_SRC_SEL_R {
        FLASH_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn flash_pll_clk_en(&self) -> FLASH_PLL_CLK_EN_R {
        FLASH_PLL_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_en(&self) -> FLASH_CORE_CLK_EN_R {
        FLASH_CORE_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_div_num(&self) -> FLASH_CORE_CLK_DIV_NUM_R {
        FLASH_CORE_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn psram_clk_src_sel(&self) -> PSRAM_CLK_SRC_SEL_R {
        PSRAM_CLK_SRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn psram_pll_clk_en(&self) -> PSRAM_PLL_CLK_EN_R {
        PSRAM_PLL_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_en(&self) -> PSRAM_CORE_CLK_EN_R {
        PSRAM_CORE_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_div_num(&self) -> PSRAM_CORE_CLK_DIV_NUM_R {
        PSRAM_CORE_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pad_emac_ref_clk_en(&self) -> PAD_EMAC_REF_CLK_EN_R {
        PAD_EMAC_REF_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_src_sel(&self) -> EMAC_RMII_CLK_SRC_SEL_R {
        EMAC_RMII_CLK_SRC_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_en(&self) -> EMAC_RMII_CLK_EN_R {
        EMAC_RMII_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_src_sel(&self) -> EMAC_RX_CLK_SRC_SEL_R {
        EMAC_RX_CLK_SRC_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_en(&self) -> EMAC_RX_CLK_EN_R {
        EMAC_RX_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL00")
            .field("flash_clk_src_sel", &self.flash_clk_src_sel())
            .field("flash_pll_clk_en", &self.flash_pll_clk_en())
            .field("flash_core_clk_en", &self.flash_core_clk_en())
            .field("flash_core_clk_div_num", &self.flash_core_clk_div_num())
            .field("psram_clk_src_sel", &self.psram_clk_src_sel())
            .field("psram_pll_clk_en", &self.psram_pll_clk_en())
            .field("psram_core_clk_en", &self.psram_core_clk_en())
            .field("psram_core_clk_div_num", &self.psram_core_clk_div_num())
            .field("pad_emac_ref_clk_en", &self.pad_emac_ref_clk_en())
            .field("emac_rmii_clk_src_sel", &self.emac_rmii_clk_src_sel())
            .field("emac_rmii_clk_en", &self.emac_rmii_clk_en())
            .field("emac_rx_clk_src_sel", &self.emac_rx_clk_src_sel())
            .field("emac_rx_clk_en", &self.emac_rx_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn flash_clk_src_sel(&mut self) -> FLASH_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn flash_pll_clk_en(&mut self) -> FLASH_PLL_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_PLL_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_en(&mut self) -> FLASH_CORE_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_CORE_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn flash_core_clk_div_num(&mut self) -> FLASH_CORE_CLK_DIV_NUM_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_CORE_CLK_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn psram_clk_src_sel(&mut self) -> PSRAM_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_CLK_SRC_SEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn psram_pll_clk_en(&mut self) -> PSRAM_PLL_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_PLL_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_en(&mut self) -> PSRAM_CORE_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_CORE_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn psram_core_clk_div_num(&mut self) -> PSRAM_CORE_CLK_DIV_NUM_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_CORE_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn pad_emac_ref_clk_en(&mut self) -> PAD_EMAC_REF_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        PAD_EMAC_REF_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_src_sel(&mut self) -> EMAC_RMII_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RMII_CLK_SRC_SEL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn emac_rmii_clk_en(&mut self) -> EMAC_RMII_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RMII_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_src_sel(&mut self) -> EMAC_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RX_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_en(&mut self) -> EMAC_RX_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RX_CLK_EN_W::new(self, 29)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL00_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl00::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL00_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl00::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL00_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL00 to value 0xc03c"]
impl crate::Resettable for PERI_CLK_CTRL00_SPEC {
    const RESET_VALUE: u32 = 0xc03c;
}
