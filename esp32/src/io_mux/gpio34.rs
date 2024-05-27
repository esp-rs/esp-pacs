#[doc = "Register `GPIO34` reader"]
pub type R = crate::R<GPIO34_SPEC>;
#[doc = "Register `GPIO34` writer"]
pub type W = crate::W<GPIO34_SPEC>;
#[doc = "Field `MCU_OE` reader - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
pub type MCU_OE_R = crate::BitReader;
#[doc = "Field `MCU_OE` writer - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
pub type MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPD` reader - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
pub type MCU_WPD_R = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
pub type MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPU` reader - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled; 0: internal pull-up disabled."]
pub type MCU_WPU_R = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled; 0: internal pull-up disabled."]
pub type MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_IE` reader - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
pub type MCU_IE_R = crate::BitReader;
#[doc = "Field `MCU_IE` writer - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
pub type MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_DRV` reader - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
pub type MCU_DRV_R = crate::FieldReader;
#[doc = "Field `MCU_DRV` writer - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
pub type MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUN_WPD` reader - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
pub type FUN_WPD_R = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
pub type FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_WPU` reader - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
pub type FUN_WPU_R = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
pub type FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - Input enable of the pad. 1: input enabled; 0: input disabled."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable of the pad. 1: input enabled; 0: input disabled."]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_DRV` reader - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
pub type FUN_DRV_R = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
pub type FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU_SEL` reader - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
pub type MCU_SEL_R = crate::FieldReader;
#[doc = "Field `MCU_SEL` writer - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
pub type MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled; 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO34")
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
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Output enable of the pad in sleep mode. 1: enable output; 0: disable output."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<GPIO34_SPEC> {
        MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sleep mode selection of this pad. Set to 1 to put the pad in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<GPIO34_SPEC> {
        SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Pull-down enable of the pad during sleep mode. 1: internal pull-down enabled; 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<GPIO34_SPEC> {
        MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Pull-up enable of the pad during sleep mode. 1: internal pull-up enabled; 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<GPIO34_SPEC> {
        MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4 - Input enable of the pad during sleep mode. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<GPIO34_SPEC> {
        MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Select the drive strength of the pad during sleep mode. A higher value corresponds with a higher strength."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W<GPIO34_SPEC> {
        MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7 - Pull-down enable of the pad. 1: internal pull-down enabled, 0: internal pull-down disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull-down circuitry, therefore, their FUN_WPD is always 0."]
    #[inline(always)]
    #[must_use]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<GPIO34_SPEC> {
        FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - Pull-up enable of the pad. 1: internal pull-up enabled; 0: internal pull-up disabled. GPIO pins 34-39 are input-only. These pins do not feature an output driver or internal pull- up/pull- down circuitry, therefore, their FUN_WPU is always 0."]
    #[inline(always)]
    #[must_use]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<GPIO34_SPEC> {
        FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - Input enable of the pad. 1: input enabled; 0: input disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<GPIO34_SPEC> {
        FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - Select the drive strength of the pad. A higher value corresponds with a higher strength. For GPIO34-39, FUN_DRV is always 0. For detailed drive strength, please see note 8 in Table ”Notes on ESP32 Pin Lists”, in ESP32 Datasheet."]
    #[inline(always)]
    #[must_use]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<GPIO34_SPEC> {
        FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - Select the IO_MUX function for this signal. 0 selects Function 0, 1 selects Function 1, etc."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<GPIO34_SPEC> {
        MCU_SEL_W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO34_SPEC;
impl crate::RegisterSpec for GPIO34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio34::R`](R) reader structure"]
impl crate::Readable for GPIO34_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio34::W`](W) writer structure"]
impl crate::Writable for GPIO34_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO34 to value 0"]
impl crate::Resettable for GPIO34_SPEC {
    const RESET_VALUE: u32 = 0;
}
