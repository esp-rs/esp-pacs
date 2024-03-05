#[doc = "Register `W9` reader"]
pub type R = crate::R<W9_SPEC>;
#[doc = "Register `W9` writer"]
pub type W = crate::W<W9_SPEC>;
#[doc = "Field `BUF9` reader - data buffer"]
pub type BUF9_R = crate::FieldReader<u32>;
#[doc = "Field `BUF9` writer - data buffer"]
pub type BUF9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf9(&self) -> BUF9_R {
        BUF9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W9")
            .field("buf9", &format_args!("{}", self.buf9().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf9(&mut self) -> BUF9_W<W9_SPEC> {
        BUF9_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W9_SPEC;
impl crate::RegisterSpec for W9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w9::R`](R) reader structure"]
impl crate::Readable for W9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w9::W`](W) writer structure"]
impl crate::Writable for W9_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W9 to value 0"]
impl crate::Resettable for W9_SPEC {
    const RESET_VALUE: u32 = 0;
}
