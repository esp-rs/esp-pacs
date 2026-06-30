#[doc = "Register `GMAC_PHY_RXDV` reader"]
pub type R = crate::R<GMAC_PHY_RXDV_SPEC>;
#[doc = "Register `GMAC_PHY_RXDV` writer"]
pub type W = crate::W<GMAC_PHY_RXDV_SPEC>;
#[doc = "Field `SLP_SEL` reader - Configures whether or not to enter sleep mode for GMAC_PHY_RXDV.\\\\ 0: Not enter\\\\ 1: Enter\\\\"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - Configures whether or not to enter sleep mode for GMAC_PHY_RXDV.\\\\ 0: Not enter\\\\ 1: Enter\\\\"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPD` reader - Configure whether or not to enable pull-down resistor of GMAC_PHY_RXDV in sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MCU_WPD_R = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - Configure whether or not to enable pull-down resistor of GMAC_PHY_RXDV in sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPU` reader - Configures whether or not to enable pull-up resistor of GMAC_PHY_RXDV during sleep mode. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MCU_WPU_R = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - Configures whether or not to enable pull-up resistor of GMAC_PHY_RXDV during sleep mode. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_IE` reader - Configures whether or not to enable the input of GMAC_PHY_RXDV during sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MCU_IE_R = crate::BitReader;
#[doc = "Field `MCU_IE` writer - Configures whether or not to enable the input of GMAC_PHY_RXDV during sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_DRV` reader - Configures the drive strength of GMAC_PHY_RXDV during sleep mode. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
pub type MCU_DRV_R = crate::FieldReader;
#[doc = "Field `MCU_DRV` writer - Configures the drive strength of GMAC_PHY_RXDV during sleep mode. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
pub type MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUN_WPD` reader - Configures whether or not to enable pull-down resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type FUN_WPD_R = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - Configures whether or not to enable pull-down resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_WPU` reader - Configures whether or not enable pull-up resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type FUN_WPU_R = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - Configures whether or not enable pull-up resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - Configures whether or not to enable input of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Configures whether or not to enable input of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_DRV` reader - Configures the drive strength of GMAC_PHY_RXDV. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
pub type FUN_DRV_R = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - Configures the drive strength of GMAC_PHY_RXDV. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
pub type FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HYS_EN` reader - Configures whether or not to enable the hysteresis function of the pin when CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_SEL is set to 1.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type HYS_EN_R = crate::BitReader;
#[doc = "Field `HYS_EN` writer - Configures whether or not to enable the hysteresis function of the pin when CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_SEL is set to 1.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type HYS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS_SEL` reader - Configures to choose the signal for enabling the hysteresis function for GMAC_PHY_RXDV. \\\\ 0: Choose the output enable signal of eFuse\\\\ 1: Choose the output enable signal of CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_EN\\\\"]
pub type HYS_SEL_R = crate::BitReader;
#[doc = "Field `HYS_SEL` writer - Configures to choose the signal for enabling the hysteresis function for GMAC_PHY_RXDV. \\\\ 0: Choose the output enable signal of eFuse\\\\ 1: Choose the output enable signal of CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_EN\\\\"]
pub type HYS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enter sleep mode for GMAC_PHY_RXDV.\\\\ 0: Not enter\\\\ 1: Enter\\\\"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure whether or not to enable pull-down resistor of GMAC_PHY_RXDV in sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable pull-up resistor of GMAC_PHY_RXDV during sleep mode. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable the input of GMAC_PHY_RXDV during sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Configures the drive strength of GMAC_PHY_RXDV during sleep mode. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Configures whether or not to enable pull-down resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not enable pull-up resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable input of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Configures the drive strength of GMAC_PHY_RXDV. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to enable the hysteresis function of the pin when CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_SEL is set to 1.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn hys_en(&self) -> HYS_EN_R {
        HYS_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Configures to choose the signal for enabling the hysteresis function for GMAC_PHY_RXDV. \\\\ 0: Choose the output enable signal of eFuse\\\\ 1: Choose the output enable signal of CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_EN\\\\"]
    #[inline(always)]
    pub fn hys_sel(&self) -> HYS_SEL_R {
        HYS_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC_PHY_RXDV")
            .field("slp_sel", &self.slp_sel())
            .field("mcu_wpd", &self.mcu_wpd())
            .field("mcu_wpu", &self.mcu_wpu())
            .field("mcu_ie", &self.mcu_ie())
            .field("mcu_drv", &self.mcu_drv())
            .field("fun_wpd", &self.fun_wpd())
            .field("fun_wpu", &self.fun_wpu())
            .field("fun_ie", &self.fun_ie())
            .field("fun_drv", &self.fun_drv())
            .field("hys_en", &self.hys_en())
            .field("hys_sel", &self.hys_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enter sleep mode for GMAC_PHY_RXDV.\\\\ 0: Not enter\\\\ 1: Enter\\\\"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<'_, GMAC_PHY_RXDV_SPEC> {
        SLP_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure whether or not to enable pull-down resistor of GMAC_PHY_RXDV in sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<'_, GMAC_PHY_RXDV_SPEC> {
        MCU_WPD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable pull-up resistor of GMAC_PHY_RXDV during sleep mode. \\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<'_, GMAC_PHY_RXDV_SPEC> {
        MCU_WPU_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable the input of GMAC_PHY_RXDV during sleep mode.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<'_, GMAC_PHY_RXDV_SPEC> {
        MCU_IE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Configures the drive strength of GMAC_PHY_RXDV during sleep mode. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
    #[inline(always)]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W<'_, GMAC_PHY_RXDV_SPEC> {
        MCU_DRV_W::new(self, 4)
    }
    #[doc = "Bit 6 - Configures whether or not to enable pull-down resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<'_, GMAC_PHY_RXDV_SPEC> {
        FUN_WPD_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not enable pull-up resistor of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<'_, GMAC_PHY_RXDV_SPEC> {
        FUN_WPU_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable input of GMAC_PHY_RXDV.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<'_, GMAC_PHY_RXDV_SPEC> {
        FUN_IE_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Configures the drive strength of GMAC_PHY_RXDV. \\\\ 0: ~5 mA\\\\ 1: ~10 mA\\\\ 2: ~20 mA\\\\ 3: ~40 mA\\\\"]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<'_, GMAC_PHY_RXDV_SPEC> {
        FUN_DRV_W::new(self, 9)
    }
    #[doc = "Bit 11 - Configures whether or not to enable the hysteresis function of the pin when CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_SEL is set to 1.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn hys_en(&mut self) -> HYS_EN_W<'_, GMAC_PHY_RXDV_SPEC> {
        HYS_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures to choose the signal for enabling the hysteresis function for GMAC_PHY_RXDV. \\\\ 0: Choose the output enable signal of eFuse\\\\ 1: Choose the output enable signal of CNNT_IO_MUX_GMAC_PHY_RXDV_HYS_EN\\\\"]
    #[inline(always)]
    pub fn hys_sel(&mut self) -> HYS_SEL_W<'_, GMAC_PHY_RXDV_SPEC> {
        HYS_SEL_W::new(self, 12)
    }
}
#[doc = "GMAC IO MUX configuration register for GMAC_PHY_RXDV\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac_phy_rxdv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac_phy_rxdv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC_PHY_RXDV_SPEC;
impl crate::RegisterSpec for GMAC_PHY_RXDV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_phy_rxdv::R`](R) reader structure"]
impl crate::Readable for GMAC_PHY_RXDV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac_phy_rxdv::W`](W) writer structure"]
impl crate::Writable for GMAC_PHY_RXDV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC_PHY_RXDV to value 0x0528"]
impl crate::Resettable for GMAC_PHY_RXDV_SPEC {
    const RESET_VALUE: u32 = 0x0528;
}
