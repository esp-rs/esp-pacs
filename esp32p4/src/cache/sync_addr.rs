#[doc = "Register `SYNC_ADDR` reader"]
pub type R = crate::R<SYNC_ADDR_SPEC>;
#[doc = "Register `SYNC_ADDR` writer"]
pub type W = crate::W<SYNC_ADDR_SPEC>;
#[doc = "Field `SYNC_ADDR` reader - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
pub type SYNC_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SYNC_ADDR` writer - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
pub type SYNC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
    #[inline(always)]
    pub fn sync_addr(&self) -> SYNC_ADDR_R {
        SYNC_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC_ADDR")
            .field("sync_addr", &format_args!("{}", self.sync_addr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYNC_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the sync operation, which should be used together with CACHE_SYNC_SIZE_REG"]
    #[inline(always)]
    #[must_use]
    pub fn sync_addr(&mut self) -> SYNC_ADDR_W<SYNC_ADDR_SPEC> {
        SYNC_ADDR_W::new(self, 0)
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
#[doc = "Sync address configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_ADDR_SPEC;
impl crate::RegisterSpec for SYNC_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync_addr::R`](R) reader structure"]
impl crate::Readable for SYNC_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync_addr::W`](W) writer structure"]
impl crate::Writable for SYNC_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC_ADDR to value 0"]
impl crate::Resettable for SYNC_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
