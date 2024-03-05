#[doc = "Register `W2` reader"]
pub type R = crate::R<W2_SPEC>;
#[doc = "Register `W2` writer"]
pub type W = crate::W<W2_SPEC>;
#[doc = "Field `BUF2` reader - data buffer"]
pub type BUF2_R = crate::FieldReader<u32>;
#[doc = "Field `BUF2` writer - data buffer"]
pub type BUF2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf2(&self) -> BUF2_R {
        BUF2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W2")
            .field("buf2", &format_args!("{}", self.buf2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf2(&mut self) -> BUF2_W<W2_SPEC> {
        BUF2_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W2_SPEC;
impl crate::RegisterSpec for W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w2::R`](R) reader structure"]
impl crate::Readable for W2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w2::W`](W) writer structure"]
impl crate::Writable for W2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W2 to value 0"]
impl crate::Resettable for W2_SPEC {
    const RESET_VALUE: u32 = 0;
}
