#[doc = "Register `L2_CACHE_PRELOCK_SCT_SIZE` reader"]
pub type R = crate::R<L2_CACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Register `L2_CACHE_PRELOCK_SCT_SIZE` writer"]
pub type W = crate::W<L2_CACHE_PRELOCK_SCT_SIZE_SPEC>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT0_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT0_ADDR_REG"]
pub type L2_CACHE_PRELOCK_SCT0_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT0_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT0_ADDR_REG"]
pub type L2_CACHE_PRELOCK_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_SIZE` reader - Those bits are used to configure the size of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_ADDR_REG"]
pub type L2_CACHE_PRELOCK_SCT1_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_SIZE` writer - Those bits are used to configure the size of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_ADDR_REG"]
pub type L2_CACHE_PRELOCK_SCT1_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    pub fn l2_cache_prelock_sct0_size(&self) -> L2_CACHE_PRELOCK_SCT0_SIZE_R {
        L2_CACHE_PRELOCK_SCT0_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Those bits are used to configure the size of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    pub fn l2_cache_prelock_sct1_size(&self) -> L2_CACHE_PRELOCK_SCT1_SIZE_R {
        L2_CACHE_PRELOCK_SCT1_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PRELOCK_SCT_SIZE")
            .field(
                "l2_cache_prelock_sct0_size",
                &format_args!("{}", self.l2_cache_prelock_sct0_size().bits()),
            )
            .field(
                "l2_cache_prelock_sct1_size",
                &format_args!("{}", self.l2_cache_prelock_sct1_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_PRELOCK_SCT_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Those bits are used to configure the size of the first section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT0_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_prelock_sct0_size(
        &mut self,
    ) -> L2_CACHE_PRELOCK_SCT0_SIZE_W<L2_CACHE_PRELOCK_SCT_SIZE_SPEC> {
        L2_CACHE_PRELOCK_SCT0_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Those bits are used to configure the size of the second section of prelock on L2-Cache, which should be used together with L2_CACHE_PRELOCK_SCT1_ADDR_REG"]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_prelock_sct1_size(
        &mut self,
    ) -> L2_CACHE_PRELOCK_SCT1_SIZE_W<L2_CACHE_PRELOCK_SCT_SIZE_SPEC> {
        L2_CACHE_PRELOCK_SCT1_SIZE_W::new(self, 16)
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
#[doc = "L2 Cache prelock section size configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_prelock_sct_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_prelock_sct_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_PRELOCK_SCT_SIZE_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOCK_SCT_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_prelock_sct_size::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOCK_SCT_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_prelock_sct_size::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_PRELOCK_SCT_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOCK_SCT_SIZE to value 0xffff_ffff"]
impl crate::Resettable for L2_CACHE_PRELOCK_SCT_SIZE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
