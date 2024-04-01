#[doc = "Register `W%s` reader"]
pub type R = crate::R<W_SPEC>;
#[doc = "Register `W%s` writer"]
pub type W = crate::W<W_SPEC>;
#[doc = "Field `BUF` reader - data buffer"]
pub type BUF_R = crate::FieldReader<u32>;
#[doc = "Field `BUF` writer - data buffer"]
pub type BUF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W")
            .field("buf", &format_args!("{}", self.buf().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf(&mut self) -> BUF_W<W_SPEC> {
        BUF_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W_SPEC;
impl crate::RegisterSpec for W_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w::R`](R) reader structure"]
impl crate::Readable for W_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w::W`](W) writer structure"]
impl crate::Writable for W_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W%s to value 0"]
impl crate::Resettable for W_SPEC {
    const RESET_VALUE: u32 = 0;
}
