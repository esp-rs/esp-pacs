#[doc = "Register `W15` reader"]
pub type R = crate::R<W15_SPEC>;
#[doc = "Register `W15` writer"]
pub type W = crate::W<W15_SPEC>;
#[doc = "Field `BUF15` reader - data buffer"]
pub type BUF15_R = crate::FieldReader<u32>;
#[doc = "Field `BUF15` writer - data buffer"]
pub type BUF15_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf15(&self) -> BUF15_R {
        BUF15_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W15").field("buf15", &self.buf15()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf15(&mut self) -> BUF15_W<W15_SPEC> {
        BUF15_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer15\n\nYou can [`read`](crate::Reg::read) this register and get [`w15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W15_SPEC;
impl crate::RegisterSpec for W15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w15::R`](R) reader structure"]
impl crate::Readable for W15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w15::W`](W) writer structure"]
impl crate::Writable for W15_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W15 to value 0"]
impl crate::Resettable for W15_SPEC {}
