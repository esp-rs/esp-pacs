#[doc = "Register `CACHE_SYNC_RST_CTRL` reader"]
pub type R = crate::R<CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Register `CACHE_SYNC_RST_CTRL` writer"]
pub type W = crate::W<CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Field `ICACHE0_SYNC_RST` reader - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type ICACHE0_SYNC_RST_R = crate::BitReader;
#[doc = "Field `ICACHE1_SYNC_RST` reader - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type ICACHE1_SYNC_RST_R = crate::BitReader;
#[doc = "Field `ICACHE2_SYNC_RST` reader - Reserved"]
pub type ICACHE2_SYNC_RST_R = crate::BitReader;
#[doc = "Field `ICACHE3_SYNC_RST` reader - Reserved"]
pub type ICACHE3_SYNC_RST_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_RST` reader - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type CACHE_SYNC_RST_R = crate::BitReader;
#[doc = "Field `CACHE_SYNC_RST` writer - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type CACHE_SYNC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to reset sync-logic inside L1-ICache0. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn icache0_sync_rst(&self) -> ICACHE0_SYNC_RST_R {
        ICACHE0_SYNC_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set this bit to reset sync-logic inside L1-ICache1. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn icache1_sync_rst(&self) -> ICACHE1_SYNC_RST_R {
        ICACHE1_SYNC_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_sync_rst(&self) -> ICACHE2_SYNC_RST_R {
        ICACHE2_SYNC_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_sync_rst(&self) -> ICACHE3_SYNC_RST_R {
        ICACHE3_SYNC_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn cache_sync_rst(&self) -> CACHE_SYNC_RST_R {
        CACHE_SYNC_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SYNC_RST_CTRL")
            .field("icache0_sync_rst", &self.icache0_sync_rst())
            .field("icache1_sync_rst", &self.icache1_sync_rst())
            .field("icache2_sync_rst", &self.icache2_sync_rst())
            .field("icache3_sync_rst", &self.icache3_sync_rst())
            .field("cache_sync_rst", &self.cache_sync_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - set this bit to reset sync-logic inside L1-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn cache_sync_rst(&mut self) -> CACHE_SYNC_RST_W<CACHE_SYNC_RST_CTRL_SPEC> {
        CACHE_SYNC_RST_W::new(self, 4)
    }
}
#[doc = "Cache Sync Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_sync_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_sync_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SYNC_RST_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_SYNC_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_sync_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_SYNC_RST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_sync_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_SYNC_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_SYNC_RST_CTRL to value 0"]
impl crate::Resettable for CACHE_SYNC_RST_CTRL_SPEC {}
