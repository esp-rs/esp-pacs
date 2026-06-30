#[doc = "Register `KEY_11` reader"]
pub type R = crate::R<KEY_11_SPEC>;
#[doc = "Register `KEY_11` writer"]
pub type W = crate::W<KEY_11_SPEC>;
#[doc = "Field `KEY_11` reader - These bits stores key_11 that is a part of key material."]
pub type KEY_11_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_11` writer - These bits stores key_11 that is a part of key material."]
pub type KEY_11_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_11 that is a part of key material."]
    #[inline(always)]
    pub fn key_11(&self) -> KEY_11_R {
        KEY_11_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_11")
            .field("key_11", &self.key_11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_11 that is a part of key material."]
    #[inline(always)]
    pub fn key_11(&mut self) -> KEY_11_W<'_, KEY_11_SPEC> {
        KEY_11_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`key_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_11_SPEC;
impl crate::RegisterSpec for KEY_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_11::R`](R) reader structure"]
impl crate::Readable for KEY_11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_11::W`](W) writer structure"]
impl crate::Writable for KEY_11_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_11 to value 0"]
impl crate::Resettable for KEY_11_SPEC {}
