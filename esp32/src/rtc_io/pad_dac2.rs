#[doc = "Register `PAD_DAC2` reader"]
pub type R = crate::R<PAD_DAC2_SPEC>;
#[doc = "Register `PAD_DAC2` writer"]
pub type W = crate::W<PAD_DAC2_SPEC>;
#[doc = "Field `PDAC2_DAC_XPD_FORCE` reader - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_DAC_XPD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC2_DAC_XPD_FORCE` writer - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_DAC_XPD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_FUN_IE` reader - the input enable of the pad"]
pub type PDAC2_FUN_IE_R = crate::BitReader;
#[doc = "Field `PDAC2_FUN_IE` writer - the input enable of the pad"]
pub type PDAC2_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_SLP_OE` reader - the output enable of the pad in sleep status"]
pub type PDAC2_SLP_OE_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_OE` writer - the output enable of the pad in sleep status"]
pub type PDAC2_SLP_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type PDAC2_SLP_IE_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type PDAC2_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type PDAC2_SLP_SEL_R = crate::BitReader;
#[doc = "Field `PDAC2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type PDAC2_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_FUN_SEL` reader - the functional selection signal of the pad"]
pub type PDAC2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `PDAC2_FUN_SEL` writer - the functional selection signal of the pad"]
pub type PDAC2_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDAC2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type PDAC2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `PDAC2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type PDAC2_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_XPD_DAC` reader - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_XPD_DAC_R = crate::BitReader;
#[doc = "Field `PDAC2_XPD_DAC` writer - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
pub type PDAC2_XPD_DAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_DAC` reader - PAD DAC2 control code."]
pub type PDAC2_DAC_R = crate::FieldReader;
#[doc = "Field `PDAC2_DAC` writer - PAD DAC2 control code."]
pub type PDAC2_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PDAC2_RUE` reader - the pull up enable of the pad"]
pub type PDAC2_RUE_R = crate::BitReader;
#[doc = "Field `PDAC2_RUE` writer - the pull up enable of the pad"]
pub type PDAC2_RUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_RDE` reader - the pull down enable of the pad"]
pub type PDAC2_RDE_R = crate::BitReader;
#[doc = "Field `PDAC2_RDE` writer - the pull down enable of the pad"]
pub type PDAC2_RDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type PDAC2_HOLD_R = crate::BitReader;
#[doc = "Field `PDAC2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type PDAC2_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_DRV` reader - the driver strength of the pad"]
pub type PDAC2_DRV_R = crate::FieldReader;
#[doc = "Field `PDAC2_DRV` writer - the driver strength of the pad"]
pub type PDAC2_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 10 - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_dac_xpd_force(&self) -> PDAC2_DAC_XPD_FORCE_R {
        PDAC2_DAC_XPD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_ie(&self) -> PDAC2_FUN_IE_R {
        PDAC2_FUN_IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_oe(&self) -> PDAC2_SLP_OE_R {
        PDAC2_SLP_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn pdac2_slp_ie(&self) -> PDAC2_SLP_IE_R {
        PDAC2_SLP_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_slp_sel(&self) -> PDAC2_SLP_SEL_R {
        PDAC2_SLP_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn pdac2_fun_sel(&self) -> PDAC2_FUN_SEL_R {
        PDAC2_FUN_SEL_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn pdac2_mux_sel(&self) -> PDAC2_MUX_SEL_R {
        PDAC2_MUX_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    pub fn pdac2_xpd_dac(&self) -> PDAC2_XPD_DAC_R {
        PDAC2_XPD_DAC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - PAD DAC2 control code."]
    #[inline(always)]
    pub fn pdac2_dac(&self) -> PDAC2_DAC_R {
        PDAC2_DAC_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rue(&self) -> PDAC2_RUE_R {
        PDAC2_RUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    pub fn pdac2_rde(&self) -> PDAC2_RDE_R {
        PDAC2_RDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn pdac2_hold(&self) -> PDAC2_HOLD_R {
        PDAC2_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    pub fn pdac2_drv(&self) -> PDAC2_DRV_R {
        PDAC2_DRV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_DAC2")
            .field("pdac2_dac_xpd_force", &self.pdac2_dac_xpd_force())
            .field("pdac2_fun_ie", &self.pdac2_fun_ie())
            .field("pdac2_slp_oe", &self.pdac2_slp_oe())
            .field("pdac2_slp_ie", &self.pdac2_slp_ie())
            .field("pdac2_slp_sel", &self.pdac2_slp_sel())
            .field("pdac2_fun_sel", &self.pdac2_fun_sel())
            .field("pdac2_mux_sel", &self.pdac2_mux_sel())
            .field("pdac2_xpd_dac", &self.pdac2_xpd_dac())
            .field("pdac2_dac", &self.pdac2_dac())
            .field("pdac2_rue", &self.pdac2_rue())
            .field("pdac2_rde", &self.pdac2_rde())
            .field("pdac2_hold", &self.pdac2_hold())
            .field("pdac2_drv", &self.pdac2_drv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - Power on DAC2. Usually we need to tristate PDAC2 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_dac_xpd_force(&mut self) -> PDAC2_DAC_XPD_FORCE_W<PAD_DAC2_SPEC> {
        PDAC2_DAC_XPD_FORCE_W::new(self, 10)
    }
    #[doc = "Bit 11 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_fun_ie(&mut self) -> PDAC2_FUN_IE_W<PAD_DAC2_SPEC> {
        PDAC2_FUN_IE_W::new(self, 11)
    }
    #[doc = "Bit 12 - the output enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_slp_oe(&mut self) -> PDAC2_SLP_OE_W<PAD_DAC2_SPEC> {
        PDAC2_SLP_OE_W::new(self, 12)
    }
    #[doc = "Bit 13 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_slp_ie(&mut self) -> PDAC2_SLP_IE_W<PAD_DAC2_SPEC> {
        PDAC2_SLP_IE_W::new(self, 13)
    }
    #[doc = "Bit 14 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_slp_sel(&mut self) -> PDAC2_SLP_SEL_W<PAD_DAC2_SPEC> {
        PDAC2_SLP_SEL_W::new(self, 14)
    }
    #[doc = "Bits 15:16 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_fun_sel(&mut self) -> PDAC2_FUN_SEL_W<PAD_DAC2_SPEC> {
        PDAC2_FUN_SEL_W::new(self, 15)
    }
    #[doc = "Bit 17 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_mux_sel(&mut self) -> PDAC2_MUX_SEL_W<PAD_DAC2_SPEC> {
        PDAC2_MUX_SEL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Power on DAC2. Usually we need to tristate PDAC1 if we power on the DAC i.e. IE=0 OE=0 RDE=0 RUE=0"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_xpd_dac(&mut self) -> PDAC2_XPD_DAC_W<PAD_DAC2_SPEC> {
        PDAC2_XPD_DAC_W::new(self, 18)
    }
    #[doc = "Bits 19:26 - PAD DAC2 control code."]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_dac(&mut self) -> PDAC2_DAC_W<PAD_DAC2_SPEC> {
        PDAC2_DAC_W::new(self, 19)
    }
    #[doc = "Bit 27 - the pull up enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_rue(&mut self) -> PDAC2_RUE_W<PAD_DAC2_SPEC> {
        PDAC2_RUE_W::new(self, 27)
    }
    #[doc = "Bit 28 - the pull down enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_rde(&mut self) -> PDAC2_RDE_W<PAD_DAC2_SPEC> {
        PDAC2_RDE_W::new(self, 28)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_hold(&mut self) -> PDAC2_HOLD_W<PAD_DAC2_SPEC> {
        PDAC2_HOLD_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - the driver strength of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_drv(&mut self) -> PDAC2_DRV_W<PAD_DAC2_SPEC> {
        PDAC2_DRV_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_dac2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_dac2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_DAC2_SPEC;
impl crate::RegisterSpec for PAD_DAC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_dac2::R`](R) reader structure"]
impl crate::Readable for PAD_DAC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_dac2::W`](W) writer structure"]
impl crate::Writable for PAD_DAC2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_DAC2 to value 0x8000_0000"]
impl crate::Resettable for PAD_DAC2_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
