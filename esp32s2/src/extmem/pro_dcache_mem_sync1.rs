#[doc = "Register `PRO_DCACHE_MEM_SYNC1` reader"]
pub type R = crate::R<PRO_DCACHE_MEM_SYNC1_SPEC>;
#[doc = "Register `PRO_DCACHE_MEM_SYNC1` writer"]
pub type W = crate::W<PRO_DCACHE_MEM_SYNC1_SPEC>;
#[doc = "Field `PRO_DCACHE_MEMSYNC_SIZE` reader - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC0."]
pub type PRO_DCACHE_MEMSYNC_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_DCACHE_MEMSYNC_SIZE` writer - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC0."]
pub type PRO_DCACHE_MEMSYNC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:18 - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC0."]
    #[inline(always)]
    pub fn pro_dcache_memsync_size(&self) -> PRO_DCACHE_MEMSYNC_SIZE_R {
        PRO_DCACHE_MEMSYNC_SIZE_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_MEM_SYNC1")
            .field(
                "pro_dcache_memsync_size",
                &format_args!("{}", self.pro_dcache_memsync_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_MEM_SYNC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18 - The bits are used to configure the length for invalidate, flush, clean, lock and unlock operations. The manual operations will be issued if it is validate. The auto operations will be issued if it is invalidate. It should be combined with PRO_DCACHE_MEM_SYNC0."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dcache_memsync_size(
        &mut self,
    ) -> PRO_DCACHE_MEMSYNC_SIZE_W<PRO_DCACHE_MEM_SYNC1_SPEC> {
        PRO_DCACHE_MEMSYNC_SIZE_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_mem_sync1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dcache_mem_sync1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_MEM_SYNC1_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_MEM_SYNC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_mem_sync1::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_MEM_SYNC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_mem_sync1::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_MEM_SYNC1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_MEM_SYNC1 to value 0"]
impl crate::Resettable for PRO_DCACHE_MEM_SYNC1_SPEC {
    const RESET_VALUE: u32 = 0;
}
