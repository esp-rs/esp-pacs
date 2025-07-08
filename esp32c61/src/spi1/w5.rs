#[doc = "Register `W5` reader"]
pub type R = crate::R<W5_SPEC>;
#[doc = "Register `W5` writer"]
pub type W = crate::W<W5_SPEC>;
#[doc = "Field `BUF5` reader - data buffer"]
pub type BUF5_R = crate::FieldReader<u32>;
#[doc = "Field `BUF5` writer - data buffer"]
pub type BUF5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf5(&self) -> BUF5_R {
        BUF5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W5").field("buf5", &self.buf5()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf5(&mut self) -> BUF5_W<W5_SPEC> {
        BUF5_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer5\n\nYou can [`read`](crate::Reg::read) this register and get [`w5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W5_SPEC;
impl crate::RegisterSpec for W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w5::R`](R) reader structure"]
impl crate::Readable for W5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w5::W`](W) writer structure"]
impl crate::Writable for W5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W5 to value 0"]
impl crate::Resettable for W5_SPEC {}
