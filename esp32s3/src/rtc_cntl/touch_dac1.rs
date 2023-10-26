#[doc = "Register `TOUCH_DAC1` reader"]
pub type R = crate::R<TOUCH_DAC1_SPEC>;
#[doc = "Register `TOUCH_DAC1` writer"]
pub type W = crate::W<TOUCH_DAC1_SPEC>;
#[doc = "Field `TOUCH_PAD14_DAC` reader - configure touch pad dac14"]
pub type TOUCH_PAD14_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD14_DAC` writer - configure touch pad dac14"]
pub type TOUCH_PAD14_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TOUCH_PAD13_DAC` reader - configure touch pad dac13"]
pub type TOUCH_PAD13_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD13_DAC` writer - configure touch pad dac13"]
pub type TOUCH_PAD13_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TOUCH_PAD12_DAC` reader - configure touch pad dac12"]
pub type TOUCH_PAD12_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD12_DAC` writer - configure touch pad dac12"]
pub type TOUCH_PAD12_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TOUCH_PAD11_DAC` reader - configure touch pad dac11"]
pub type TOUCH_PAD11_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD11_DAC` writer - configure touch pad dac11"]
pub type TOUCH_PAD11_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TOUCH_PAD10_DAC` reader - configure touch pad dac10"]
pub type TOUCH_PAD10_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD10_DAC` writer - configure touch pad dac10"]
pub type TOUCH_PAD10_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 17:19 - configure touch pad dac14"]
    #[inline(always)]
    pub fn touch_pad14_dac(&self) -> TOUCH_PAD14_DAC_R {
        TOUCH_PAD14_DAC_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - configure touch pad dac13"]
    #[inline(always)]
    pub fn touch_pad13_dac(&self) -> TOUCH_PAD13_DAC_R {
        TOUCH_PAD13_DAC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - configure touch pad dac12"]
    #[inline(always)]
    pub fn touch_pad12_dac(&self) -> TOUCH_PAD12_DAC_R {
        TOUCH_PAD12_DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - configure touch pad dac11"]
    #[inline(always)]
    pub fn touch_pad11_dac(&self) -> TOUCH_PAD11_DAC_R {
        TOUCH_PAD11_DAC_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - configure touch pad dac10"]
    #[inline(always)]
    pub fn touch_pad10_dac(&self) -> TOUCH_PAD10_DAC_R {
        TOUCH_PAD10_DAC_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_DAC1")
            .field(
                "touch_pad14_dac",
                &format_args!("{}", self.touch_pad14_dac().bits()),
            )
            .field(
                "touch_pad13_dac",
                &format_args!("{}", self.touch_pad13_dac().bits()),
            )
            .field(
                "touch_pad12_dac",
                &format_args!("{}", self.touch_pad12_dac().bits()),
            )
            .field(
                "touch_pad11_dac",
                &format_args!("{}", self.touch_pad11_dac().bits()),
            )
            .field(
                "touch_pad10_dac",
                &format_args!("{}", self.touch_pad10_dac().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_DAC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 17:19 - configure touch pad dac14"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad14_dac(&mut self) -> TOUCH_PAD14_DAC_W<TOUCH_DAC1_SPEC, 17> {
        TOUCH_PAD14_DAC_W::new(self)
    }
    #[doc = "Bits 20:22 - configure touch pad dac13"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad13_dac(&mut self) -> TOUCH_PAD13_DAC_W<TOUCH_DAC1_SPEC, 20> {
        TOUCH_PAD13_DAC_W::new(self)
    }
    #[doc = "Bits 23:25 - configure touch pad dac12"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad12_dac(&mut self) -> TOUCH_PAD12_DAC_W<TOUCH_DAC1_SPEC, 23> {
        TOUCH_PAD12_DAC_W::new(self)
    }
    #[doc = "Bits 26:28 - configure touch pad dac11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad11_dac(&mut self) -> TOUCH_PAD11_DAC_W<TOUCH_DAC1_SPEC, 26> {
        TOUCH_PAD11_DAC_W::new(self)
    }
    #[doc = "Bits 29:31 - configure touch pad dac10"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad10_dac(&mut self) -> TOUCH_PAD10_DAC_W<TOUCH_DAC1_SPEC, 29> {
        TOUCH_PAD10_DAC_W::new(self)
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
#[doc = "configure touch dac\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_dac1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_dac1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_DAC1_SPEC;
impl crate::RegisterSpec for TOUCH_DAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_dac1::R`](R) reader structure"]
impl crate::Readable for TOUCH_DAC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_dac1::W`](W) writer structure"]
impl crate::Writable for TOUCH_DAC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_DAC1 to value 0"]
impl crate::Resettable for TOUCH_DAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
