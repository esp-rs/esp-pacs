#[doc = "Register `RTC_PAD19` reader"]
pub type R = crate::R<RTC_PAD19_SPEC>;
#[doc = "Register `RTC_PAD19` writer"]
pub type W = crate::W<RTC_PAD19_SPEC>;
#[doc = "Field `FUN_IE` reader - Input enable in normal execution."]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Input enable in normal execution."]
pub type FUN_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLP_OE` reader - Output enable in sleep mode."]
pub type SLP_OE_R = crate::BitReader;
#[doc = "Field `SLP_OE` writer - Output enable in sleep mode."]
pub type SLP_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLP_IE` reader - Input enable in sleep mode."]
pub type SLP_IE_R = crate::BitReader;
#[doc = "Field `SLP_IE` writer - Input enable in sleep mode."]
pub type SLP_IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLP_SEL` reader - 1: enable sleep mode. 0: no sleep mode"]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - 1: enable sleep mode. 0: no sleep mode"]
pub type SLP_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FUN_SEL` reader - Function selection."]
pub type FUN_SEL_R = crate::FieldReader;
#[doc = "Field `FUN_SEL` writer - Function selection."]
pub type FUN_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MUX_SEL` reader - 1: use RTC GPIO. 0: use digital GPIO."]
pub type MUX_SEL_R = crate::BitReader;
#[doc = "Field `MUX_SEL` writer - 1: use RTC GPIO. 0: use digital GPIO."]
pub type MUX_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RUE` reader - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_R = crate::BitReader;
#[doc = "Field `RUE` writer - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
pub type RUE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDE` reader - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_R = crate::BitReader;
#[doc = "Field `RDE` writer - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
pub type RDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRV` reader - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
pub type DRV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    pub fn slp_oe(&self) -> SLP_OE_R {
        SLP_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    pub fn slp_ie(&self) -> SLP_IE_R {
        SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    pub fn fun_sel(&self) -> FUN_SEL_R {
        FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO."]
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
        f.debug_struct("RTC_PAD19")
            .field("fun_ie", &format_args!("{}", self.fun_ie().bit()))
            .field("slp_oe", &format_args!("{}", self.slp_oe().bit()))
            .field("slp_ie", &format_args!("{}", self.slp_ie().bit()))
            .field("slp_sel", &format_args!("{}", self.slp_sel().bit()))
            .field("fun_sel", &format_args!("{}", self.fun_sel().bits()))
            .field("mux_sel", &format_args!("{}", self.mux_sel().bit()))
            .field("rue", &format_args!("{}", self.rue().bit()))
            .field("rde", &format_args!("{}", self.rde().bit()))
            .field("drv", &format_args!("{}", self.drv().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_PAD19_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - Input enable in normal execution."]
    #[inline(always)]
    #[must_use]
    pub fn fun_ie(&mut self) -> FUN_IE_W<RTC_PAD19_SPEC, 13> {
        FUN_IE_W::new(self)
    }
    #[doc = "Bit 14 - Output enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_oe(&mut self) -> SLP_OE_W<RTC_PAD19_SPEC, 14> {
        SLP_OE_W::new(self)
    }
    #[doc = "Bit 15 - Input enable in sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn slp_ie(&mut self) -> SLP_IE_W<RTC_PAD19_SPEC, 15> {
        SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - 1: enable sleep mode. 0: no sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<RTC_PAD19_SPEC, 16> {
        SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - Function selection."]
    #[inline(always)]
    #[must_use]
    pub fn fun_sel(&mut self) -> FUN_SEL_W<RTC_PAD19_SPEC, 17> {
        FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - 1: use RTC GPIO. 0: use digital GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn mux_sel(&mut self) -> MUX_SEL_W<RTC_PAD19_SPEC, 19> {
        MUX_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Pull-down enable of the pad. 1: internal pull-down enabled. 0: internal pull-down disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rue(&mut self) -> RUE_W<RTC_PAD19_SPEC, 27> {
        RUE_W::new(self)
    }
    #[doc = "Bit 28 - Pull-up enable of the pad. 1: internal pull-up enabled. 0: internal pull-up disabled."]
    #[inline(always)]
    #[must_use]
    pub fn rde(&mut self) -> RDE_W<RTC_PAD19_SPEC, 28> {
        RDE_W::new(self)
    }
    #[doc = "Bits 29:30 - Select the drive strength of the pad. 0: ~5 mA. 1: ~10 mA. 2: ~20 mA. 3: ~40 mA."]
    #[inline(always)]
    #[must_use]
    pub fn drv(&mut self) -> DRV_W<RTC_PAD19_SPEC, 29> {
        DRV_W::new(self)
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
#[doc = "Touch pad 19 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_pad19::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_pad19::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_PAD19_SPEC;
impl crate::RegisterSpec for RTC_PAD19_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_pad19::R`](R) reader structure"]
impl crate::Readable for RTC_PAD19_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_pad19::W`](W) writer structure"]
impl crate::Writable for RTC_PAD19_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_PAD19 to value 0x5000_0000"]
impl crate::Resettable for RTC_PAD19_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000_0000;
}
