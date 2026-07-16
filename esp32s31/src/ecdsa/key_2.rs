#[doc = "Register `KEY_2` reader"]
pub type R = crate::R<KEY_2_SPEC>;
#[doc = "Register `KEY_2` writer"]
pub type W = crate::W<KEY_2_SPEC>;
#[doc = "Field `KEY_2` reader - These bits stores key_2 that is a part of key material."]
pub type KEY_2_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_2` writer - These bits stores key_2 that is a part of key material."]
pub type KEY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_2 that is a part of key material."]
    #[inline(always)]
    pub fn key_2(&self) -> KEY_2_R {
        KEY_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_2")
            .field("key_2", &self.key_2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_2 that is a part of key material."]
    #[inline(always)]
    pub fn key_2(&mut self) -> KEY_2_W<'_, KEY_2_SPEC> {
        KEY_2_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`key_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_2_SPEC;
impl crate::RegisterSpec for KEY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_2::R`](R) reader structure"]
impl crate::Readable for KEY_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_2::W`](W) writer structure"]
impl crate::Writable for KEY_2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_2 to value 0"]
impl crate::Resettable for KEY_2_SPEC {}
