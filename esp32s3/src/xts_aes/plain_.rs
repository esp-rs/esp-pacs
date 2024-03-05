#[doc = "Register `PLAIN_%s` reader"]
pub type R = crate::R<PLAIN__SPEC>;
#[doc = "Register `PLAIN_%s` writer"]
pub type W = crate::W<PLAIN__SPEC>;
#[doc = "Field `PLAIN` reader - Stores the nth 32-bit piece of plaintext."]
pub type PLAIN_R = crate::FieldReader<u32>;
#[doc = "Field `PLAIN` writer - Stores the nth 32-bit piece of plaintext."]
pub type PLAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the nth 32-bit piece of plaintext."]
    #[inline(always)]
    pub fn plain(&self) -> PLAIN_R {
        PLAIN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLAIN_")
            .field("plain", &format_args!("{}", self.plain().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PLAIN__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the nth 32-bit piece of plaintext."]
    #[inline(always)]
    #[must_use]
    pub fn plain(&mut self) -> PLAIN_W<PLAIN__SPEC> {
        PLAIN_W::new(self, 0)
    }
}
#[doc = "Plaintext register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`plain_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`plain_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLAIN__SPEC;
impl crate::RegisterSpec for PLAIN__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plain_::R`](R) reader structure"]
impl crate::Readable for PLAIN__SPEC {}
#[doc = "`write(|w| ..)` method takes [`plain_::W`](W) writer structure"]
impl crate::Writable for PLAIN__SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLAIN_%s to value 0"]
impl crate::Resettable for PLAIN__SPEC {
    const RESET_VALUE: u32 = 0;
}
