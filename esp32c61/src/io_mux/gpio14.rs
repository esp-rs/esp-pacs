#[doc = "Register `GPIO14` reader"]
pub type R = crate::R<GPIO14_SPEC>;
#[doc = "Register `GPIO14` writer"]
pub type W = crate::W<GPIO14_SPEC>;
#[doc = "Field `MCU_OE` reader - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
pub type MCU_OE_R = crate::BitReader;
#[doc = "Field `MCU_OE` writer - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
pub type MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPD` reader - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type MCU_WPD_R = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPU` reader - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type MCU_WPU_R = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_IE` reader - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
pub type MCU_IE_R = crate::BitReader;
#[doc = "Field `MCU_IE` writer - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
pub type MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_DRV` reader - Select the drive strength of the pad during sleep mode."]
pub type MCU_DRV_R = crate::FieldReader;
#[doc = "Field `MCU_DRV` writer - Select the drive strength of the pad during sleep mode."]
pub type MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUN_WPD` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type FUN_WPD_R = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_WPU` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type FUN_WPU_R = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - Input enable of the pad. 1: input enabled. 0: input disabled."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable of the pad. 1: input enabled. 0: input disabled."]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_DRV` reader - Select the drive strength of the pad."]
pub type FUN_DRV_R = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - Select the drive strength of the pad."]
pub type FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU_SEL` reader - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
pub type MCU_SEL_R = crate::FieldReader;
#[doc = "Field `MCU_SEL` writer - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
pub type MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER_EN` reader - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS_EN` reader - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
pub type HYS_EN_R = crate::BitReader;
#[doc = "Field `HYS_EN` writer - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
pub type HYS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS_SEL` reader - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
pub type HYS_SEL_R = crate::BitReader;
#[doc = "Field `HYS_SEL` writer - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
pub type HYS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode."]
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad."]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
    #[inline(always)]
    pub fn hys_en(&self) -> HYS_EN_R {
        HYS_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
    #[inline(always)]
    pub fn hys_sel(&self) -> HYS_SEL_R {
        HYS_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO14")
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
            .field("hys_en", &self.hys_en())
            .field("hys_sel", &self.hys_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: output enabled. 0: output disabled."]
    #[inline(always)]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<'_, GPIO14_SPEC> {
        MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in pad mode."]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<'_, GPIO14_SPEC> {
        SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad in sleep mode. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<'_, GPIO14_SPEC> {
        MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<'_, GPIO14_SPEC> {
        MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<'_, GPIO14_SPEC> {
        MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode."]
    #[inline(always)]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W<'_, GPIO14_SPEC> {
        MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<'_, GPIO14_SPEC> {
        FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<'_, GPIO14_SPEC> {
        FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled. 0: input disabled."]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<'_, GPIO14_SPEC> {
        FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad."]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<'_, GPIO14_SPEC> {
        FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - Select IO MUX function for this signal. 0: Select Function 1. 1: Select Function 2. etc."]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<'_, GPIO14_SPEC> {
        MCU_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - Enable filter for pin input signals. 1: Filter enabled. 0: Filter disabled."]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W<'_, GPIO14_SPEC> {
        FILTER_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Software enables hysteresis function for the pad. 1: Hysteresis enabled. 0: Hysteresis disabled."]
    #[inline(always)]
    pub fn hys_en(&mut self) -> HYS_EN_W<'_, GPIO14_SPEC> {
        HYS_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Select enabling signals of the pad from software and efuse hardware. 1: Select enabling siganl from slftware. 0: Select enabling signal from efuse hardware."]
    #[inline(always)]
    pub fn hys_sel(&mut self) -> HYS_SEL_W<'_, GPIO14_SPEC> {
        HYS_SEL_W::new(self, 17)
    }
}
#[doc = "IO_MUX Configure Register for pad SPICS1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO14_SPEC;
impl crate::RegisterSpec for GPIO14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio14::R`](R) reader structure"]
impl crate::Readable for GPIO14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio14::W`](W) writer structure"]
impl crate::Writable for GPIO14_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO14 to value 0x1b00"]
impl crate::Resettable for GPIO14_SPEC {
    const RESET_VALUE: u32 = 0x1b00;
}
