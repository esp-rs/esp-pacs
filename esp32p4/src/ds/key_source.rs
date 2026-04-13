#[doc = "Register `KEY_SOURCE` reader"]
pub type R = crate::R<KEY_SOURCE_SPEC>;
#[doc = "Register `KEY_SOURCE` writer"]
pub type W = crate::W<KEY_SOURCE_SPEC>;
#[doc = "Field `KEY_SOURCE` reader - "]
pub type KEY_SOURCE_R = crate::BitReader;
#[doc = "Field `KEY_SOURCE` writer - "]
pub type KEY_SOURCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn key_source(&self) -> KEY_SOURCE_R {
        KEY_SOURCE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_SOURCE")
            .field("key_source", &self.key_source())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn key_source(&mut self) -> KEY_SOURCE_W<'_, KEY_SOURCE_SPEC> {
        KEY_SOURCE_W::new(self, 0)
    }
}
#[doc = "Digital signature key source\n\nYou can [`read`](crate::Reg::read) this register and get [`key_source::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_source::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_SOURCE_SPEC;
impl crate::RegisterSpec for KEY_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_source::R`](R) reader structure"]
impl crate::Readable for KEY_SOURCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_source::W`](W) writer structure"]
impl crate::Writable for KEY_SOURCE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY_SOURCE to value 0"]
impl crate::Resettable for KEY_SOURCE_SPEC {}
