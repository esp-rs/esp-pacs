#[doc = "Register `L1_CACHE_MISS_ACCESS_CTRL` reader"]
pub type R = crate::R<L1_CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_MISS_ACCESS_CTRL` writer"]
pub type W = crate::W<L1_CACHE_MISS_ACCESS_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_MISS_DISABLE_ACCESS` reader - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
pub type L1_ICACHE0_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_MISS_DISABLE_ACCESS` writer - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
pub type L1_ICACHE0_MISS_DISABLE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_MISS_DISABLE_ACCESS` reader - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
pub type L1_ICACHE1_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_MISS_DISABLE_ACCESS` writer - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
pub type L1_ICACHE1_MISS_DISABLE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_MISS_DISABLE_ACCESS` reader - Reserved"]
pub type L1_ICACHE2_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `L1_ICACHE2_MISS_DISABLE_ACCESS` writer - Reserved"]
pub type L1_ICACHE2_MISS_DISABLE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE3_MISS_DISABLE_ACCESS` reader - Reserved"]
pub type L1_ICACHE3_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_MISS_DISABLE_ACCESS` writer - Reserved"]
pub type L1_ICACHE3_MISS_DISABLE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_CACHE_MISS_DISABLE_ACCESS` reader - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
pub type L1_CACHE_MISS_DISABLE_ACCESS_R = crate::BitReader;
#[doc = "Field `L1_CACHE_MISS_DISABLE_ACCESS` writer - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
pub type L1_CACHE_MISS_DISABLE_ACCESS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn l1_icache0_miss_disable_access(&self) -> L1_ICACHE0_MISS_DISABLE_ACCESS_R {
        L1_ICACHE0_MISS_DISABLE_ACCESS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn l1_icache1_miss_disable_access(&self) -> L1_ICACHE1_MISS_DISABLE_ACCESS_R {
        L1_ICACHE1_MISS_DISABLE_ACCESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_miss_disable_access(&self) -> L1_ICACHE2_MISS_DISABLE_ACCESS_R {
        L1_ICACHE2_MISS_DISABLE_ACCESS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_miss_disable_access(&self) -> L1_ICACHE3_MISS_DISABLE_ACCESS_R {
        L1_ICACHE3_MISS_DISABLE_ACCESS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn l1_cache_miss_disable_access(&self) -> L1_CACHE_MISS_DISABLE_ACCESS_R {
        L1_CACHE_MISS_DISABLE_ACCESS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_MISS_ACCESS_CTRL")
            .field(
                "l1_icache0_miss_disable_access",
                &self.l1_icache0_miss_disable_access(),
            )
            .field(
                "l1_icache1_miss_disable_access",
                &self.l1_icache1_miss_disable_access(),
            )
            .field(
                "l1_icache2_miss_disable_access",
                &self.l1_icache2_miss_disable_access(),
            )
            .field(
                "l1_icache3_miss_disable_access",
                &self.l1_icache3_miss_disable_access(),
            )
            .field(
                "l1_cache_miss_disable_access",
                &self.l1_cache_miss_disable_access(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn l1_icache0_miss_disable_access(
        &mut self,
    ) -> L1_ICACHE0_MISS_DISABLE_ACCESS_W<'_, L1_CACHE_MISS_ACCESS_CTRL_SPEC> {
        L1_ICACHE0_MISS_DISABLE_ACCESS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn l1_icache1_miss_disable_access(
        &mut self,
    ) -> L1_ICACHE1_MISS_DISABLE_ACCESS_W<'_, L1_CACHE_MISS_ACCESS_CTRL_SPEC> {
        L1_ICACHE1_MISS_DISABLE_ACCESS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_miss_disable_access(
        &mut self,
    ) -> L1_ICACHE2_MISS_DISABLE_ACCESS_W<'_, L1_CACHE_MISS_ACCESS_CTRL_SPEC> {
        L1_ICACHE2_MISS_DISABLE_ACCESS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_miss_disable_access(
        &mut self,
    ) -> L1_ICACHE3_MISS_DISABLE_ACCESS_W<'_, L1_CACHE_MISS_ACCESS_CTRL_SPEC> {
        L1_ICACHE3_MISS_DISABLE_ACCESS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether to enable early restart function.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn l1_cache_miss_disable_access(
        &mut self,
    ) -> L1_CACHE_MISS_DISABLE_ACCESS_W<'_, L1_CACHE_MISS_ACCESS_CTRL_SPEC> {
        L1_CACHE_MISS_DISABLE_ACCESS_W::new(self, 4)
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_miss_access_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_miss_access_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_MISS_ACCESS_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_MISS_ACCESS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_miss_access_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_MISS_ACCESS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_miss_access_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_MISS_ACCESS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_MISS_ACCESS_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_MISS_ACCESS_CTRL_SPEC {}
