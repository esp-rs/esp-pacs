#[doc = "Register `DCACHE_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Register `DCACHE_PRELOCK_SCT_SIZE` writer"]
pub type W = crate::W<DCACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_SIZE` reader - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `DCACHE_PRELOCK_SCT1_SIZE` writer - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DCACHE_PRELOCK_SCT0_SIZE` reader - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `DCACHE_PRELOCK_SCT0_SIZE` writer - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
pub type DCACHE_PRELOCK_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct1_size(&self) -> DCACHE_PRELOCK_SCT1_SIZE_R {
        DCACHE_PRELOCK_SCT1_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct0_size(&self) -> DCACHE_PRELOCK_SCT0_SIZE_R {
        DCACHE_PRELOCK_SCT0_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOCK_SCT_SIZE")
            .field("dcache_prelock_sct1_size", &self.dcache_prelock_sct1_size())
            .field("dcache_prelock_sct0_size", &self.dcache_prelock_sct0_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the second length of data locking, which is combined with DCACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct1_size(
        &mut self,
    ) -> DCACHE_PRELOCK_SCT1_SIZE_W<DCACHE_PRELOCK_SCT_SIZE_SPEC> {
        DCACHE_PRELOCK_SCT1_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The bits are used to configure the first length of data locking, which is combined with DCACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn dcache_prelock_sct0_size(
        &mut self,
    ) -> DCACHE_PRELOCK_SCT0_SIZE_W<DCACHE_PRELOCK_SCT_SIZE_SPEC> {
        DCACHE_PRELOCK_SCT0_SIZE_W::new(self, 16)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_prelock_sct_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_prelock_sct_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOCK_SCT_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_prelock_sct_size::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOCK_SCT_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_PRELOCK_SCT_SIZE to value 0"]
impl crate::Resettable for DCACHE_PRELOCK_SCT_SIZE_SPEC {}
