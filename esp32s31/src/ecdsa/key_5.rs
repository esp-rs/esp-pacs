#[doc = "Register `KEY_5` reader"]
pub type R = crate::R<KEY_5_SPEC>;
#[doc = "Register `KEY_5` writer"]
pub type W = crate::W<KEY_5_SPEC>;
#[doc = "Field `KEY_5` reader - These bits stores key_5 that is a part of key material."]
pub type KEY_5_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_5` writer - These bits stores key_5 that is a part of key material."]
pub type KEY_5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_5 that is a part of key material."]
    #[inline(always)]
    pub fn key_5(&self) -> KEY_5_R {
        KEY_5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_5")
            .field("key_5", &self.key_5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_5 that is a part of key material."]
    #[inline(always)]
    pub fn key_5(&mut self) -> KEY_5_W<'_, KEY_5_SPEC> {
        KEY_5_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`key_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_5_SPEC;
impl crate::RegisterSpec for KEY_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_5::R`](R) reader structure"]
impl crate::Readable for KEY_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_5::W`](W) writer structure"]
impl crate::Writable for KEY_5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_5 to value 0"]
impl crate::Resettable for KEY_5_SPEC {}
