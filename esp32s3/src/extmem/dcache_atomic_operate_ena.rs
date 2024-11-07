#[doc = "Register `DCACHE_ATOMIC_OPERATE_ENA` reader"]
pub type R = crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "Register `DCACHE_ATOMIC_OPERATE_ENA` writer"]
pub type W = crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
#[doc = "Field `DCACHE_ATOMIC_OPERATE_ENA` reader - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type DCACHE_ATOMIC_OPERATE_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_ATOMIC_OPERATE_ENA` writer - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type DCACHE_ATOMIC_OPERATE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn dcache_atomic_operate_ena(&self) -> DCACHE_ATOMIC_OPERATE_ENA_R {
        DCACHE_ATOMIC_OPERATE_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_ATOMIC_OPERATE_ENA")
            .field(
                "dcache_atomic_operate_ena",
                &self.dcache_atomic_operate_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn dcache_atomic_operate_ena(
        &mut self,
    ) -> DCACHE_ATOMIC_OPERATE_ENA_W<DCACHE_ATOMIC_OPERATE_ENA_SPEC> {
        DCACHE_ATOMIC_OPERATE_ENA_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_atomic_operate_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_atomic_operate_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_ATOMIC_OPERATE_ENA_SPEC;
impl crate::RegisterSpec for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_atomic_operate_ena::R`](R) reader structure"]
impl crate::Readable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_atomic_operate_ena::W`](W) writer structure"]
impl crate::Writable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_ATOMIC_OPERATE_ENA to value 0x01"]
impl crate::Resettable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
