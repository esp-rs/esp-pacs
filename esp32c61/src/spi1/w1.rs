#[doc = "Register `W1` reader"]
pub type R = crate::R<W1_SPEC>;
#[doc = "Register `W1` writer"]
pub type W = crate::W<W1_SPEC>;
#[doc = "Field `BUF1` reader - data buffer"]
pub type BUF1_R = crate::FieldReader<u32>;
#[doc = "Field `BUF1` writer - data buffer"]
pub type BUF1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf1(&self) -> BUF1_R {
        BUF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W1").field("buf1", &self.buf1()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf1(&mut self) -> BUF1_W<W1_SPEC> {
        BUF1_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer1\n\nYou can [`read`](crate::Reg::read) this register and get [`w1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W1_SPEC;
impl crate::RegisterSpec for W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w1::R`](R) reader structure"]
impl crate::Readable for W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w1::W`](W) writer structure"]
impl crate::Writable for W1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W1 to value 0"]
impl crate::Resettable for W1_SPEC {}
