#[doc = "Register `SYNC_MAP` reader"]
pub type R = crate::R<SYNC_MAP_SPEC>;
#[doc = "Register `SYNC_MAP` writer"]
pub type W = crate::W<SYNC_MAP_SPEC>;
#[doc = "Field `SYNC_MAP` reader - Those bits are used to indicate which caches in the two-level cache structure will apply the sync operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
pub type SYNC_MAP_R = crate::FieldReader;
#[doc = "Field `SYNC_MAP` writer - Those bits are used to indicate which caches in the two-level cache structure will apply the sync operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
pub type SYNC_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply the sync operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
    #[inline(always)]
    pub fn sync_map(&self) -> SYNC_MAP_R {
        SYNC_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_MAP")
            .field("sync_map", &format_args!("{}", self.sync_map().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYNC_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Those bits are used to indicate which caches in the two-level cache structure will apply the sync operation. \\[0\\]: L1-ICache0, \\[1\\]: L1-ICache1, \\[2\\]: L1-ICache2, \\[3\\]: L1-ICache3, \\[4\\]: L1-DCache, \\[5\\]: L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn sync_map(&mut self) -> SYNC_MAP_W<SYNC_MAP_SPEC> {
        SYNC_MAP_W::new(self, 0)
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
#[doc = "Sync map configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_MAP_SPEC;
impl crate::RegisterSpec for SYNC_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_map::R`](R) reader structure"]
impl crate::Readable for SYNC_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_map::W`](W) writer structure"]
impl crate::Writable for SYNC_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC_MAP to value 0x1f"]
impl crate::Resettable for SYNC_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
