#[doc = "Register `L2_CACHE_PRELOCK_CONF` reader"]
pub type R = crate::R<L2_CACHE_PRELOCK_CONF_SPEC>;
#[doc = "Register `L2_CACHE_PRELOCK_CONF` writer"]
pub type W = crate::W<L2_CACHE_PRELOCK_CONF_SPEC>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L2-Cache."]
pub type L2_CACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function on L2-Cache."]
pub type L2_CACHE_PRELOCK_SCT0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L2-Cache."]
pub type L2_CACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function on L2-Cache."]
pub type L2_CACHE_PRELOCK_SCT1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_PRELOCK_RGID` reader - The bit is used to set the gid of l2 cache prelock."]
pub type L2_CACHE_PRELOCK_RGID_R = crate::FieldReader;
#[doc = "Field `L2_CACHE_PRELOCK_RGID` writer - The bit is used to set the gid of l2 cache prelock."]
pub type L2_CACHE_PRELOCK_RGID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_prelock_sct0_en(&self) -> L2_CACHE_PRELOCK_SCT0_EN_R {
        L2_CACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_prelock_sct1_en(&self) -> L2_CACHE_PRELOCK_SCT1_EN_R {
        L2_CACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l2 cache prelock."]
    #[inline(always)]
    pub fn l2_cache_prelock_rgid(&self) -> L2_CACHE_PRELOCK_RGID_R {
        L2_CACHE_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PRELOCK_CONF")
            .field("l2_cache_prelock_sct0_en", &self.l2_cache_prelock_sct0_en())
            .field("l2_cache_prelock_sct1_en", &self.l2_cache_prelock_sct1_en())
            .field("l2_cache_prelock_rgid", &self.l2_cache_prelock_rgid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_prelock_sct0_en(
        &mut self,
    ) -> L2_CACHE_PRELOCK_SCT0_EN_W<L2_CACHE_PRELOCK_CONF_SPEC> {
        L2_CACHE_PRELOCK_SCT0_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_prelock_sct1_en(
        &mut self,
    ) -> L2_CACHE_PRELOCK_SCT1_EN_W<L2_CACHE_PRELOCK_CONF_SPEC> {
        L2_CACHE_PRELOCK_SCT1_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l2 cache prelock."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_prelock_rgid(&mut self) -> L2_CACHE_PRELOCK_RGID_W<L2_CACHE_PRELOCK_CONF_SPEC> {
        L2_CACHE_PRELOCK_RGID_W::new(self, 2)
    }
}
#[doc = "L2 Cache prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_prelock_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_prelock_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_prelock_conf::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOCK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_prelock_conf::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_PRELOCK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOCK_CONF to value 0"]
impl crate::Resettable for L2_CACHE_PRELOCK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
