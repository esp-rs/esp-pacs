#[doc = "Register `CACHE_MUX_MODE` reader"]
pub type R = crate::R<CACHE_MUX_MODE_SPEC>;
#[doc = "Register `CACHE_MUX_MODE` writer"]
pub type W = crate::W<CACHE_MUX_MODE_SPEC>;
#[doc = "Field `CACHE_MUX_MODE` reader - "]
pub type CACHE_MUX_MODE_R = crate::FieldReader;
#[doc = "Field `CACHE_MUX_MODE` writer - "]
pub type CACHE_MUX_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cache_mux_mode(&self) -> CACHE_MUX_MODE_R {
        CACHE_MUX_MODE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MUX_MODE")
            .field(
                "cache_mux_mode",
                &format_args!("{}", self.cache_mux_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MUX_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mux_mode(&mut self) -> CACHE_MUX_MODE_W<CACHE_MUX_MODE_SPEC, 0> {
        CACHE_MUX_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_mux_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_mux_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MUX_MODE_SPEC;
impl crate::RegisterSpec for CACHE_MUX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_mux_mode::R`](R) reader structure"]
impl crate::Readable for CACHE_MUX_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_mux_mode::W`](W) writer structure"]
impl crate::Writable for CACHE_MUX_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_MUX_MODE to value 0"]
impl crate::Resettable for CACHE_MUX_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
