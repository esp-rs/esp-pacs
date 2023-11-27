#[doc = "Register `PERI_CLK_CTRL00` reader"]
pub type R = crate::R<PERI_CLK_CTRL00_SPEC>;
#[doc = "Register `PERI_CLK_CTRL00` writer"]
pub type W = crate::W<PERI_CLK_CTRL00_SPEC>;
#[doc = "Field `REG_FLASH_CLK_SRC_SEL` reader - Reserved"]
pub type REG_FLASH_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_FLASH_CLK_SRC_SEL` writer - Reserved"]
pub type REG_FLASH_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_FLASH_PLL_CLK_EN` reader - Reserved"]
pub type REG_FLASH_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_FLASH_PLL_CLK_EN` writer - Reserved"]
pub type REG_FLASH_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_FLASH_CORE_CLK_EN` reader - Reserved"]
pub type REG_FLASH_CORE_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_FLASH_CORE_CLK_EN` writer - Reserved"]
pub type REG_FLASH_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_FLASH_CORE_CLK_DIV_NUM` reader - Reserved"]
pub type REG_FLASH_CORE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_FLASH_CORE_CLK_DIV_NUM` writer - Reserved"]
pub type REG_FLASH_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_PSRAM_CLK_SRC_SEL` reader - Reserved"]
pub type REG_PSRAM_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_CLK_SRC_SEL` writer - Reserved"]
pub type REG_PSRAM_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PSRAM_PLL_CLK_EN` reader - Reserved"]
pub type REG_PSRAM_PLL_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_PLL_CLK_EN` writer - Reserved"]
pub type REG_PSRAM_PLL_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_CORE_CLK_EN` reader - Reserved"]
pub type REG_PSRAM_CORE_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_CORE_CLK_EN` writer - Reserved"]
pub type REG_PSRAM_CORE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_CORE_CLK_DIV_NUM` reader - Reserved"]
pub type REG_PSRAM_CORE_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_CORE_CLK_DIV_NUM` writer - Reserved"]
pub type REG_PSRAM_CORE_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_PAD_EMAC_REF_CLK_EN` reader - Reserved"]
pub type REG_PAD_EMAC_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_PAD_EMAC_REF_CLK_EN` writer - Reserved"]
pub type REG_PAD_EMAC_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_EMAC_RMII_CLK_SRC_SEL` reader - Reserved"]
pub type REG_EMAC_RMII_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_EMAC_RMII_CLK_SRC_SEL` writer - Reserved"]
pub type REG_EMAC_RMII_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_EMAC_RMII_CLK_EN` reader - Reserved"]
pub type REG_EMAC_RMII_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_EMAC_RMII_CLK_EN` writer - Reserved"]
pub type REG_EMAC_RMII_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_EMAC_RX_CLK_SRC_SEL` reader - Reserved"]
pub type REG_EMAC_RX_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `REG_EMAC_RX_CLK_SRC_SEL` writer - Reserved"]
pub type REG_EMAC_RX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_EMAC_RX_CLK_EN` reader - Reserved"]
pub type REG_EMAC_RX_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_EMAC_RX_CLK_EN` writer - Reserved"]
pub type REG_EMAC_RX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reg_flash_clk_src_sel(&self) -> REG_FLASH_CLK_SRC_SEL_R {
        REG_FLASH_CLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reg_flash_pll_clk_en(&self) -> REG_FLASH_PLL_CLK_EN_R {
        REG_FLASH_PLL_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reg_flash_core_clk_en(&self) -> REG_FLASH_CORE_CLK_EN_R {
        REG_FLASH_CORE_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    pub fn reg_flash_core_clk_div_num(&self) -> REG_FLASH_CORE_CLK_DIV_NUM_R {
        REG_FLASH_CORE_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_clk_src_sel(&self) -> REG_PSRAM_CLK_SRC_SEL_R {
        REG_PSRAM_CLK_SRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_pll_clk_en(&self) -> REG_PSRAM_PLL_CLK_EN_R {
        REG_PSRAM_PLL_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_core_clk_en(&self) -> REG_PSRAM_CORE_CLK_EN_R {
        REG_PSRAM_CORE_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_core_clk_div_num(&self) -> REG_PSRAM_CORE_CLK_DIV_NUM_R {
        REG_PSRAM_CORE_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reg_pad_emac_ref_clk_en(&self) -> REG_PAD_EMAC_REF_CLK_EN_R {
        REG_PAD_EMAC_REF_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn reg_emac_rmii_clk_src_sel(&self) -> REG_EMAC_RMII_CLK_SRC_SEL_R {
        REG_EMAC_RMII_CLK_SRC_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reg_emac_rmii_clk_en(&self) -> REG_EMAC_RMII_CLK_EN_R {
        REG_EMAC_RMII_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reg_emac_rx_clk_src_sel(&self) -> REG_EMAC_RX_CLK_SRC_SEL_R {
        REG_EMAC_RX_CLK_SRC_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reg_emac_rx_clk_en(&self) -> REG_EMAC_RX_CLK_EN_R {
        REG_EMAC_RX_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL00")
            .field(
                "reg_flash_clk_src_sel",
                &format_args!("{}", self.reg_flash_clk_src_sel().bits()),
            )
            .field(
                "reg_flash_pll_clk_en",
                &format_args!("{}", self.reg_flash_pll_clk_en().bit()),
            )
            .field(
                "reg_flash_core_clk_en",
                &format_args!("{}", self.reg_flash_core_clk_en().bit()),
            )
            .field(
                "reg_flash_core_clk_div_num",
                &format_args!("{}", self.reg_flash_core_clk_div_num().bits()),
            )
            .field(
                "reg_psram_clk_src_sel",
                &format_args!("{}", self.reg_psram_clk_src_sel().bits()),
            )
            .field(
                "reg_psram_pll_clk_en",
                &format_args!("{}", self.reg_psram_pll_clk_en().bit()),
            )
            .field(
                "reg_psram_core_clk_en",
                &format_args!("{}", self.reg_psram_core_clk_en().bit()),
            )
            .field(
                "reg_psram_core_clk_div_num",
                &format_args!("{}", self.reg_psram_core_clk_div_num().bits()),
            )
            .field(
                "reg_pad_emac_ref_clk_en",
                &format_args!("{}", self.reg_pad_emac_ref_clk_en().bit()),
            )
            .field(
                "reg_emac_rmii_clk_src_sel",
                &format_args!("{}", self.reg_emac_rmii_clk_src_sel().bits()),
            )
            .field(
                "reg_emac_rmii_clk_en",
                &format_args!("{}", self.reg_emac_rmii_clk_en().bit()),
            )
            .field(
                "reg_emac_rx_clk_src_sel",
                &format_args!("{}", self.reg_emac_rx_clk_src_sel().bit()),
            )
            .field(
                "reg_emac_rx_clk_en",
                &format_args!("{}", self.reg_emac_rx_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL00_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_flash_clk_src_sel(&mut self) -> REG_FLASH_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        REG_FLASH_CLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_flash_pll_clk_en(&mut self) -> REG_FLASH_PLL_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_FLASH_PLL_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_flash_core_clk_en(&mut self) -> REG_FLASH_CORE_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_FLASH_CORE_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_flash_core_clk_div_num(
        &mut self,
    ) -> REG_FLASH_CORE_CLK_DIV_NUM_W<PERI_CLK_CTRL00_SPEC> {
        REG_FLASH_CORE_CLK_DIV_NUM_W::new(self, 4)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_clk_src_sel(&mut self) -> REG_PSRAM_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        REG_PSRAM_CLK_SRC_SEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_pll_clk_en(&mut self) -> REG_PSRAM_PLL_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_PSRAM_PLL_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_core_clk_en(&mut self) -> REG_PSRAM_CORE_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_PSRAM_CORE_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_psram_core_clk_div_num(
        &mut self,
    ) -> REG_PSRAM_CORE_CLK_DIV_NUM_W<PERI_CLK_CTRL00_SPEC> {
        REG_PSRAM_CORE_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pad_emac_ref_clk_en(&mut self) -> REG_PAD_EMAC_REF_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_PAD_EMAC_REF_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emac_rmii_clk_src_sel(
        &mut self,
    ) -> REG_EMAC_RMII_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        REG_EMAC_RMII_CLK_SRC_SEL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emac_rmii_clk_en(&mut self) -> REG_EMAC_RMII_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_EMAC_RMII_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emac_rx_clk_src_sel(&mut self) -> REG_EMAC_RX_CLK_SRC_SEL_W<PERI_CLK_CTRL00_SPEC> {
        REG_EMAC_RX_CLK_SRC_SEL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_emac_rx_clk_en(&mut self) -> REG_EMAC_RX_CLK_EN_W<PERI_CLK_CTRL00_SPEC> {
        REG_EMAC_RX_CLK_EN_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL00_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl00::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL00_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl00::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL00_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL00 to value 0xc03c"]
impl crate::Resettable for PERI_CLK_CTRL00_SPEC {
    const RESET_VALUE: Self::Ux = 0xc03c;
}
