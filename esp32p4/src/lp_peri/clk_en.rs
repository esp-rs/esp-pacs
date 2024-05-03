#[doc = "Register `CLK_EN` reader"]
pub type R = crate::R<CLK_EN_SPEC>;
#[doc = "Register `CLK_EN` writer"]
pub type W = crate::W<CLK_EN_SPEC>;
#[doc = "Field `CK_EN_RNG` reader - need_des"]
pub type CK_EN_RNG_R = crate::BitReader;
#[doc = "Field `CK_EN_RNG` writer - need_des"]
pub type CK_EN_RNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_TSENS` reader - need_des"]
pub type CK_EN_LP_TSENS_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_TSENS` writer - need_des"]
pub type CK_EN_LP_TSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_PMS` reader - need_des"]
pub type CK_EN_LP_PMS_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_PMS` writer - need_des"]
pub type CK_EN_LP_PMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_EFUSE` reader - need_des"]
pub type CK_EN_LP_EFUSE_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_EFUSE` writer - need_des"]
pub type CK_EN_LP_EFUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_IOMUX` reader - need_des"]
pub type CK_EN_LP_IOMUX_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_IOMUX` writer - need_des"]
pub type CK_EN_LP_IOMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_TOUCH` reader - need_des"]
pub type CK_EN_LP_TOUCH_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_TOUCH` writer - need_des"]
pub type CK_EN_LP_TOUCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_SPI` reader - need_des"]
pub type CK_EN_LP_SPI_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_SPI` writer - need_des"]
pub type CK_EN_LP_SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_ADC` reader - need_des"]
pub type CK_EN_LP_ADC_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_ADC` writer - need_des"]
pub type CK_EN_LP_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2S_TX` reader - need_des"]
pub type CK_EN_LP_I2S_TX_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2S_TX` writer - need_des"]
pub type CK_EN_LP_I2S_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2S_RX` reader - need_des"]
pub type CK_EN_LP_I2S_RX_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2S_RX` writer - need_des"]
pub type CK_EN_LP_I2S_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2S` reader - need_des"]
pub type CK_EN_LP_I2S_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2S` writer - need_des"]
pub type CK_EN_LP_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2CMST` reader - need_des"]
pub type CK_EN_LP_I2CMST_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2CMST` writer - need_des"]
pub type CK_EN_LP_I2CMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_I2C` reader - need_des"]
pub type CK_EN_LP_I2C_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_I2C` writer - need_des"]
pub type CK_EN_LP_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_UART` reader - need_des"]
pub type CK_EN_LP_UART_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_UART` writer - need_des"]
pub type CK_EN_LP_UART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_INTR` reader - need_des"]
pub type CK_EN_LP_INTR_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_INTR` writer - need_des"]
pub type CK_EN_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_EN_LP_CORE` reader - write 1 to force on lp_core clk"]
pub type CK_EN_LP_CORE_R = crate::BitReader;
#[doc = "Field `CK_EN_LP_CORE` writer - write 1 to force on lp_core clk"]
pub type CK_EN_LP_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn ck_en_rng(&self) -> CK_EN_RNG_R {
        CK_EN_RNG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_tsens(&self) -> CK_EN_LP_TSENS_R {
        CK_EN_LP_TSENS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_pms(&self) -> CK_EN_LP_PMS_R {
        CK_EN_LP_PMS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_efuse(&self) -> CK_EN_LP_EFUSE_R {
        CK_EN_LP_EFUSE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_iomux(&self) -> CK_EN_LP_IOMUX_R {
        CK_EN_LP_IOMUX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_touch(&self) -> CK_EN_LP_TOUCH_R {
        CK_EN_LP_TOUCH_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_spi(&self) -> CK_EN_LP_SPI_R {
        CK_EN_LP_SPI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_adc(&self) -> CK_EN_LP_ADC_R {
        CK_EN_LP_ADC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s_tx(&self) -> CK_EN_LP_I2S_TX_R {
        CK_EN_LP_I2S_TX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s_rx(&self) -> CK_EN_LP_I2S_RX_R {
        CK_EN_LP_I2S_RX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2s(&self) -> CK_EN_LP_I2S_R {
        CK_EN_LP_I2S_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2cmst(&self) -> CK_EN_LP_I2CMST_R {
        CK_EN_LP_I2CMST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_i2c(&self) -> CK_EN_LP_I2C_R {
        CK_EN_LP_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_uart(&self) -> CK_EN_LP_UART_R {
        CK_EN_LP_UART_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn ck_en_lp_intr(&self) -> CK_EN_LP_INTR_R {
        CK_EN_LP_INTR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to force on lp_core clk"]
    #[inline(always)]
    pub fn ck_en_lp_core(&self) -> CK_EN_LP_CORE_R {
        CK_EN_LP_CORE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN")
            .field("ck_en_rng", &self.ck_en_rng().bit())
            .field("ck_en_lp_tsens", &self.ck_en_lp_tsens().bit())
            .field("ck_en_lp_pms", &self.ck_en_lp_pms().bit())
            .field("ck_en_lp_efuse", &self.ck_en_lp_efuse().bit())
            .field("ck_en_lp_iomux", &self.ck_en_lp_iomux().bit())
            .field("ck_en_lp_touch", &self.ck_en_lp_touch().bit())
            .field("ck_en_lp_spi", &self.ck_en_lp_spi().bit())
            .field("ck_en_lp_adc", &self.ck_en_lp_adc().bit())
            .field("ck_en_lp_i2s_tx", &self.ck_en_lp_i2s_tx().bit())
            .field("ck_en_lp_i2s_rx", &self.ck_en_lp_i2s_rx().bit())
            .field("ck_en_lp_i2s", &self.ck_en_lp_i2s().bit())
            .field("ck_en_lp_i2cmst", &self.ck_en_lp_i2cmst().bit())
            .field("ck_en_lp_i2c", &self.ck_en_lp_i2c().bit())
            .field("ck_en_lp_uart", &self.ck_en_lp_uart().bit())
            .field("ck_en_lp_intr", &self.ck_en_lp_intr().bit())
            .field("ck_en_lp_core", &self.ck_en_lp_core().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_rng(&mut self) -> CK_EN_RNG_W<CLK_EN_SPEC> {
        CK_EN_RNG_W::new(self, 16)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_tsens(&mut self) -> CK_EN_LP_TSENS_W<CLK_EN_SPEC> {
        CK_EN_LP_TSENS_W::new(self, 17)
    }
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_pms(&mut self) -> CK_EN_LP_PMS_W<CLK_EN_SPEC> {
        CK_EN_LP_PMS_W::new(self, 18)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_efuse(&mut self) -> CK_EN_LP_EFUSE_W<CLK_EN_SPEC> {
        CK_EN_LP_EFUSE_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_iomux(&mut self) -> CK_EN_LP_IOMUX_W<CLK_EN_SPEC> {
        CK_EN_LP_IOMUX_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_touch(&mut self) -> CK_EN_LP_TOUCH_W<CLK_EN_SPEC> {
        CK_EN_LP_TOUCH_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_spi(&mut self) -> CK_EN_LP_SPI_W<CLK_EN_SPEC> {
        CK_EN_LP_SPI_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_adc(&mut self) -> CK_EN_LP_ADC_W<CLK_EN_SPEC> {
        CK_EN_LP_ADC_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_i2s_tx(&mut self) -> CK_EN_LP_I2S_TX_W<CLK_EN_SPEC> {
        CK_EN_LP_I2S_TX_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_i2s_rx(&mut self) -> CK_EN_LP_I2S_RX_W<CLK_EN_SPEC> {
        CK_EN_LP_I2S_RX_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_i2s(&mut self) -> CK_EN_LP_I2S_W<CLK_EN_SPEC> {
        CK_EN_LP_I2S_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_i2cmst(&mut self) -> CK_EN_LP_I2CMST_W<CLK_EN_SPEC> {
        CK_EN_LP_I2CMST_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_i2c(&mut self) -> CK_EN_LP_I2C_W<CLK_EN_SPEC> {
        CK_EN_LP_I2C_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_uart(&mut self) -> CK_EN_LP_UART_W<CLK_EN_SPEC> {
        CK_EN_LP_UART_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_intr(&mut self) -> CK_EN_LP_INTR_W<CLK_EN_SPEC> {
        CK_EN_LP_INTR_W::new(self, 30)
    }
    #[doc = "Bit 31 - write 1 to force on lp_core clk"]
    #[inline(always)]
    #[must_use]
    pub fn ck_en_lp_core(&mut self) -> CK_EN_LP_CORE_W<CLK_EN_SPEC> {
        CK_EN_LP_CORE_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN_SPEC;
impl crate::RegisterSpec for CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en::R`](R) reader structure"]
impl crate::Readable for CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en::W`](W) writer structure"]
impl crate::Writable for CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_EN to value 0x7fff_0000"]
impl crate::Resettable for CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x7fff_0000;
}
