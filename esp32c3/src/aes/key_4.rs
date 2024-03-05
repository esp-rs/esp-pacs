#[doc = "Register `KEY_4` reader"]
pub type R = crate::R<KEY_4_SPEC>;
#[doc = "Register `KEY_4` writer"]
pub type W = crate::W<KEY_4_SPEC>;
#[doc = "Field `KEY_4` reader - This bits stores key_4 that is a part of key material."]
pub type KEY_4_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_4` writer - This bits stores key_4 that is a part of key material."]
pub type KEY_4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_4 that is a part of key material."]
    #[inline(always)]
    pub fn key_4(&self) -> KEY_4_R {
        KEY_4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_4")
            .field("key_4", &format_args!("{}", self.key_4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_4 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_4(&mut self) -> KEY_4_W<KEY_4_SPEC> {
        KEY_4_W::new(self, 0)
    }
}
#[doc = "Key material key_4 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_4_SPEC;
impl crate::RegisterSpec for KEY_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_4::R`](R) reader structure"]
impl crate::Readable for KEY_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_4::W`](W) writer structure"]
impl crate::Writable for KEY_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_4 to value 0"]
impl crate::Resettable for KEY_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
