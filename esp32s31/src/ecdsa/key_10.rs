#[doc = "Register `KEY_10` reader"]
pub type R = crate::R<KEY_10_SPEC>;
#[doc = "Register `KEY_10` writer"]
pub type W = crate::W<KEY_10_SPEC>;
#[doc = "Field `KEY_10` reader - These bits stores key_10 that is a part of key material."]
pub type KEY_10_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_10` writer - These bits stores key_10 that is a part of key material."]
pub type KEY_10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_10 that is a part of key material."]
    #[inline(always)]
    pub fn key_10(&self) -> KEY_10_R {
        KEY_10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_10")
            .field("key_10", &self.key_10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_10 that is a part of key material."]
    #[inline(always)]
    pub fn key_10(&mut self) -> KEY_10_W<'_, KEY_10_SPEC> {
        KEY_10_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`key_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_10_SPEC;
impl crate::RegisterSpec for KEY_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_10::R`](R) reader structure"]
impl crate::Readable for KEY_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_10::W`](W) writer structure"]
impl crate::Writable for KEY_10_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_10 to value 0"]
impl crate::Resettable for KEY_10_SPEC {}
