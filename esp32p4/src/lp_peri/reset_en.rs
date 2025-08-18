#[doc = "Register `RESET_EN` reader"]
pub type R = crate::R<RESET_EN_SPEC>;
#[doc = "Register `RESET_EN` writer"]
pub type W = crate::W<RESET_EN_SPEC>;
#[doc = "Field `RST_EN_LP_TSENS` reader - need_des"]
pub type RST_EN_LP_TSENS_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_TSENS` writer - need_des"]
pub type RST_EN_LP_TSENS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_PMS` reader - need_des"]
pub type RST_EN_LP_PMS_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_PMS` writer - need_des"]
pub type RST_EN_LP_PMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_EFUSE` reader - need_des"]
pub type RST_EN_LP_EFUSE_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_EFUSE` writer - need_des"]
pub type RST_EN_LP_EFUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_IOMUX` reader - need_des"]
pub type RST_EN_LP_IOMUX_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_IOMUX` writer - need_des"]
pub type RST_EN_LP_IOMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_TOUCH` reader - need_des"]
pub type RST_EN_LP_TOUCH_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_TOUCH` writer - need_des"]
pub type RST_EN_LP_TOUCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_SPI` reader - need_des"]
pub type RST_EN_LP_SPI_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_SPI` writer - need_des"]
pub type RST_EN_LP_SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_ADC` reader - need_des"]
pub type RST_EN_LP_ADC_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_ADC` writer - need_des"]
pub type RST_EN_LP_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_I2S` reader - need_des"]
pub type RST_EN_LP_I2S_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_I2S` writer - need_des"]
pub type RST_EN_LP_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_I2CMST` reader - need_des"]
pub type RST_EN_LP_I2CMST_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_I2CMST` writer - need_des"]
pub type RST_EN_LP_I2CMST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_I2C` reader - need_des"]
pub type RST_EN_LP_I2C_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_I2C` writer - need_des"]
pub type RST_EN_LP_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_UART` reader - need_des"]
pub type RST_EN_LP_UART_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_UART` writer - need_des"]
pub type RST_EN_LP_UART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_INTR` reader - need_des"]
pub type RST_EN_LP_INTR_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_INTR` writer - need_des"]
pub type RST_EN_LP_INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_ROM` reader - need_des"]
pub type RST_EN_LP_ROM_R = crate::BitReader;
#[doc = "Field `RST_EN_LP_ROM` writer - need_des"]
pub type RST_EN_LP_ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN_LP_CORE` writer - need_des"]
pub type RST_EN_LP_CORE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_tsens(&self) -> RST_EN_LP_TSENS_R {
        RST_EN_LP_TSENS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_pms(&self) -> RST_EN_LP_PMS_R {
        RST_EN_LP_PMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_efuse(&self) -> RST_EN_LP_EFUSE_R {
        RST_EN_LP_EFUSE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_iomux(&self) -> RST_EN_LP_IOMUX_R {
        RST_EN_LP_IOMUX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_touch(&self) -> RST_EN_LP_TOUCH_R {
        RST_EN_LP_TOUCH_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_spi(&self) -> RST_EN_LP_SPI_R {
        RST_EN_LP_SPI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_adc(&self) -> RST_EN_LP_ADC_R {
        RST_EN_LP_ADC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_i2s(&self) -> RST_EN_LP_I2S_R {
        RST_EN_LP_I2S_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_i2cmst(&self) -> RST_EN_LP_I2CMST_R {
        RST_EN_LP_I2CMST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_i2c(&self) -> RST_EN_LP_I2C_R {
        RST_EN_LP_I2C_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_uart(&self) -> RST_EN_LP_UART_R {
        RST_EN_LP_UART_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_intr(&self) -> RST_EN_LP_INTR_R {
        RST_EN_LP_INTR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_rom(&self) -> RST_EN_LP_ROM_R {
        RST_EN_LP_ROM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_EN")
            .field("rst_en_lp_tsens", &self.rst_en_lp_tsens())
            .field("rst_en_lp_pms", &self.rst_en_lp_pms())
            .field("rst_en_lp_efuse", &self.rst_en_lp_efuse())
            .field("rst_en_lp_iomux", &self.rst_en_lp_iomux())
            .field("rst_en_lp_touch", &self.rst_en_lp_touch())
            .field("rst_en_lp_spi", &self.rst_en_lp_spi())
            .field("rst_en_lp_adc", &self.rst_en_lp_adc())
            .field("rst_en_lp_i2s", &self.rst_en_lp_i2s())
            .field("rst_en_lp_i2cmst", &self.rst_en_lp_i2cmst())
            .field("rst_en_lp_i2c", &self.rst_en_lp_i2c())
            .field("rst_en_lp_uart", &self.rst_en_lp_uart())
            .field("rst_en_lp_intr", &self.rst_en_lp_intr())
            .field("rst_en_lp_rom", &self.rst_en_lp_rom())
            .finish()
    }
}
impl W {
    #[doc = "Bit 18 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_tsens(&mut self) -> RST_EN_LP_TSENS_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_TSENS_W::new(self, 18)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_pms(&mut self) -> RST_EN_LP_PMS_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_PMS_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_efuse(&mut self) -> RST_EN_LP_EFUSE_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_EFUSE_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_iomux(&mut self) -> RST_EN_LP_IOMUX_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_IOMUX_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_touch(&mut self) -> RST_EN_LP_TOUCH_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_TOUCH_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_spi(&mut self) -> RST_EN_LP_SPI_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_SPI_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_adc(&mut self) -> RST_EN_LP_ADC_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_ADC_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_i2s(&mut self) -> RST_EN_LP_I2S_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_I2S_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_i2cmst(&mut self) -> RST_EN_LP_I2CMST_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_I2CMST_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_i2c(&mut self) -> RST_EN_LP_I2C_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_I2C_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_uart(&mut self) -> RST_EN_LP_UART_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_UART_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_intr(&mut self) -> RST_EN_LP_INTR_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_INTR_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_rom(&mut self) -> RST_EN_LP_ROM_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_ROM_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn rst_en_lp_core(&mut self) -> RST_EN_LP_CORE_W<'_, RESET_EN_SPEC> {
        RST_EN_LP_CORE_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_EN_SPEC;
impl crate::RegisterSpec for RESET_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_en::R`](R) reader structure"]
impl crate::Readable for RESET_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_en::W`](W) writer structure"]
impl crate::Writable for RESET_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_EN to value 0"]
impl crate::Resettable for RESET_EN_SPEC {}
