#[doc = "Register `KEY_0` reader"]
pub type R = crate::R<KEY_0_SPEC>;
#[doc = "Register `KEY_0` writer"]
pub type W = crate::W<KEY_0_SPEC>;
#[doc = "Field `KEY_0` reader - These bits stores key_0 that is a part of key material."]
pub type KEY_0_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_0` writer - These bits stores key_0 that is a part of key material."]
pub type KEY_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - These bits stores key_0 that is a part of key material."]
    #[inline(always)]
    pub fn key_0(&self) -> KEY_0_R {
        KEY_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_0")
            .field("key_0", &self.key_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - These bits stores key_0 that is a part of key material."]
    #[inline(always)]
    pub fn key_0(&mut self) -> KEY_0_W<'_, KEY_0_SPEC> {
        KEY_0_W::new(self, 0)
    }
}
#[doc = "ECDSA key data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`key_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_0_SPEC;
impl crate::RegisterSpec for KEY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_0::R`](R) reader structure"]
impl crate::Readable for KEY_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_0::W`](W) writer structure"]
impl crate::Writable for KEY_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_0 to value 0"]
impl crate::Resettable for KEY_0_SPEC {}
