#[doc = "Register `KEY_2` reader"]
pub type R = crate::R<KEY_2_SPEC>;
#[doc = "Register `KEY_2` writer"]
pub type W = crate::W<KEY_2_SPEC>;
#[doc = "Field `KEY_2` reader - This bits stores key_2 that is a part of key material."]
pub type KEY_2_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_2` writer - This bits stores key_2 that is a part of key material."]
pub type KEY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_2 that is a part of key material."]
    #[inline(always)]
    pub fn key_2(&self) -> KEY_2_R {
        KEY_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_2")
            .field("key_2", &format_args!("{}", self.key_2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_2 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_2(&mut self) -> KEY_2_W<KEY_2_SPEC> {
        KEY_2_W::new(self, 0)
    }
}
#[doc = "Key material key_2 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_2_SPEC;
impl crate::RegisterSpec for KEY_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_2::R`](R) reader structure"]
impl crate::Readable for KEY_2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_2::W`](W) writer structure"]
impl crate::Writable for KEY_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_2 to value 0"]
impl crate::Resettable for KEY_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
