#[doc = "Register `_3_MODE` reader"]
pub type R = crate::R<_3_MODE_SPEC>;
#[doc = "Register `_3_MODE` writer"]
pub type W = crate::W<_3_MODE_SPEC>;
#[doc = "Field `_3_MODE` reader - Sha3 mode"]
pub type _3_MODE_R = crate::FieldReader;
#[doc = "Field `_3_MODE` writer - Sha3 mode"]
pub type _3_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Sha3 mode"]
    #[inline(always)]
    pub fn _3_mode(&self) -> _3_MODE_R {
        _3_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_3_MODE")
            .field("_3_mode", &self._3_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Sha3 mode"]
    #[inline(always)]
    pub fn _3_mode(&mut self) -> _3_MODE_W<'_, _3_MODE_SPEC> {
        _3_MODE_W::new(self, 0)
    }
}
#[doc = "Initial configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_MODE_SPEC;
impl crate::RegisterSpec for _3_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_mode::R`](R) reader structure"]
impl crate::Readable for _3_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_mode::W`](W) writer structure"]
impl crate::Writable for _3_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_MODE to value 0"]
impl crate::Resettable for _3_MODE_SPEC {}
