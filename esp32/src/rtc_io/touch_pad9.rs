#[doc = "Register `TOUCH_PAD9` reader"]
pub struct R(crate::R<TOUCH_PAD9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOUCH_PAD9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOUCH_PAD9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOUCH_PAD9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOUCH_PAD9` writer"]
pub struct W(crate::W<TOUCH_PAD9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOUCH_PAD9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TOUCH_PAD9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOUCH_PAD9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO_GPIO` reader - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
pub type TO_GPIO_R = crate::BitReader;
#[doc = "Field `TO_GPIO` writer - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
pub type TO_GPIO_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD9_SPEC, O>;
#[doc = "Field `XPD` reader - touch sensor power on."]
pub type XPD_R = crate::BitReader;
#[doc = "Field `XPD` writer - touch sensor power on."]
pub type XPD_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD9_SPEC, O>;
#[doc = "Field `TIE_OPT` reader - default touch sensor tie option. 0: tie low 1: tie high."]
pub type TIE_OPT_R = crate::BitReader;
#[doc = "Field `TIE_OPT` writer - default touch sensor tie option. 0: tie low 1: tie high."]
pub type TIE_OPT_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD9_SPEC, O>;
#[doc = "Field `START` reader - start touch sensor."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - start touch sensor."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, TOUCH_PAD9_SPEC, O>;
#[doc = "Field `DAC` reader - touch sensor slope control. 3-bit for each touch panel default 100."]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - touch sensor slope control. 3-bit for each touch panel default 100."]
pub type DAC_W<'a, const O: u8> = crate::FieldWriter<'a, TOUCH_PAD9_SPEC, 3, O>;
impl R {
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
    #[inline(always)]
    pub fn to_gpio(&self) -> TO_GPIO_R {
        TO_GPIO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD9")
            .field("to_gpio", &format_args!("{}", self.to_gpio().bit()))
            .field("xpd", &format_args!("{}", self.xpd().bit()))
            .field("tie_opt", &format_args!("{}", self.tie_opt().bit()))
            .field("start", &format_args!("{}", self.start().bit()))
            .field("dac", &format_args!("{}", self.dac().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_PAD9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
    #[inline(always)]
    #[must_use]
    pub fn to_gpio(&mut self) -> TO_GPIO_W<19> {
        TO_GPIO_W::new(self)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    #[must_use]
    pub fn xpd(&mut self) -> XPD_W<20> {
        XPD_W::new(self)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    #[must_use]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<21> {
        TIE_OPT_W::new(self)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<22> {
        START_W::new(self)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    #[must_use]
    pub fn dac(&mut self) -> DAC_W<23> {
        DAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [touch_pad9](index.html) module"]
pub struct TOUCH_PAD9_SPEC;
impl crate::RegisterSpec for TOUCH_PAD9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [touch_pad9::R](R) reader structure"]
impl crate::Readable for TOUCH_PAD9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [touch_pad9::W](W) writer structure"]
impl crate::Writable for TOUCH_PAD9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_PAD9 to value 0x0200_0000"]
impl crate::Resettable for TOUCH_PAD9_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
