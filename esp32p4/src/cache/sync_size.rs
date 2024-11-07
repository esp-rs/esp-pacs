#[doc = "Register `SYNC_SIZE` reader"]
pub type R = crate::R<SYNC_SIZE_SPEC>;
#[doc = "Register `SYNC_SIZE` writer"]
pub type W = crate::W<SYNC_SIZE_SPEC>;
#[doc = "Field `SYNC_SIZE` reader - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
pub type SYNC_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `SYNC_SIZE` writer - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
pub type SYNC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
    #[inline(always)]
    pub fn sync_size(&self) -> SYNC_SIZE_R {
        SYNC_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_SIZE")
            .field("sync_size", &self.sync_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the sync operation, which should be used together with CACHE_SYNC_ADDR_REG"]
    #[inline(always)]
    pub fn sync_size(&mut self) -> SYNC_SIZE_W<SYNC_SIZE_SPEC> {
        SYNC_SIZE_W::new(self, 0)
    }
}
#[doc = "Sync size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SIZE_SPEC;
impl crate::RegisterSpec for SYNC_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_size::R`](R) reader structure"]
impl crate::Readable for SYNC_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_size::W`](W) writer structure"]
impl crate::Writable for SYNC_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC_SIZE to value 0"]
impl crate::Resettable for SYNC_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
