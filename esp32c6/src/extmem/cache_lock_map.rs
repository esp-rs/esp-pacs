#[doc = "Register `CACHE_LOCK_MAP` reader"]
pub type R = crate::R<CACHE_LOCK_MAP_SPEC>;
#[doc = "Register `CACHE_LOCK_MAP` writer"]
pub type W = crate::W<CACHE_LOCK_MAP_SPEC>;
#[doc = "Field `CACHE_LOCK_MAP` reader - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[4\\]: L1-Cache"]
pub type CACHE_LOCK_MAP_R = crate::FieldReader;
#[doc = "Field `CACHE_LOCK_MAP` writer - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[4\\]: L1-Cache"]
pub type CACHE_LOCK_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[4\\]: L1-Cache"]
    #[inline(always)]
    pub fn cache_lock_map(&self) -> CACHE_LOCK_MAP_R {
        CACHE_LOCK_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_LOCK_MAP")
            .field(
                "cache_lock_map",
                &format_args!("{}", self.cache_lock_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_LOCK_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply this lock/unlock operation. \\[4\\]: L1-Cache"]
    #[inline(always)]
    #[must_use]
    pub fn cache_lock_map(&mut self) -> CACHE_LOCK_MAP_W<CACHE_LOCK_MAP_SPEC, 0> {
        CACHE_LOCK_MAP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Lock (manual lock) map configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cache_lock_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_lock_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_LOCK_MAP_SPEC;
impl crate::RegisterSpec for CACHE_LOCK_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_lock_map::R`](R) reader structure"]
impl crate::Readable for CACHE_LOCK_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_lock_map::W`](W) writer structure"]
impl crate::Writable for CACHE_LOCK_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_LOCK_MAP to value 0"]
impl crate::Resettable for CACHE_LOCK_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
