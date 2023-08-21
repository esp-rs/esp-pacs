#[doc = "Register `SIGMADELTA3` reader"]
pub type R = crate::R<SIGMADELTA3_SPEC>;
#[doc = "Register `SIGMADELTA3` writer"]
pub type W = crate::W<SIGMADELTA3_SPEC>;
#[doc = "Field `SD3_IN` reader - "]
pub type SD3_IN_R = crate::FieldReader;
#[doc = "Field `SD3_IN` writer - "]
pub type SD3_IN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SD3_PRESCALE` reader - "]
pub type SD3_PRESCALE_R = crate::FieldReader;
#[doc = "Field `SD3_PRESCALE` writer - "]
pub type SD3_PRESCALE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sd3_in(&self) -> SD3_IN_R {
        SD3_IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sd3_prescale(&self) -> SD3_PRESCALE_R {
        SD3_PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA3")
            .field("sd3_in", &format_args!("{}", self.sd3_in().bits()))
            .field(
                "sd3_prescale",
                &format_args!("{}", self.sd3_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sd3_in(&mut self) -> SD3_IN_W<SIGMADELTA3_SPEC, 0> {
        SD3_IN_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sd3_prescale(&mut self) -> SD3_PRESCALE_W<SIGMADELTA3_SPEC, 8> {
        SD3_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA3_SPEC;
impl crate::RegisterSpec for SIGMADELTA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta3::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta3::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGMADELTA3 to value 0xff00"]
impl crate::Resettable for SIGMADELTA3_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00;
}
