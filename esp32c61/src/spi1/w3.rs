#[doc = "Register `W3` reader"]
pub type R = crate::R<W3_SPEC>;
#[doc = "Register `W3` writer"]
pub type W = crate::W<W3_SPEC>;
#[doc = "Field `BUF3` reader - data buffer"]
pub type BUF3_R = crate::FieldReader<u32>;
#[doc = "Field `BUF3` writer - data buffer"]
pub type BUF3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf3(&self) -> BUF3_R {
        BUF3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W3").field("buf3", &self.buf3()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf3(&mut self) -> BUF3_W<W3_SPEC> {
        BUF3_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer3\n\nYou can [`read`](crate::Reg::read) this register and get [`w3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W3_SPEC;
impl crate::RegisterSpec for W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w3::R`](R) reader structure"]
impl crate::Readable for W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w3::W`](W) writer structure"]
impl crate::Writable for W3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W3 to value 0"]
impl crate::Resettable for W3_SPEC {}
