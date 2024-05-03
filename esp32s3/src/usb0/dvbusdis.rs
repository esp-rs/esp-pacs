#[doc = "Register `DVBUSDIS` reader"]
pub type R = crate::R<DVBUSDIS_SPEC>;
#[doc = "Register `DVBUSDIS` writer"]
pub type W = crate::W<DVBUSDIS_SPEC>;
#[doc = "Field `DVBUSDIS` reader - "]
pub type DVBUSDIS_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSDIS` writer - "]
pub type DVBUSDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dvbusdis(&self) -> DVBUSDIS_R {
        DVBUSDIS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSDIS")
            .field("dvbusdis", &self.dvbusdis().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DVBUSDIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn dvbusdis(&mut self) -> DVBUSDIS_W<DVBUSDIS_SPEC> {
        DVBUSDIS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dvbusdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dvbusdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVBUSDIS_SPEC;
impl crate::RegisterSpec for DVBUSDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbusdis::R`](R) reader structure"]
impl crate::Readable for DVBUSDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dvbusdis::W`](W) writer structure"]
impl crate::Writable for DVBUSDIS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DVBUSDIS to value 0x17d7"]
impl crate::Resettable for DVBUSDIS_SPEC {
    const RESET_VALUE: u32 = 0x17d7;
}
