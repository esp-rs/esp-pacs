#[doc = "Register `KEY_7` reader"]
pub type R = crate::R<KEY_7_SPEC>;
#[doc = "Register `KEY_7` writer"]
pub type W = crate::W<KEY_7_SPEC>;
#[doc = "Field `KEY_7` reader - These bits stores key_7 that is a part of key material."]
pub type KEY_7_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_7` writer - These bits stores key_7 that is a part of key material."]
pub type KEY_7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_7 that is a part of key material."]
    #[inline(always)]
    pub fn key_7(&self) -> KEY_7_R {
        KEY_7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_7")
            .field("key_7", &self.key_7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_7 that is a part of key material."]
    #[inline(always)]
    pub fn key_7(&mut self) -> KEY_7_W<'_, KEY_7_SPEC> {
        KEY_7_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`key_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_7_SPEC;
impl crate::RegisterSpec for KEY_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_7::R`](R) reader structure"]
impl crate::Readable for KEY_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_7::W`](W) writer structure"]
impl crate::Writable for KEY_7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_7 to value 0"]
impl crate::Resettable for KEY_7_SPEC {}
