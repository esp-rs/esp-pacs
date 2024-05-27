///Register `L1_CACHE_FREEZE_CTRL` reader
pub type R = crate::R<L1_CACHE_FREEZE_CTRL_SPEC>;
///Register `L1_CACHE_FREEZE_CTRL` writer
pub type W = crate::W<L1_CACHE_FREEZE_CTRL_SPEC>;
///Field `L1_ICACHE0_FREEZE_EN` reader - The bit is used to enable freeze operation on L1-ICache0. It can be cleared by software.
pub type L1_ICACHE0_FREEZE_EN_R = crate::BitReader;
///Field `L1_ICACHE0_FREEZE_EN` writer - The bit is used to enable freeze operation on L1-ICache0. It can be cleared by software.
pub type L1_ICACHE0_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE0_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L1-ICache0. 0: a miss-access will not stuck. 1: a miss-access will stuck.
pub type L1_ICACHE0_FREEZE_MODE_R = crate::BitReader;
///Field `L1_ICACHE0_FREEZE_MODE` writer - The bit is used to configure mode of freeze operation L1-ICache0. 0: a miss-access will not stuck. 1: a miss-access will stuck.
pub type L1_ICACHE0_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE0_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L1-ICache0 is finished or not. 0: not finished. 1: finished.
pub type L1_ICACHE0_FREEZE_DONE_R = crate::BitReader;
///Field `L1_ICACHE1_FREEZE_EN` reader - The bit is used to enable freeze operation on L1-ICache1. It can be cleared by software.
pub type L1_ICACHE1_FREEZE_EN_R = crate::BitReader;
///Field `L1_ICACHE1_FREEZE_EN` writer - The bit is used to enable freeze operation on L1-ICache1. It can be cleared by software.
pub type L1_ICACHE1_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L1-ICache1. 0: a miss-access will not stuck. 1: a miss-access will stuck.
pub type L1_ICACHE1_FREEZE_MODE_R = crate::BitReader;
///Field `L1_ICACHE1_FREEZE_MODE` writer - The bit is used to configure mode of freeze operation L1-ICache1. 0: a miss-access will not stuck. 1: a miss-access will stuck.
pub type L1_ICACHE1_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_ICACHE1_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L1-ICache1 is finished or not. 0: not finished. 1: finished.
pub type L1_ICACHE1_FREEZE_DONE_R = crate::BitReader;
///Field `L1_ICACHE2_FREEZE_EN` reader - Reserved
pub type L1_ICACHE2_FREEZE_EN_R = crate::BitReader;
///Field `L1_ICACHE2_FREEZE_MODE` reader - Reserved
pub type L1_ICACHE2_FREEZE_MODE_R = crate::BitReader;
///Field `L1_ICACHE2_FREEZE_DONE` reader - Reserved
pub type L1_ICACHE2_FREEZE_DONE_R = crate::BitReader;
///Field `L1_ICACHE3_FREEZE_EN` reader - Reserved
pub type L1_ICACHE3_FREEZE_EN_R = crate::BitReader;
///Field `L1_ICACHE3_FREEZE_MODE` reader - Reserved
pub type L1_ICACHE3_FREEZE_MODE_R = crate::BitReader;
///Field `L1_ICACHE3_FREEZE_DONE` reader - Reserved
pub type L1_ICACHE3_FREEZE_DONE_R = crate::BitReader;
///Field `L1_DCACHE_FREEZE_EN` reader - The bit is used to enable freeze operation on L1-DCache. It can be cleared by software.
pub type L1_DCACHE_FREEZE_EN_R = crate::BitReader;
///Field `L1_DCACHE_FREEZE_EN` writer - The bit is used to enable freeze operation on L1-DCache. It can be cleared by software.
pub type L1_DCACHE_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DCACHE_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L1-DCache. 0: a miss-access will not stuck. 1: a miss-access will stuck.
pub type L1_DCACHE_FREEZE_MODE_R = crate::BitReader;
///Field `L1_DCACHE_FREEZE_MODE` writer - The bit is used to configure mode of freeze operation L1-DCache. 0: a miss-access will not stuck. 1: a miss-access will stuck.
pub type L1_DCACHE_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `L1_DCACHE_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L1-DCache is finished or not. 0: not finished. 1: finished.
pub type L1_DCACHE_FREEZE_DONE_R = crate::BitReader;
impl R {
    ///Bit 0 - The bit is used to enable freeze operation on L1-ICache0. It can be cleared by software.
    #[inline(always)]
    pub fn l1_icache0_freeze_en(&self) -> L1_ICACHE0_FREEZE_EN_R {
        L1_ICACHE0_FREEZE_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to configure mode of freeze operation L1-ICache0. 0: a miss-access will not stuck. 1: a miss-access will stuck.
    #[inline(always)]
    pub fn l1_icache0_freeze_mode(&self) -> L1_ICACHE0_FREEZE_MODE_R {
        L1_ICACHE0_FREEZE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The bit is used to indicate whether freeze operation on L1-ICache0 is finished or not. 0: not finished. 1: finished.
    #[inline(always)]
    pub fn l1_icache0_freeze_done(&self) -> L1_ICACHE0_FREEZE_DONE_R {
        L1_ICACHE0_FREEZE_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - The bit is used to enable freeze operation on L1-ICache1. It can be cleared by software.
    #[inline(always)]
    pub fn l1_icache1_freeze_en(&self) -> L1_ICACHE1_FREEZE_EN_R {
        L1_ICACHE1_FREEZE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The bit is used to configure mode of freeze operation L1-ICache1. 0: a miss-access will not stuck. 1: a miss-access will stuck.
    #[inline(always)]
    pub fn l1_icache1_freeze_mode(&self) -> L1_ICACHE1_FREEZE_MODE_R {
        L1_ICACHE1_FREEZE_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The bit is used to indicate whether freeze operation on L1-ICache1 is finished or not. 0: not finished. 1: finished.
    #[inline(always)]
    pub fn l1_icache1_freeze_done(&self) -> L1_ICACHE1_FREEZE_DONE_R {
        L1_ICACHE1_FREEZE_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Reserved
    #[inline(always)]
    pub fn l1_icache2_freeze_en(&self) -> L1_ICACHE2_FREEZE_EN_R {
        L1_ICACHE2_FREEZE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    pub fn l1_icache2_freeze_mode(&self) -> L1_ICACHE2_FREEZE_MODE_R {
        L1_ICACHE2_FREEZE_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Reserved
    #[inline(always)]
    pub fn l1_icache2_freeze_done(&self) -> L1_ICACHE2_FREEZE_DONE_R {
        L1_ICACHE2_FREEZE_DONE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Reserved
    #[inline(always)]
    pub fn l1_icache3_freeze_en(&self) -> L1_ICACHE3_FREEZE_EN_R {
        L1_ICACHE3_FREEZE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reserved
    #[inline(always)]
    pub fn l1_icache3_freeze_mode(&self) -> L1_ICACHE3_FREEZE_MODE_R {
        L1_ICACHE3_FREEZE_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Reserved
    #[inline(always)]
    pub fn l1_icache3_freeze_done(&self) -> L1_ICACHE3_FREEZE_DONE_R {
        L1_ICACHE3_FREEZE_DONE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - The bit is used to enable freeze operation on L1-DCache. It can be cleared by software.
    #[inline(always)]
    pub fn l1_dcache_freeze_en(&self) -> L1_DCACHE_FREEZE_EN_R {
        L1_DCACHE_FREEZE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - The bit is used to configure mode of freeze operation L1-DCache. 0: a miss-access will not stuck. 1: a miss-access will stuck.
    #[inline(always)]
    pub fn l1_dcache_freeze_mode(&self) -> L1_DCACHE_FREEZE_MODE_R {
        L1_DCACHE_FREEZE_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - The bit is used to indicate whether freeze operation on L1-DCache is finished or not. 0: not finished. 1: finished.
    #[inline(always)]
    pub fn l1_dcache_freeze_done(&self) -> L1_DCACHE_FREEZE_DONE_R {
        L1_DCACHE_FREEZE_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_FREEZE_CTRL")
            .field("l1_icache0_freeze_en", &self.l1_icache0_freeze_en())
            .field("l1_icache0_freeze_mode", &self.l1_icache0_freeze_mode())
            .field("l1_icache0_freeze_done", &self.l1_icache0_freeze_done())
            .field("l1_icache1_freeze_en", &self.l1_icache1_freeze_en())
            .field("l1_icache1_freeze_mode", &self.l1_icache1_freeze_mode())
            .field("l1_icache1_freeze_done", &self.l1_icache1_freeze_done())
            .field("l1_icache2_freeze_en", &self.l1_icache2_freeze_en())
            .field("l1_icache2_freeze_mode", &self.l1_icache2_freeze_mode())
            .field("l1_icache2_freeze_done", &self.l1_icache2_freeze_done())
            .field("l1_icache3_freeze_en", &self.l1_icache3_freeze_en())
            .field("l1_icache3_freeze_mode", &self.l1_icache3_freeze_mode())
            .field("l1_icache3_freeze_done", &self.l1_icache3_freeze_done())
            .field("l1_dcache_freeze_en", &self.l1_dcache_freeze_en())
            .field("l1_dcache_freeze_mode", &self.l1_dcache_freeze_mode())
            .field("l1_dcache_freeze_done", &self.l1_dcache_freeze_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - The bit is used to enable freeze operation on L1-ICache0. It can be cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_freeze_en(&mut self) -> L1_ICACHE0_FREEZE_EN_W<L1_CACHE_FREEZE_CTRL_SPEC> {
        L1_ICACHE0_FREEZE_EN_W::new(self, 0)
    }
    ///Bit 1 - The bit is used to configure mode of freeze operation L1-ICache0. 0: a miss-access will not stuck. 1: a miss-access will stuck.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_freeze_mode(
        &mut self,
    ) -> L1_ICACHE0_FREEZE_MODE_W<L1_CACHE_FREEZE_CTRL_SPEC> {
        L1_ICACHE0_FREEZE_MODE_W::new(self, 1)
    }
    ///Bit 4 - The bit is used to enable freeze operation on L1-ICache1. It can be cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_freeze_en(&mut self) -> L1_ICACHE1_FREEZE_EN_W<L1_CACHE_FREEZE_CTRL_SPEC> {
        L1_ICACHE1_FREEZE_EN_W::new(self, 4)
    }
    ///Bit 5 - The bit is used to configure mode of freeze operation L1-ICache1. 0: a miss-access will not stuck. 1: a miss-access will stuck.
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_freeze_mode(
        &mut self,
    ) -> L1_ICACHE1_FREEZE_MODE_W<L1_CACHE_FREEZE_CTRL_SPEC> {
        L1_ICACHE1_FREEZE_MODE_W::new(self, 5)
    }
    ///Bit 16 - The bit is used to enable freeze operation on L1-DCache. It can be cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_freeze_en(&mut self) -> L1_DCACHE_FREEZE_EN_W<L1_CACHE_FREEZE_CTRL_SPEC> {
        L1_DCACHE_FREEZE_EN_W::new(self, 16)
    }
    ///Bit 17 - The bit is used to configure mode of freeze operation L1-DCache. 0: a miss-access will not stuck. 1: a miss-access will stuck.
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_freeze_mode(&mut self) -> L1_DCACHE_FREEZE_MODE_W<L1_CACHE_FREEZE_CTRL_SPEC> {
        L1_DCACHE_FREEZE_MODE_W::new(self, 17)
    }
}
/**Cache Freeze control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_freeze_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_freeze_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_FREEZE_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_FREEZE_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_freeze_ctrl::R`](R) reader structure
impl crate::Readable for L1_CACHE_FREEZE_CTRL_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_freeze_ctrl::W`](W) writer structure
impl crate::Writable for L1_CACHE_FREEZE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_FREEZE_CTRL to value 0
impl crate::Resettable for L1_CACHE_FREEZE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
