#[doc = "Register `W11` reader"]
pub type R = crate::R<W11_SPEC>;
#[doc = "Register `W11` writer"]
pub type W = crate::W<W11_SPEC>;
#[doc = "Field `BUF11` reader - data buffer"]
pub type BUF11_R = crate::FieldReader<u32>;
#[doc = "Field `BUF11` writer - data buffer"]
pub type BUF11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf11(&self) -> BUF11_R {
        BUF11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W11")
            .field("buf11", &format_args!("{}", self.buf11().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf11(&mut self) -> BUF11_W<W11_SPEC> {
        BUF11_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W11_SPEC;
impl crate::RegisterSpec for W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w11::R`](R) reader structure"]
impl crate::Readable for W11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w11::W`](W) writer structure"]
impl crate::Writable for W11_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W11 to value 0"]
impl crate::Resettable for W11_SPEC {
    const RESET_VALUE: u32 = 0;
}
