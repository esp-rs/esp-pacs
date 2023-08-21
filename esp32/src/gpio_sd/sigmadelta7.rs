#[doc = "Register `SIGMADELTA7` reader"]
pub type R = crate::R<SIGMADELTA7_SPEC>;
#[doc = "Register `SIGMADELTA7` writer"]
pub type W = crate::W<SIGMADELTA7_SPEC>;
#[doc = "Field `SD7_IN` reader - "]
pub type SD7_IN_R = crate::FieldReader;
#[doc = "Field `SD7_IN` writer - "]
pub type SD7_IN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SD7_PRESCALE` reader - "]
pub type SD7_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD7_PRESCALE` writer - "]
pub type SD7_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd7_in(&self) -> SD7_IN_R {
        SD7_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd7_prescale(&self) -> SD7_PRESCALE_R {
        SD7_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA7")
            .field("sd7_in", &format_args!("{}", self.sd7_in().bits()))
            .field(
                "sd7_prescale",
                &format_args!("{}", self.sd7_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd7_in(&mut self) -> SD7_IN_W<SIGMADELTA7_SPEC, 0> {
        SD7_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd7_prescale(&mut self) -> SD7_PRESCALE_W<SIGMADELTA7_SPEC, 8> {
        SD7_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA7_SPEC;
impl crate::RegisterSpec for SIGMADELTA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta7::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta7::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA7 to value 0xff00"]
impl crate::Resettable for SIGMADELTA7_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
