///Register `GPIO%s` reader
pub type R = crate::R<GPIO_SPEC>;
///Register `GPIO%s` writer
pub type W = crate::W<GPIO_SPEC>;
///Field `MCU_OE` reader - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable
pub type MCU_OE_R = crate::BitReader;
///Field `MCU_OE` writer - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable
pub type MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLP_SEL` reader - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter
pub type SLP_SEL_R = crate::BitReader;
///Field `SLP_SEL` writer - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_WPD` reader - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable
pub type MCU_WPD_R = crate::BitReader;
///Field `MCU_WPD` writer - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable
pub type MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_WPU` reader - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable
pub type MCU_WPU_R = crate::BitReader;
///Field `MCU_WPU` writer - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable
pub type MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_IE` reader - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable
pub type MCU_IE_R = crate::BitReader;
///Field `MCU_IE` writer - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable
pub type MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MCU_DRV` reader - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
pub type MCU_DRV_R = crate::FieldReader;
///Field `MCU_DRV` writer - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
pub type MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `FUN_WPD` reader - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable
pub type FUN_WPD_R = crate::BitReader;
///Field `FUN_WPD` writer - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable
pub type FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_WPU` reader - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable
pub type FUN_WPU_R = crate::BitReader;
///Field `FUN_WPU` writer - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable
pub type FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_IE` reader - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable
pub type FUN_IE_R = crate::BitReader;
///Field `FUN_IE` writer - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FUN_DRV` reader - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
pub type FUN_DRV_R = crate::FieldReader;
///Field `FUN_DRV` writer - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
pub type FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `MCU_SEL` reader - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......
pub type MCU_SEL_R = crate::FieldReader;
///Field `MCU_SEL` writer - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......
pub type MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `FILTER_EN` reader - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable
pub type FILTER_EN_R = crate::BitReader;
///Field `FILTER_EN` writer - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:14 - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO")
            .field("mcu_oe", &self.mcu_oe())
            .field("slp_sel", &self.slp_sel())
            .field("mcu_wpd", &self.mcu_wpd())
            .field("mcu_wpu", &self.mcu_wpu())
            .field("mcu_ie", &self.mcu_ie())
            .field("mcu_drv", &self.mcu_drv())
            .field("fun_wpd", &self.fun_wpd())
            .field("fun_wpu", &self.fun_wpu())
            .field("fun_ie", &self.fun_ie())
            .field("fun_drv", &self.fun_drv())
            .field("mcu_sel", &self.mcu_sel())
            .field("filter_en", &self.filter_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<GPIO_SPEC> {
        MCU_OE_W::new(self, 0)
    }
    ///Bit 1 - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<GPIO_SPEC> {
        SLP_SEL_W::new(self, 1)
    }
    ///Bit 2 - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<GPIO_SPEC> {
        MCU_WPD_W::new(self, 2)
    }
    ///Bit 3 - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<GPIO_SPEC> {
        MCU_WPU_W::new(self, 3)
    }
    ///Bit 4 - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<GPIO_SPEC> {
        MCU_IE_W::new(self, 4)
    }
    ///Bits 5:6 - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
    #[inline(always)]
    #[must_use]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W<GPIO_SPEC> {
        MCU_DRV_W::new(self, 5)
    }
    ///Bit 7 - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<GPIO_SPEC> {
        FUN_WPD_W::new(self, 7)
    }
    ///Bit 8 - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<GPIO_SPEC> {
        FUN_WPU_W::new(self, 8)
    }
    ///Bit 9 - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<GPIO_SPEC> {
        FUN_IE_W::new(self, 9)
    }
    ///Bits 10:11 - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA
    #[inline(always)]
    #[must_use]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<GPIO_SPEC> {
        FUN_DRV_W::new(self, 10)
    }
    ///Bits 12:14 - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......
    #[inline(always)]
    #[must_use]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<GPIO_SPEC> {
        MCU_SEL_W::new(self, 12)
    }
    ///Bit 15 - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<GPIO_SPEC> {
        FILTER_EN_W::new(self, 15)
    }
}
/**IO_MUX Control Register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO_SPEC;
impl crate::RegisterSpec for GPIO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio::R`](R) reader structure
impl crate::Readable for GPIO_SPEC {}
///`write(|w| ..)` method takes [`gpio::W`](W) writer structure
impl crate::Writable for GPIO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO%s to value 0
impl crate::Resettable for GPIO_SPEC {
    const RESET_VALUE: u32 = 0;
}
