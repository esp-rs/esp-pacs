#[doc = "Register `KEY_5` reader"]
pub type R = crate::R<KEY_5_SPEC>;
#[doc = "Register `KEY_5` writer"]
pub type W = crate::W<KEY_5_SPEC>;
#[doc = "Field `KEY_5` reader - This bits stores key_5 that is a part of key material."]
pub type KEY_5_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_5` writer - This bits stores key_5 that is a part of key material."]
pub type KEY_5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_5 that is a part of key material."]
    #[inline(always)]
    pub fn key_5(&self) -> KEY_5_R {
        KEY_5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_5")
            .field("key_5", &format_args!("{}", self.key_5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_5 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_5(&mut self) -> KEY_5_W<KEY_5_SPEC> {
        KEY_5_W::new(self, 0)
    }
}
#[doc = "Key material key_5 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_5_SPEC;
impl crate::RegisterSpec for KEY_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_5::R`](R) reader structure"]
impl crate::Readable for KEY_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_5::W`](W) writer structure"]
impl crate::Writable for KEY_5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_5 to value 0"]
impl crate::Resettable for KEY_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
