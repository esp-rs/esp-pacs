#[doc = "Register `L1_DCACHE_PRELOCK_CONF` reader"]
pub type R = crate::R<L1_DCACHE_PRELOCK_CONF_SPEC>;
#[doc = "Register `L1_DCACHE_PRELOCK_CONF` writer"]
pub type W = crate::W<L1_DCACHE_PRELOCK_CONF_SPEC>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-DCache."]
pub type L1_DCACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function on L1-DCache."]
pub type L1_DCACHE_PRELOCK_SCT0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-DCache."]
pub type L1_DCACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function on L1-DCache."]
pub type L1_DCACHE_PRELOCK_SCT1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_DCACHE_PRELOCK_RGID` reader - The bit is used to set the gid of l1 dcache prelock."]
pub type L1_DCACHE_PRELOCK_RGID_R = crate::FieldReader;
#[doc = "Field `L1_DCACHE_PRELOCK_RGID` writer - The bit is used to set the gid of l1 dcache prelock."]
pub type L1_DCACHE_PRELOCK_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct0_en(&self) -> L1_DCACHE_PRELOCK_SCT0_EN_R {
        L1_DCACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-DCache."]
    #[inline(always)]
    pub fn l1_dcache_prelock_sct1_en(&self) -> L1_DCACHE_PRELOCK_SCT1_EN_R {
        L1_DCACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 dcache prelock."]
    #[inline(always)]
    pub fn l1_dcache_prelock_rgid(&self) -> L1_DCACHE_PRELOCK_RGID_R {
        L1_DCACHE_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_DCACHE_PRELOCK_CONF")
            .field(
                "l1_dcache_prelock_sct0_en",
                &format_args!("{}", self.l1_dcache_prelock_sct0_en().bit()),
            )
            .field(
                "l1_dcache_prelock_sct1_en",
                &format_args!("{}", self.l1_dcache_prelock_sct1_en().bit()),
            )
            .field(
                "l1_dcache_prelock_rgid",
                &format_args!("{}", self.l1_dcache_prelock_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_DCACHE_PRELOCK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_prelock_sct0_en(
        &mut self,
    ) -> L1_DCACHE_PRELOCK_SCT0_EN_W<L1_DCACHE_PRELOCK_CONF_SPEC> {
        L1_DCACHE_PRELOCK_SCT0_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-DCache."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_prelock_sct1_en(
        &mut self,
    ) -> L1_DCACHE_PRELOCK_SCT1_EN_W<L1_DCACHE_PRELOCK_CONF_SPEC> {
        L1_DCACHE_PRELOCK_SCT1_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 dcache prelock."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_prelock_rgid(
        &mut self,
    ) -> L1_DCACHE_PRELOCK_RGID_W<L1_DCACHE_PRELOCK_CONF_SPEC> {
        L1_DCACHE_PRELOCK_RGID_W::new(self, 2)
    }
}
#[doc = "L1 data Cache prelock configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_prelock_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_prelock_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_DCACHE_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for L1_DCACHE_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dcache_prelock_conf::R`](R) reader structure"]
impl crate::Readable for L1_DCACHE_PRELOCK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_dcache_prelock_conf::W`](W) writer structure"]
impl crate::Writable for L1_DCACHE_PRELOCK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_DCACHE_PRELOCK_CONF to value 0"]
impl crate::Resettable for L1_DCACHE_PRELOCK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
