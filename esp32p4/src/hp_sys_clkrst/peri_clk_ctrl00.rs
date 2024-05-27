///Register `PERI_CLK_CTRL00` reader
pub type R = crate::R<PERI_CLK_CTRL00_SPEC>;
///Register `PERI_CLK_CTRL00` writer
pub type W = crate::W<PERI_CLK_CTRL00_SPEC>;
///Field `FLASH_CLK_SRC_SEL` reader - Reserved
pub type FLASH_CLK_SRC_SEL_R = crate::FieldReader;
///Field `FLASH_CLK_SRC_SEL` writer - Reserved
pub type FLASH_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FLASH_PLL_CLK_EN` reader - Reserved
pub type FLASH_PLL_CLK_EN_R = crate::BitReader;
///Field `FLASH_PLL_CLK_EN` writer - Reserved
pub type FLASH_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_CORE_CLK_EN` reader - Reserved
pub type FLASH_CORE_CLK_EN_R = crate::BitReader;
///Field `FLASH_CORE_CLK_EN` writer - Reserved
pub type FLASH_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_CORE_CLK_DIV_NUM` reader - Reserved
pub type FLASH_CORE_CLK_DIV_NUM_R = crate::FieldReader;
///Field `FLASH_CORE_CLK_DIV_NUM` writer - Reserved
pub type FLASH_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PSRAM_CLK_SRC_SEL` reader - Reserved
pub type PSRAM_CLK_SRC_SEL_R = crate::FieldReader;
///Field `PSRAM_CLK_SRC_SEL` writer - Reserved
pub type PSRAM_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PSRAM_PLL_CLK_EN` reader - Reserved
pub type PSRAM_PLL_CLK_EN_R = crate::BitReader;
///Field `PSRAM_PLL_CLK_EN` writer - Reserved
pub type PSRAM_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSRAM_CORE_CLK_EN` reader - Reserved
pub type PSRAM_CORE_CLK_EN_R = crate::BitReader;
///Field `PSRAM_CORE_CLK_EN` writer - Reserved
pub type PSRAM_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PSRAM_CORE_CLK_DIV_NUM` reader - Reserved
pub type PSRAM_CORE_CLK_DIV_NUM_R = crate::FieldReader;
///Field `PSRAM_CORE_CLK_DIV_NUM` writer - Reserved
pub type PSRAM_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PAD_EMAC_REF_CLK_EN` reader - Reserved
pub type PAD_EMAC_REF_CLK_EN_R = crate::BitReader;
///Field `PAD_EMAC_REF_CLK_EN` writer - Reserved
pub type PAD_EMAC_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_RMII_CLK_SRC_SEL` reader - Reserved
pub type EMAC_RMII_CLK_SRC_SEL_R = crate::FieldReader;
///Field `EMAC_RMII_CLK_SRC_SEL` writer - Reserved
pub type EMAC_RMII_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `EMAC_RMII_CLK_EN` reader - Reserved
pub type EMAC_RMII_CLK_EN_R = crate::BitReader;
///Field `EMAC_RMII_CLK_EN` writer - Reserved
pub type EMAC_RMII_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_RX_CLK_SRC_SEL` reader - Reserved
pub type EMAC_RX_CLK_SRC_SEL_R = crate::BitReader;
///Field `EMAC_RX_CLK_SRC_SEL` writer - Reserved
pub type EMAC_RX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_RX_CLK_EN` reader - Reserved
pub type EMAC_RX_CLK_EN_R = crate::BitReader;
///Field `EMAC_RX_CLK_EN` writer - Reserved
pub type EMAC_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Reserved
    #[inline(always)]
    pub fn flash_clk_src_sel(&self) -> FLASH_CLK_SRC_SEL_R {
        FLASH_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    pub fn flash_pll_clk_en(&self) -> FLASH_PLL_CLK_EN_R {
        FLASH_PLL_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    pub fn flash_core_clk_en(&self) -> FLASH_CORE_CLK_EN_R {
        FLASH_CORE_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:11 - Reserved
    #[inline(always)]
    pub fn flash_core_clk_div_num(&self) -> FLASH_CORE_CLK_DIV_NUM_R {
        FLASH_CORE_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    ///Bits 12:13 - Reserved
    #[inline(always)]
    pub fn psram_clk_src_sel(&self) -> PSRAM_CLK_SRC_SEL_R {
        PSRAM_CLK_SRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Reserved
    #[inline(always)]
    pub fn psram_pll_clk_en(&self) -> PSRAM_PLL_CLK_EN_R {
        PSRAM_PLL_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Reserved
    #[inline(always)]
    pub fn psram_core_clk_en(&self) -> PSRAM_CORE_CLK_EN_R {
        PSRAM_CORE_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Reserved
    #[inline(always)]
    pub fn psram_core_clk_div_num(&self) -> PSRAM_CORE_CLK_DIV_NUM_R {
        PSRAM_CORE_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - Reserved
    #[inline(always)]
    pub fn pad_emac_ref_clk_en(&self) -> PAD_EMAC_REF_CLK_EN_R {
        PAD_EMAC_REF_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Reserved
    #[inline(always)]
    pub fn emac_rmii_clk_src_sel(&self) -> EMAC_RMII_CLK_SRC_SEL_R {
        EMAC_RMII_CLK_SRC_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Reserved
    #[inline(always)]
    pub fn emac_rmii_clk_en(&self) -> EMAC_RMII_CLK_EN_R {
        EMAC_RMII_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Reserved
    #[inline(always)]
    pub fn emac_rx_clk_src_sel(&self) -> EMAC_RX_CLK_SRC_SEL_R {
        EMAC_RX_CLK_SRC_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Reserved
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
    ///Bits 0:1 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn flash_clk_src_sel(&mut self) -> FLASH_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_CLK_SRC_SEL_W::new(self, 0)
    }
    ///Bit 2 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn flash_pll_clk_en(&mut self) -> FLASH_PLL_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_PLL_CLK_EN_W::new(self, 2)
    }
    ///Bit 3 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn flash_core_clk_en(&mut self) -> FLASH_CORE_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_CORE_CLK_EN_W::new(self, 3)
    }
    ///Bits 4:11 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn flash_core_clk_div_num(&mut self) -> FLASH_CORE_CLK_DIV_NUM_W<PERI_CLK_CTRL00_SPEC> {
        FLASH_CORE_CLK_DIV_NUM_W::new(self, 4)
    }
    ///Bits 12:13 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn psram_clk_src_sel(&mut self) -> PSRAM_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_CLK_SRC_SEL_W::new(self, 12)
    }
    ///Bit 14 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn psram_pll_clk_en(&mut self) -> PSRAM_PLL_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_PLL_CLK_EN_W::new(self, 14)
    }
    ///Bit 15 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn psram_core_clk_en(&mut self) -> PSRAM_CORE_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_CORE_CLK_EN_W::new(self, 15)
    }
    ///Bits 16:23 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn psram_core_clk_div_num(&mut self) -> PSRAM_CORE_CLK_DIV_NUM_W<PERI_CLK_CTRL00_SPEC> {
        PSRAM_CORE_CLK_DIV_NUM_W::new(self, 16)
    }
    ///Bit 24 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn pad_emac_ref_clk_en(&mut self) -> PAD_EMAC_REF_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        PAD_EMAC_REF_CLK_EN_W::new(self, 24)
    }
    ///Bits 25:26 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_rmii_clk_src_sel(&mut self) -> EMAC_RMII_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RMII_CLK_SRC_SEL_W::new(self, 25)
    }
    ///Bit 27 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_rmii_clk_en(&mut self) -> EMAC_RMII_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RMII_CLK_EN_W::new(self, 27)
    }
    ///Bit 28 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_rx_clk_src_sel(&mut self) -> EMAC_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RX_CLK_SRC_SEL_W::new(self, 28)
    }
    ///Bit 29 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_rx_clk_en(&mut self) -> EMAC_RX_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        EMAC_RX_CLK_EN_W::new(self, 29)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL00_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL00_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl00::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL00_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl00::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL00_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL00 to value 0xc03c
impl crate::Resettable for PERI_CLK_CTRL00_SPEC {
    const RESET_VALUE: u32 = 0xc03c;
}
