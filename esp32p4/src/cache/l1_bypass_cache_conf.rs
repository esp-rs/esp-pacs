#[doc = "Register `L1_BYPASS_CACHE_CONF` reader"]
pub type R = crate::R<L1_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Register `L1_BYPASS_CACHE_CONF` writer"]
pub type W = crate::W<L1_BYPASS_CACHE_CONF_SPEC>;
#[doc = "Field `BYPASS_L1_ICACHE0_EN` reader - The bit is used to enable bypass L1-ICache0. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_ICACHE0_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_ICACHE0_EN` writer - The bit is used to enable bypass L1-ICache0. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_ICACHE0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_L1_ICACHE1_EN` reader - The bit is used to enable bypass L1-ICache1. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_ICACHE1_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_ICACHE1_EN` writer - The bit is used to enable bypass L1-ICache1. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_ICACHE1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS_L1_ICACHE2_EN` reader - Reserved"]
pub type BYPASS_L1_ICACHE2_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_ICACHE3_EN` reader - Reserved"]
pub type BYPASS_L1_ICACHE3_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_DCACHE_EN` reader - The bit is used to enable bypass L1-DCache. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_DCACHE_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_DCACHE_EN` writer - The bit is used to enable bypass L1-DCache. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_DCACHE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable bypass L1-ICache0. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l1_icache0_en(&self) -> BYPASS_L1_ICACHE0_EN_R {
        BYPASS_L1_ICACHE0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable bypass L1-ICache1. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l1_icache1_en(&self) -> BYPASS_L1_ICACHE1_EN_R {
        BYPASS_L1_ICACHE1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn bypass_l1_icache2_en(&self) -> BYPASS_L1_ICACHE2_EN_R {
        BYPASS_L1_ICACHE2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn bypass_l1_icache3_en(&self) -> BYPASS_L1_ICACHE3_EN_R {
        BYPASS_L1_ICACHE3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable bypass L1-DCache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l1_dcache_en(&self) -> BYPASS_L1_DCACHE_EN_R {
        BYPASS_L1_DCACHE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_BYPASS_CACHE_CONF")
            .field("bypass_l1_icache0_en", &self.bypass_l1_icache0_en())
            .field("bypass_l1_icache1_en", &self.bypass_l1_icache1_en())
            .field("bypass_l1_icache2_en", &self.bypass_l1_icache2_en())
            .field("bypass_l1_icache3_en", &self.bypass_l1_icache3_en())
            .field("bypass_l1_dcache_en", &self.bypass_l1_dcache_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable bypass L1-ICache0. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_l1_icache0_en(&mut self) -> BYPASS_L1_ICACHE0_EN_W<L1_BYPASS_CACHE_CONF_SPEC> {
        BYPASS_L1_ICACHE0_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable bypass L1-ICache1. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_l1_icache1_en(&mut self) -> BYPASS_L1_ICACHE1_EN_W<L1_BYPASS_CACHE_CONF_SPEC> {
        BYPASS_L1_ICACHE1_EN_W::new(self, 1)
    }
    #[doc = "Bit 4 - The bit is used to enable bypass L1-DCache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_l1_dcache_en(&mut self) -> BYPASS_L1_DCACHE_EN_W<L1_BYPASS_CACHE_CONF_SPEC> {
        BYPASS_L1_DCACHE_EN_W::new(self, 4)
    }
}
#[doc = "Bypass Cache configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_bypass_cache_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_bypass_cache_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_BYPASS_CACHE_CONF_SPEC;
impl crate::RegisterSpec for L1_BYPASS_CACHE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_bypass_cache_conf::R`](R) reader structure"]
impl crate::Readable for L1_BYPASS_CACHE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_bypass_cache_conf::W`](W) writer structure"]
impl crate::Writable for L1_BYPASS_CACHE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_BYPASS_CACHE_CONF to value 0"]
impl crate::Resettable for L1_BYPASS_CACHE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
