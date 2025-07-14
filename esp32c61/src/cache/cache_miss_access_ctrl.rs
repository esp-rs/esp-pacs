#[doc = "Register `CACHE_MISS_ACCESS_CTRL` reader"]
pub type R = crate::R<CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Register `CACHE_MISS_ACCESS_CTRL` writer"]
pub type W = crate::W<CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Field `ICACHE0_MISS_DISABLE_ACCESS` reader - Set this bit as 1 to disable early restart of L1-ICache0"]
pub type ICACHE0_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `ICACHE1_MISS_DISABLE_ACCESS` reader - Set this bit as 1 to disable early restart of L1-ICache1"]
pub type ICACHE1_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `ICACHE2_MISS_DISABLE_ACCESS` reader - Reserved"]
pub type ICACHE2_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `ICACHE3_MISS_DISABLE_ACCESS` reader - Reserved"]
pub type ICACHE3_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `CACHE_MISS_DISABLE_ACCESS` reader - Set this bit as 1 to disable early restart of L1-DCache"]
pub type CACHE_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `CACHE_MISS_DISABLE_ACCESS` writer - Set this bit as 1 to disable early restart of L1-DCache"]
pub type CACHE_MISS_DISABLE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to disable early restart of L1-ICache0"]
    #[inline(always)]
    pub fn icache0_miss_disable_access(&self) -> ICACHE0_MISS_DISABLE_ACCESS_R {
        ICACHE0_MISS_DISABLE_ACCESS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit as 1 to disable early restart of L1-ICache1"]
    #[inline(always)]
    pub fn icache1_miss_disable_access(&self) -> ICACHE1_MISS_DISABLE_ACCESS_R {
        ICACHE1_MISS_DISABLE_ACCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_miss_disable_access(&self) -> ICACHE2_MISS_DISABLE_ACCESS_R {
        ICACHE2_MISS_DISABLE_ACCESS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_miss_disable_access(&self) -> ICACHE3_MISS_DISABLE_ACCESS_R {
        ICACHE3_MISS_DISABLE_ACCESS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit as 1 to disable early restart of L1-DCache"]
    #[inline(always)]
    pub fn cache_miss_disable_access(&self) -> CACHE_MISS_DISABLE_ACCESS_R {
        CACHE_MISS_DISABLE_ACCESS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MISS_ACCESS_CTRL")
            .field(
                "icache0_miss_disable_access",
                &self.icache0_miss_disable_access(),
            )
            .field(
                "icache1_miss_disable_access",
                &self.icache1_miss_disable_access(),
            )
            .field(
                "icache2_miss_disable_access",
                &self.icache2_miss_disable_access(),
            )
            .field(
                "icache3_miss_disable_access",
                &self.icache3_miss_disable_access(),
            )
            .field(
                "cache_miss_disable_access",
                &self.cache_miss_disable_access(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Set this bit as 1 to disable early restart of L1-DCache"]
    #[inline(always)]
    pub fn cache_miss_disable_access(
        &mut self,
    ) -> CACHE_MISS_DISABLE_ACCESS_W<CACHE_MISS_ACCESS_CTRL_SPEC> {
        CACHE_MISS_DISABLE_ACCESS_W::new(self, 4)
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_miss_access_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_miss_access_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_MISS_ACCESS_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_MISS_ACCESS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_miss_access_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_MISS_ACCESS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_miss_access_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_MISS_ACCESS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_MISS_ACCESS_CTRL to value 0"]
impl crate::Resettable for CACHE_MISS_ACCESS_CTRL_SPEC {}
