#[doc = "Register `KEY_1` reader"]
pub type R = crate::R<KEY_1_SPEC>;
#[doc = "Register `KEY_1` writer"]
pub type W = crate::W<KEY_1_SPEC>;
#[doc = "Field `KEY_1` reader - These bits stores key_1 that is a part of key material."]
pub type KEY_1_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_1` writer - These bits stores key_1 that is a part of key material."]
pub type KEY_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_1 that is a part of key material."]
    #[inline(always)]
    pub fn key_1(&self) -> KEY_1_R {
        KEY_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_1")
            .field("key_1", &self.key_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_1 that is a part of key material."]
    #[inline(always)]
    pub fn key_1(&mut self) -> KEY_1_W<'_, KEY_1_SPEC> {
        KEY_1_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`key_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_1_SPEC;
impl crate::RegisterSpec for KEY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_1::R`](R) reader structure"]
impl crate::Readable for KEY_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_1::W`](W) writer structure"]
impl crate::Writable for KEY_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_1 to value 0"]
impl crate::Resettable for KEY_1_SPEC {}
