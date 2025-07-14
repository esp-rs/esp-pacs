#[doc = "Register `RNG_CFG` reader"]
pub type R = crate::R<RNG_CFG_SPEC>;
#[doc = "Register `RNG_CFG` writer"]
pub type W = crate::W<RNG_CFG_SPEC>;
#[doc = "Field `RNG_SAMPLE_ENABLE` reader - need des"]
pub type RNG_SAMPLE_ENABLE_R = crate::BitReader;
#[doc = "Field `RNG_SAMPLE_ENABLE` writer - need des"]
pub type RNG_SAMPLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_TIMER_PSCALE` reader - need des"]
pub type RNG_TIMER_PSCALE_R = crate::FieldReader;
#[doc = "Field `RNG_TIMER_PSCALE` writer - need des"]
pub type RNG_TIMER_PSCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RNG_TIMER_EN` reader - need des"]
pub type RNG_TIMER_EN_R = crate::BitReader;
#[doc = "Field `RNG_TIMER_EN` writer - need des"]
pub type RNG_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TIMER_EN` reader - need des"]
pub type RTC_TIMER_EN_R = crate::FieldReader;
#[doc = "Field `RTC_TIMER_EN` writer - need des"]
pub type RTC_TIMER_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RNG_SAMPLE_CNT` reader - need des"]
pub type RNG_SAMPLE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn rng_sample_enable(&self) -> RNG_SAMPLE_ENABLE_R {
        RNG_SAMPLE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - need des"]
    #[inline(always)]
    pub fn rng_timer_pscale(&self) -> RNG_TIMER_PSCALE_R {
        RNG_TIMER_PSCALE_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - need des"]
    #[inline(always)]
    pub fn rng_timer_en(&self) -> RNG_TIMER_EN_R {
        RNG_TIMER_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - need des"]
    #[inline(always)]
    pub fn rtc_timer_en(&self) -> RTC_TIMER_EN_R {
        RTC_TIMER_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 24:31 - need des"]
    #[inline(always)]
    pub fn rng_sample_cnt(&self) -> RNG_SAMPLE_CNT_R {
        RNG_SAMPLE_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_CFG")
            .field("rng_sample_enable", &self.rng_sample_enable())
            .field("rng_timer_pscale", &self.rng_timer_pscale())
            .field("rng_timer_en", &self.rng_timer_en())
            .field("rtc_timer_en", &self.rtc_timer_en())
            .field("rng_sample_cnt", &self.rng_sample_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn rng_sample_enable(&mut self) -> RNG_SAMPLE_ENABLE_W<RNG_CFG_SPEC> {
        RNG_SAMPLE_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - need des"]
    #[inline(always)]
    pub fn rng_timer_pscale(&mut self) -> RNG_TIMER_PSCALE_W<RNG_CFG_SPEC> {
        RNG_TIMER_PSCALE_W::new(self, 1)
    }
    #[doc = "Bit 9 - need des"]
    #[inline(always)]
    pub fn rng_timer_en(&mut self) -> RNG_TIMER_EN_W<RNG_CFG_SPEC> {
        RNG_TIMER_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - need des"]
    #[inline(always)]
    pub fn rtc_timer_en(&mut self) -> RTC_TIMER_EN_W<RNG_CFG_SPEC> {
        RTC_TIMER_EN_W::new(self, 10)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_CFG_SPEC;
impl crate::RegisterSpec for RNG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_cfg::R`](R) reader structure"]
impl crate::Readable for RNG_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_cfg::W`](W) writer structure"]
impl crate::Writable for RNG_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_CFG to value 0x0ffe"]
impl crate::Resettable for RNG_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0ffe;
}
