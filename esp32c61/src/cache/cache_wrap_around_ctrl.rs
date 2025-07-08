#[doc = "Register `CACHE_WRAP_AROUND_CTRL` reader"]
pub type R = crate::R<CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Register `CACHE_WRAP_AROUND_CTRL` writer"]
pub type W = crate::W<CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Field `ICACHE0_WRAP` reader - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
pub type ICACHE0_WRAP_R = crate::BitReader;
#[doc = "Field `ICACHE1_WRAP` reader - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
pub type ICACHE1_WRAP_R = crate::BitReader;
#[doc = "Field `ICACHE2_WRAP` reader - Reserved"]
pub type ICACHE2_WRAP_R = crate::BitReader;
#[doc = "Field `ICACHE3_WRAP` reader - Reserved"]
pub type ICACHE3_WRAP_R = crate::BitReader;
#[doc = "Field `CACHE_WRAP` reader - Set this bit as 1 to enable L1-DCache wrap around mode."]
pub type CACHE_WRAP_R = crate::BitReader;
#[doc = "Field `CACHE_WRAP` writer - Set this bit as 1 to enable L1-DCache wrap around mode."]
pub type CACHE_WRAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to enable L1-ICache0 wrap around mode."]
    #[inline(always)]
    pub fn icache0_wrap(&self) -> ICACHE0_WRAP_R {
        ICACHE0_WRAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit as 1 to enable L1-ICache1 wrap around mode."]
    #[inline(always)]
    pub fn icache1_wrap(&self) -> ICACHE1_WRAP_R {
        ICACHE1_WRAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_wrap(&self) -> ICACHE2_WRAP_R {
        ICACHE2_WRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_wrap(&self) -> ICACHE3_WRAP_R {
        ICACHE3_WRAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit as 1 to enable L1-DCache wrap around mode."]
    #[inline(always)]
    pub fn cache_wrap(&self) -> CACHE_WRAP_R {
        CACHE_WRAP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_WRAP_AROUND_CTRL")
            .field("icache0_wrap", &self.icache0_wrap())
            .field("icache1_wrap", &self.icache1_wrap())
            .field("icache2_wrap", &self.icache2_wrap())
            .field("icache3_wrap", &self.icache3_wrap())
            .field("cache_wrap", &self.cache_wrap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Set this bit as 1 to enable L1-DCache wrap around mode."]
    #[inline(always)]
    pub fn cache_wrap(&mut self) -> CACHE_WRAP_W<CACHE_WRAP_AROUND_CTRL_SPEC> {
        CACHE_WRAP_W::new(self, 4)
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_wrap_around_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_wrap_around_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_wrap_around_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_WRAP_AROUND_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_wrap_around_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_WRAP_AROUND_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for CACHE_WRAP_AROUND_CTRL_SPEC {}
