#[doc = "Register `L1_CACHE_WRAP_AROUND_CTRL` reader"]
pub type R = crate::R<L1_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Register `L1_CACHE_WRAP_AROUND_CTRL` writer"]
pub type W = crate::W<L1_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Field `L1_ICACHE0_WRAP` reader - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
pub type L1_ICACHE0_WRAP_R = crate::BitReader;
#[doc = "Field `L1_ICACHE0_WRAP` writer - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
pub type L1_ICACHE0_WRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE1_WRAP` reader - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
pub type L1_ICACHE1_WRAP_R = crate::BitReader;
#[doc = "Field `L1_ICACHE1_WRAP` writer - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
pub type L1_ICACHE1_WRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE2_WRAP` reader - Reserved"]
pub type L1_ICACHE2_WRAP_R = crate::BitReader;
#[doc = "Field `L1_ICACHE3_WRAP` reader - Reserved"]
pub type L1_ICACHE3_WRAP_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_WRAP` reader - Set this bit as 1 to enable L1-DCache wrap around mode."]
pub type L1_DCACHE_WRAP_R = crate::BitReader;
#[doc = "Field `L1_DCACHE_WRAP` writer - Set this bit as 1 to enable L1-DCache wrap around mode."]
pub type L1_DCACHE_WRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
    #[inline(always)]
    pub fn l1_icache0_wrap(&self) -> L1_ICACHE0_WRAP_R {
        L1_ICACHE0_WRAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
    #[inline(always)]
    pub fn l1_icache1_wrap(&self) -> L1_ICACHE1_WRAP_R {
        L1_ICACHE1_WRAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn l1_icache2_wrap(&self) -> L1_ICACHE2_WRAP_R {
        L1_ICACHE2_WRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn l1_icache3_wrap(&self) -> L1_ICACHE3_WRAP_R {
        L1_ICACHE3_WRAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit as 1 to enable L1-DCache wrap around mode."]
    #[inline(always)]
    pub fn l1_dcache_wrap(&self) -> L1_DCACHE_WRAP_R {
        L1_DCACHE_WRAP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_WRAP_AROUND_CTRL")
            .field("l1_icache0_wrap", &self.l1_icache0_wrap())
            .field("l1_icache1_wrap", &self.l1_icache1_wrap())
            .field("l1_icache2_wrap", &self.l1_icache2_wrap())
            .field("l1_icache3_wrap", &self.l1_icache3_wrap())
            .field("l1_dcache_wrap", &self.l1_dcache_wrap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache0_wrap(&mut self) -> L1_ICACHE0_WRAP_W<L1_CACHE_WRAP_AROUND_CTRL_SPEC> {
        L1_ICACHE0_WRAP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
    #[inline(always)]
    #[must_use]
    pub fn l1_icache1_wrap(&mut self) -> L1_ICACHE1_WRAP_W<L1_CACHE_WRAP_AROUND_CTRL_SPEC> {
        L1_ICACHE1_WRAP_W::new(self, 1)
    }
    #[doc = "Bit 4 - Set this bit as 1 to enable L1-DCache wrap around mode."]
    #[inline(always)]
    #[must_use]
    pub fn l1_dcache_wrap(&mut self) -> L1_DCACHE_WRAP_W<L1_CACHE_WRAP_AROUND_CTRL_SPEC> {
        L1_DCACHE_WRAP_W::new(self, 4)
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_wrap_around_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_wrap_around_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for L1_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_wrap_around_ctrl::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_WRAP_AROUND_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_wrap_around_ctrl::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for L1_CACHE_WRAP_AROUND_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
