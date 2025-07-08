#[doc = "Register `W4` reader"]
pub type R = crate::R<W4_SPEC>;
#[doc = "Register `W4` writer"]
pub type W = crate::W<W4_SPEC>;
#[doc = "Field `BUF4` reader - data buffer"]
pub type BUF4_R = crate::FieldReader<u32>;
#[doc = "Field `BUF4` writer - data buffer"]
pub type BUF4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf4(&self) -> BUF4_R {
        BUF4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W4").field("buf4", &self.buf4()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf4(&mut self) -> BUF4_W<W4_SPEC> {
        BUF4_W::new(self, 0)
    }
}
#[doc = "SPI1 memory data buffer4\n\nYou can [`read`](crate::Reg::read) this register and get [`w4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W4_SPEC;
impl crate::RegisterSpec for W4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w4::R`](R) reader structure"]
impl crate::Readable for W4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w4::W`](W) writer structure"]
impl crate::Writable for W4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W4 to value 0"]
impl crate::Resettable for W4_SPEC {}
