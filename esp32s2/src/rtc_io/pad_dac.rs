#[doc = "Register `PAD_DAC%s` reader"]
pub type R = crate::R<PAD_DAC_SPEC>;
#[doc = "Register `PAD_DAC%s` writer"]
pub type W = crate::W<PAD_DAC_SPEC>;
#[doc = "Field `DAC` reader - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
pub type DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `XPD_DAC` reader - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
pub type XPD_DAC_R = crate::BitReader;
#[doc = "Field `XPD_DAC` writer - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
pub type XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_XPD_FORCE` reader - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
pub type DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `DAC_XPD_FORCE` writer - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
pub type DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - Input enable in normal execution."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable in normal execution."]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - Output enable in sleep mode"]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - Output enable in sleep mode"]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - Input enable in sleep mode"]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - Input enable in sleep mode"]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode. 0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode. 0: no sleep mode"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - DAC_1 function selection."]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - DAC_1 function selection."]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO. 0: use digital GPIO"]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO. 0: use digital GPIO"]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 3:10 - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
    #[inline(always)]
    pub fn xpd_dac(&self) -> XPD_DAC_R {
        XPD_DAC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
    #[inline(always)]
    pub fn dac_xpd_force(&self) -> DAC_XPD_FORCE_R {
        DAC_XPD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DAC_1 function selection."]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC")
            .field("dac", &self.dac())
            .field("xpd_dac", &self.xpd_dac())
            .field("dac_xpd_force", &self.dac_xpd_force())
            .field("fun_ie", &self.fun_ie())
            .field("slp_oe", &self.slp_oe())
            .field("slp_ie", &self.slp_ie())
            .field("slp_sel", &self.slp_sel())
            .field("fun_sel", &self.fun_sel())
            .field("mux_sel", &self.mux_sel())
            .field("rue", &self.rue())
            .field("rde", &self.rde())
            .field("drv", &self.drv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 3:10 - Configure DAC_1 output when RTCIO_PDAC1_DAC_XPD_FORCE is set to 1."]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<PAD_DAC_SPEC> {
        DAC_W::new(self, 3)
    }
    #[doc = "Bit 11 - When RTCIO_PDAC1_DAC_XPD_FORCE is set to 1, 1: enable DAC_1 output. 0: disable DAC_1 output."]
    #[inline(always)]
    pub fn xpd_dac(&mut self) -> XPD_DAC_W<PAD_DAC_SPEC> {
        XPD_DAC_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: use RTCIO_PDAC1_XPD_DAC to control DAC_1 output. 0: use SAR ADC FSM to control DAC_1 output."]
    #[inline(always)]
    pub fn dac_xpd_force(&mut self) -> DAC_XPD_FORCE_W<PAD_DAC_SPEC> {
        DAC_XPD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<PAD_DAC_SPEC> {
        FUN_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Output enable in sleep mode"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W<PAD_DAC_SPEC> {
        SLP_OE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Input enable in sleep mode"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W<PAD_DAC_SPEC> {
        SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<PAD_DAC_SPEC> {
        SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DAC_1 function selection."]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<PAD_DAC_SPEC> {
        FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<PAD_DAC_SPEC> {
        MUX_SEL_W::new(self, 19)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<PAD_DAC_SPEC> {
        RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<PAD_DAC_SPEC> {
        RDE_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<PAD_DAC_SPEC> {
        DRV_W::new(self, 29)
    }
}
#[doc = "DAC%s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_DAC_SPEC;
impl crate::RegisterSpec for PAD_DAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_dac::R`](R) reader structure"]
impl crate::Readable for PAD_DAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_dac::W`](W) writer structure"]
impl crate::Writable for PAD_DAC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_DAC%s to value 0x4000_0000"]
impl crate::Resettable for PAD_DAC_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
