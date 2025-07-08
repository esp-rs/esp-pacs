#[doc = "Register `W7` reader"]
pub type R = crate::R<W7_SPEC>;
#[doc = "Register `W7` writer"]
pub type W = crate::W<W7_SPEC>;
#[doc = "Field `BUF7` reader - data buffer"]
pub type BUF7_R = crate::FieldReader<u32>;
#[doc = "Field `BUF7` writer - data buffer"]
pub type BUF7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf7(&self) -> BUF7_R {
        BUF7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W7").field("buf7", &self.buf7()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf7(&mut self) -> BUF7_W<W7_SPEC> {
        BUF7_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer7\n\nYou can [`read`](crate::Reg::read) this register and get [`w7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W7_SPEC;
impl crate::RegisterSpec for W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w7::R`](R) reader structure"]
impl crate::Readable for W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w7::W`](W) writer structure"]
impl crate::Writable for W7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W7 to value 0"]
impl crate::Resettable for W7_SPEC {}
