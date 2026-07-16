#[doc = "Register `KEY_9` reader"]
pub type R = crate::R<KEY_9_SPEC>;
#[doc = "Register `KEY_9` writer"]
pub type W = crate::W<KEY_9_SPEC>;
#[doc = "Field `KEY_9` reader - These bits stores key_9 that is a part of key material."]
pub type KEY_9_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_9` writer - These bits stores key_9 that is a part of key material."]
pub type KEY_9_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_9 that is a part of key material."]
    #[inline(always)]
    pub fn key_9(&self) -> KEY_9_R {
        KEY_9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_9")
            .field("key_9", &self.key_9())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_9 that is a part of key material."]
    #[inline(always)]
    pub fn key_9(&mut self) -> KEY_9_W<'_, KEY_9_SPEC> {
        KEY_9_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`key_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_9_SPEC;
impl crate::RegisterSpec for KEY_9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_9::R`](R) reader structure"]
impl crate::Readable for KEY_9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_9::W`](W) writer structure"]
impl crate::Writable for KEY_9_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_9 to value 0"]
impl crate::Resettable for KEY_9_SPEC {}
