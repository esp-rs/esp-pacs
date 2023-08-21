#[doc = "Register `SIGMADELTA6` reader"]
pub type R = crate::R<SIGMADELTA6_SPEC>;
#[doc = "Register `SIGMADELTA6` writer"]
pub type W = crate::W<SIGMADELTA6_SPEC>;
#[doc = "Field `SD6_IN` reader - "]
pub type SD6_IN_R = crate::FieldReader;
#[doc = "Field `SD6_IN` writer - "]
pub type SD6_IN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SD6_PRESCALE` reader - "]
pub type SD6_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD6_PRESCALE` writer - "]
pub type SD6_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd6_in(&self) -> SD6_IN_R {
        SD6_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd6_prescale(&self) -> SD6_PRESCALE_R {
        SD6_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA6")
            .field("sd6_in", &format_args!("{}", self.sd6_in().bits()))
            .field(
                "sd6_prescale",
                &format_args!("{}", self.sd6_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd6_in(&mut self) -> SD6_IN_W<SIGMADELTA6_SPEC, 0> {
        SD6_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd6_prescale(&mut self) -> SD6_PRESCALE_W<SIGMADELTA6_SPEC, 8> {
        SD6_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA6_SPEC;
impl crate::RegisterSpec for SIGMADELTA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta6::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta6::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA6 to value 0xff00"]
impl crate::Resettable for SIGMADELTA6_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
