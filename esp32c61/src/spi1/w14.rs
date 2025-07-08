#[doc = "Register `W14` reader"]
pub type R = crate::R<W14_SPEC>;
#[doc = "Register `W14` writer"]
pub type W = crate::W<W14_SPEC>;
#[doc = "Field `BUF14` reader - data buffer"]
pub type BUF14_R = crate::FieldReader<u32>;
#[doc = "Field `BUF14` writer - data buffer"]
pub type BUF14_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf14(&self) -> BUF14_R {
        BUF14_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W14").field("buf14", &self.buf14()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf14(&mut self) -> BUF14_W<W14_SPEC> {
        BUF14_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer14\n\nYou can [`read`](crate::Reg::read) this register and get [`w14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W14_SPEC;
impl crate::RegisterSpec for W14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w14::R`](R) reader structure"]
impl crate::Readable for W14_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w14::W`](W) writer structure"]
impl crate::Writable for W14_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W14 to value 0"]
impl crate::Resettable for W14_SPEC {}
