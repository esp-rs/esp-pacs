#[doc = "Register `PAD_DAC%s` reader"]
pub type R = crate::R<PAD_DAC_SPEC>;
#[doc = "Register `PAD_DAC%s` writer"]
pub type W = crate::W<PAD_DAC_SPEC>;
#[doc = "Field `DAC_XPD_FORCE` reader - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `DAC_XPD_FORCE` writer - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - the input enable of the pad"]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - the input enable of the pad"]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_OE` reader - the output enable of the pad in sleep status"]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - the output enable of the pad in sleep status"]
pub type SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_SEL` reader - the functional selection signal of the pad"]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - the functional selection signal of the pad"]
pub type FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_DAC` reader - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type XPD_DAC_R = crate::BitReader;
#[doc = "Field `XPD_DAC` writer - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - PAD DAC1 control code."]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - PAD DAC1 control code."]
pub type DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RUE` reader - the pull up enable of the pad"]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - the pull up enable of the pad"]
pub type RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDE` reader - the pull down enable of the pad"]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - the pull down enable of the pad"]
pub type RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type HOLD_R = crate::BitReader;
#[doc = "Field `HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - the driver strength of the pad"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - the driver strength of the pad"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn dac_xpd_force(&self) -> DAC_XPD_FORCE_R {
        DAC_XPD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn mux_sel(&self) -> MUX_SEL_R {
        MUX_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn xpd_dac(&self) -> XPD_DAC_R {
        XPD_DAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - PAD DAC1 control code."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rue(&self) -> RUE_R {
        RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rde(&self) -> RDE_R {
        RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC")
            .field("dac_xpd_force", &self.dac_xpd_force())
            .field("fun_ie", &self.fun_ie())
            .field("slp_oe", &self.slp_oe())
            .field("slp_ie", &self.slp_ie())
            .field("slp_sel", &self.slp_sel())
            .field("fun_sel", &self.fun_sel())
            .field("mux_sel", &self.mux_sel())
            .field("xpd_dac", &self.xpd_dac())
            .field("dac", &self.dac())
            .field("rue", &self.rue())
            .field("rde", &self.rde())
            .field("hold", &self.hold())
            .field("drv", &self.drv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn dac_xpd_force(&mut self) -> DAC_XPD_FORCE_W<'_, PAD_DAC_SPEC> {
        DAC_XPD_FORCE_W::new(self, 10)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<'_, PAD_DAC_SPEC> {
        FUN_IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_oe(&mut self) -> SLP_OE_W<'_, PAD_DAC_SPEC> {
        SLP_OE_W::new(self, 12)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn slp_ie(&mut self) -> SLP_IE_W<'_, PAD_DAC_SPEC> {
        SLP_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<'_, PAD_DAC_SPEC> {
        SLP_SEL_W::new(self, 14)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<'_, PAD_DAC_SPEC> {
        FUN_SEL_W::new(self, 15)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<'_, PAD_DAC_SPEC> {
        MUX_SEL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Power on DAC1. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn xpd_dac(&mut self) -> XPD_DAC_W<'_, PAD_DAC_SPEC> {
        XPD_DAC_W::new(self, 18)
    }
    #[doc = "Bits 19:26 - PAD DAC1 control code."]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<'_, PAD_DAC_SPEC> {
        DAC_W::new(self, 19)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn rue(&mut self) -> RUE_W<'_, PAD_DAC_SPEC> {
        RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn rde(&mut self) -> RDE_W<'_, PAD_DAC_SPEC> {
        RDE_W::new(self, 28)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W<'_, PAD_DAC_SPEC> {
        HOLD_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<'_, PAD_DAC_SPEC> {
        DRV_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets PAD_DAC%s to value 0x8000_0000"]
impl crate::Resettable for PAD_DAC_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
