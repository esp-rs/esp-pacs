#[doc = "Register `KEY_8` reader"]
pub type R = crate::R<KEY_8_SPEC>;
#[doc = "Register `KEY_8` writer"]
pub type W = crate::W<KEY_8_SPEC>;
#[doc = "Field `KEY_8` reader - These bits stores key_8 that is a part of key material."]
pub type KEY_8_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_8` writer - These bits stores key_8 that is a part of key material."]
pub type KEY_8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_8 that is a part of key material."]
    #[inline(always)]
    pub fn key_8(&self) -> KEY_8_R {
        KEY_8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_8")
            .field("key_8", &self.key_8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_8 that is a part of key material."]
    #[inline(always)]
    pub fn key_8(&mut self) -> KEY_8_W<'_, KEY_8_SPEC> {
        KEY_8_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`key_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_8_SPEC;
impl crate::RegisterSpec for KEY_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_8::R`](R) reader structure"]
impl crate::Readable for KEY_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_8::W`](W) writer structure"]
impl crate::Writable for KEY_8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_8 to value 0"]
impl crate::Resettable for KEY_8_SPEC {}
