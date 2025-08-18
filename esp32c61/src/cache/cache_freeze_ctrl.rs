#[doc = "Register `CACHE_FREEZE_CTRL` reader"]
pub type R = crate::R<CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Register `CACHE_FREEZE_CTRL` writer"]
pub type W = crate::W<CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Field `ICACHE0_FREEZE_EN` reader - The bit is used to enable freeze operation on L1-ICache0. It can be cleared by software."]
pub type ICACHE0_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `ICACHE0_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L1-ICache0. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type ICACHE0_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE0_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L1-ICache0 is finished or not. 0: not finished. 1: finished."]
pub type ICACHE0_FREEZE_DONE_R = crate::BitReader;
#[doc = "Field `ICACHE1_FREEZE_EN` reader - The bit is used to enable freeze operation on L1-ICache1. It can be cleared by software."]
pub type ICACHE1_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `ICACHE1_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L1-ICache1. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type ICACHE1_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE1_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L1-ICache1 is finished or not. 0: not finished. 1: finished."]
pub type ICACHE1_FREEZE_DONE_R = crate::BitReader;
#[doc = "Field `ICACHE2_FREEZE_EN` reader - Reserved"]
pub type ICACHE2_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `ICACHE2_FREEZE_MODE` reader - Reserved"]
pub type ICACHE2_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE2_FREEZE_DONE` reader - Reserved"]
pub type ICACHE2_FREEZE_DONE_R = crate::BitReader;
#[doc = "Field `ICACHE3_FREEZE_EN` reader - Reserved"]
pub type ICACHE3_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `ICACHE3_FREEZE_MODE` reader - Reserved"]
pub type ICACHE3_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE3_FREEZE_DONE` reader - Reserved"]
pub type ICACHE3_FREEZE_DONE_R = crate::BitReader;
#[doc = "Field `CACHE_FREEZE_EN` reader - The bit is used to enable freeze operation on L1-Cache. It can be cleared by software."]
pub type CACHE_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `CACHE_FREEZE_EN` writer - The bit is used to enable freeze operation on L1-Cache. It can be cleared by software."]
pub type CACHE_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L1-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type CACHE_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `CACHE_FREEZE_MODE` writer - The bit is used to configure mode of freeze operation L1-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type CACHE_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L1-Cache is finished or not. 0: not finished. 1: finished."]
pub type CACHE_FREEZE_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable freeze operation on L1-ICache0. It can be cleared by software."]
    #[inline(always)]
    pub fn icache0_freeze_en(&self) -> ICACHE0_FREEZE_EN_R {
        ICACHE0_FREEZE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to configure mode of freeze operation L1-ICache0. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn icache0_freeze_mode(&self) -> ICACHE0_FREEZE_MODE_R {
        ICACHE0_FREEZE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to indicate whether freeze operation on L1-ICache0 is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn icache0_freeze_done(&self) -> ICACHE0_FREEZE_DONE_R {
        ICACHE0_FREEZE_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable freeze operation on L1-ICache1. It can be cleared by software."]
    #[inline(always)]
    pub fn icache1_freeze_en(&self) -> ICACHE1_FREEZE_EN_R {
        ICACHE1_FREEZE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to configure mode of freeze operation L1-ICache1. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn icache1_freeze_mode(&self) -> ICACHE1_FREEZE_MODE_R {
        ICACHE1_FREEZE_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to indicate whether freeze operation on L1-ICache1 is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn icache1_freeze_done(&self) -> ICACHE1_FREEZE_DONE_R {
        ICACHE1_FREEZE_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_en(&self) -> ICACHE2_FREEZE_EN_R {
        ICACHE2_FREEZE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_mode(&self) -> ICACHE2_FREEZE_MODE_R {
        ICACHE2_FREEZE_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_done(&self) -> ICACHE2_FREEZE_DONE_R {
        ICACHE2_FREEZE_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn icache3_freeze_en(&self) -> ICACHE3_FREEZE_EN_R {
        ICACHE3_FREEZE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn icache3_freeze_mode(&self) -> ICACHE3_FREEZE_MODE_R {
        ICACHE3_FREEZE_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn icache3_freeze_done(&self) -> ICACHE3_FREEZE_DONE_R {
        ICACHE3_FREEZE_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to enable freeze operation on L1-Cache. It can be cleared by software."]
    #[inline(always)]
    pub fn cache_freeze_en(&self) -> CACHE_FREEZE_EN_R {
        CACHE_FREEZE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to configure mode of freeze operation L1-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn cache_freeze_mode(&self) -> CACHE_FREEZE_MODE_R {
        CACHE_FREEZE_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to indicate whether freeze operation on L1-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn cache_freeze_done(&self) -> CACHE_FREEZE_DONE_R {
        CACHE_FREEZE_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_FREEZE_CTRL")
            .field("icache0_freeze_en", &self.icache0_freeze_en())
            .field("icache0_freeze_mode", &self.icache0_freeze_mode())
            .field("icache0_freeze_done", &self.icache0_freeze_done())
            .field("icache1_freeze_en", &self.icache1_freeze_en())
            .field("icache1_freeze_mode", &self.icache1_freeze_mode())
            .field("icache1_freeze_done", &self.icache1_freeze_done())
            .field("icache2_freeze_en", &self.icache2_freeze_en())
            .field("icache2_freeze_mode", &self.icache2_freeze_mode())
            .field("icache2_freeze_done", &self.icache2_freeze_done())
            .field("icache3_freeze_en", &self.icache3_freeze_en())
            .field("icache3_freeze_mode", &self.icache3_freeze_mode())
            .field("icache3_freeze_done", &self.icache3_freeze_done())
            .field("cache_freeze_en", &self.cache_freeze_en())
            .field("cache_freeze_mode", &self.cache_freeze_mode())
            .field("cache_freeze_done", &self.cache_freeze_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - The bit is used to enable freeze operation on L1-Cache. It can be cleared by software."]
    #[inline(always)]
    pub fn cache_freeze_en(&mut self) -> CACHE_FREEZE_EN_W<'_, CACHE_FREEZE_CTRL_SPEC> {
        CACHE_FREEZE_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to configure mode of freeze operation L1-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn cache_freeze_mode(&mut self) -> CACHE_FREEZE_MODE_W<'_, CACHE_FREEZE_CTRL_SPEC> {
        CACHE_FREEZE_MODE_W::new(self, 17)
    }
}
#[doc = "Cache Freeze control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_freeze_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_freeze_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_FREEZE_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_FREEZE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_freeze_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_FREEZE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_freeze_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_FREEZE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_FREEZE_CTRL to value 0"]
impl crate::Resettable for CACHE_FREEZE_CTRL_SPEC {}
