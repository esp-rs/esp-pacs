#[doc = "Register `FREE` reader"]
pub type R = crate::R<FREE_SPEC>;
#[doc = "Register `FREE` writer"]
pub type W = crate::W<FREE_SPEC>;
#[doc = "Field `FREE` reader - 0: AES is non-free; 1:AES is free."]
pub type FREE_R = crate::BitReader;
#[doc = "Field `FREE` writer - 0: AES is non-free; 1:AES is free."]
pub type FREE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: AES is non-free; 1:AES is free."]
    #[inline(always)]
    pub fn free(&self) -> FREE_R {
        FREE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREE").field("free", &self.free()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - 0: AES is non-free; 1:AES is free."]
    #[inline(always)]
    pub fn free(&mut self) -> FREE_W<'_, FREE_SPEC> {
        FREE_W::new(self, 0)
    }
}
#[doc = "AES free state register\n\nYou can [`read`](crate::Reg::read) this register and get [`free::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`free::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREE_SPEC;
impl crate::RegisterSpec for FREE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`free::R`](R) reader structure"]
impl crate::Readable for FREE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`free::W`](W) writer structure"]
impl crate::Writable for FREE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FREE to value 0x01"]
impl crate::Resettable for FREE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
