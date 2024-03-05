#[doc = "Register `W10` reader"]
pub type R = crate::R<W10_SPEC>;
#[doc = "Register `W10` writer"]
pub type W = crate::W<W10_SPEC>;
#[doc = "Field `BUF10` reader - data buffer"]
pub type BUF10_R = crate::FieldReader<u32>;
#[doc = "Field `BUF10` writer - data buffer"]
pub type BUF10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf10(&self) -> BUF10_R {
        BUF10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W10")
            .field("buf10", &format_args!("{}", self.buf10().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf10(&mut self) -> BUF10_W<W10_SPEC> {
        BUF10_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W10_SPEC;
impl crate::RegisterSpec for W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w10::R`](R) reader structure"]
impl crate::Readable for W10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w10::W`](W) writer structure"]
impl crate::Writable for W10_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W10 to value 0"]
impl crate::Resettable for W10_SPEC {
    const RESET_VALUE: u32 = 0;
}
