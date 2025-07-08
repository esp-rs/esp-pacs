#[doc = "Register `KEY_%s` reader"]
pub type R = crate::R<KEY__SPEC>;
#[doc = "Register `KEY_%s` writer"]
pub type W = crate::W<KEY__SPEC>;
#[doc = "Field `KEY_0` reader - This bits stores key_0 that is a part of key material."]
pub type KEY_0_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_0` writer - This bits stores key_0 that is a part of key material."]
pub type KEY_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_0 that is a part of key material."]
    #[inline(always)]
    pub fn key_0(&self) -> KEY_0_R {
        KEY_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_")
            .field("key_0", &self.key_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_0 that is a part of key material."]
    #[inline(always)]
    pub fn key_0(&mut self) -> KEY_0_W<KEY__SPEC> {
        KEY_0_W::new(self, 0)
    }
}
#[doc = "AES key data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`key_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY__SPEC;
impl crate::RegisterSpec for KEY__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_::R`](R) reader structure"]
impl crate::Readable for KEY__SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_::W`](W) writer structure"]
impl crate::Writable for KEY__SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_%s to value 0"]
impl crate::Resettable for KEY__SPEC {}
