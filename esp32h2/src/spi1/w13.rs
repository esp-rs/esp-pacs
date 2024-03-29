#[doc = "Register `W13` reader"]
pub type R = crate::R<W13_SPEC>;
#[doc = "Register `W13` writer"]
pub type W = crate::W<W13_SPEC>;
#[doc = "Field `BUF13` reader - data buffer"]
pub type BUF13_R = crate::FieldReader<u32>;
#[doc = "Field `BUF13` writer - data buffer"]
pub type BUF13_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf13(&self) -> BUF13_R {
        BUF13_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W13")
            .field("buf13", &format_args!("{}", self.buf13().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W13_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf13(&mut self) -> BUF13_W<W13_SPEC> {
        BUF13_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W13_SPEC;
impl crate::RegisterSpec for W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w13::R`](R) reader structure"]
impl crate::Readable for W13_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w13::W`](W) writer structure"]
impl crate::Writable for W13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W13 to value 0"]
impl crate::Resettable for W13_SPEC {
    const RESET_VALUE: u32 = 0;
}
