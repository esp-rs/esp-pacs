#[doc = "Register `SIGMADELTA4` reader"]
pub type R = crate::R<SIGMADELTA4_SPEC>;
#[doc = "Register `SIGMADELTA4` writer"]
pub type W = crate::W<SIGMADELTA4_SPEC>;
#[doc = "Field `SD4_IN` reader - "]
pub type SD4_IN_R = crate::FieldReader;
#[doc = "Field `SD4_IN` writer - "]
pub type SD4_IN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SD4_PRESCALE` reader - "]
pub type SD4_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD4_PRESCALE` writer - "]
pub type SD4_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd4_in(&self) -> SD4_IN_R {
        SD4_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd4_prescale(&self) -> SD4_PRESCALE_R {
        SD4_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA4")
            .field("sd4_in", &format_args!("{}", self.sd4_in().bits()))
            .field(
                "sd4_prescale",
                &format_args!("{}", self.sd4_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd4_in(&mut self) -> SD4_IN_W<SIGMADELTA4_SPEC, 0> {
        SD4_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd4_prescale(&mut self) -> SD4_PRESCALE_W<SIGMADELTA4_SPEC, 8> {
        SD4_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA4_SPEC;
impl crate::RegisterSpec for SIGMADELTA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta4::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta4::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA4 to value 0xff00"]
impl crate::Resettable for SIGMADELTA4_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
