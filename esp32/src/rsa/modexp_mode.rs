#[doc = "Register `MODEXP_MODE` reader"]
pub type R = crate::R<MODEXP_MODE_SPEC>;
#[doc = "Register `MODEXP_MODE` writer"]
pub type W = crate::W<MODEXP_MODE_SPEC>;
#[doc = "Field `MODEXP_MODE` reader - This register contains the mode of modular exponentiation."]
pub type MODEXP_MODE_R = crate::FieldReader;
#[doc = "Field `MODEXP_MODE` writer - This register contains the mode of modular exponentiation."]
pub type MODEXP_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - This register contains the mode of modular exponentiation."]
    #[inline(always)]
    pub fn modexp_mode(&self) -> MODEXP_MODE_R {
        MODEXP_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEXP_MODE")
            .field(
                "modexp_mode",
                &format_args!("{}", self.modexp_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODEXP_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - This register contains the mode of modular exponentiation."]
    #[inline(always)]
    #[must_use]
    pub fn modexp_mode(&mut self) -> MODEXP_MODE_W<MODEXP_MODE_SPEC, 0> {
        MODEXP_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modexp_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modexp_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEXP_MODE_SPEC;
impl crate::RegisterSpec for MODEXP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modexp_mode::R`](R) reader structure"]
impl crate::Readable for MODEXP_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modexp_mode::W`](W) writer structure"]
impl crate::Writable for MODEXP_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODEXP_MODE to value 0"]
impl crate::Resettable for MODEXP_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
