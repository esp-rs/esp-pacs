#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "Field `SAMPLE_ENABLE` reader - reserved"]
pub type SAMPLE_ENABLE_R = crate::BitReader;
#[doc = "Field `SAMPLE_ENABLE` writer - reserved"]
pub type SAMPLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_PSCALE` reader - configure rng timer clk div"]
pub type TIMER_PSCALE_R = crate::FieldReader;
#[doc = "Field `TIMER_PSCALE` writer - configure rng timer clk div"]
pub type TIMER_PSCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TIMER_EN` reader - enable rng xor async rng timer"]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - enable rng xor async rng timer"]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TIMER_EN` reader - reserved"]
pub type RTC_TIMER_EN_R = crate::FieldReader;
#[doc = "Field `RTC_TIMER_EN` writer - reserved"]
pub type RTC_TIMER_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAMPLE_CNT` reader - reserved"]
pub type SAMPLE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn sample_enable(&self) -> SAMPLE_ENABLE_R {
        SAMPLE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - configure rng timer clk div"]
    #[inline(always)]
    pub fn timer_pscale(&self) -> TIMER_PSCALE_R {
        TIMER_PSCALE_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - enable rng xor async rng timer"]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn rtc_timer_en(&self) -> RTC_TIMER_EN_R {
        RTC_TIMER_EN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn sample_cnt(&self) -> SAMPLE_CNT_R {
        SAMPLE_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("sample_enable", &self.sample_enable())
            .field("timer_pscale", &self.timer_pscale())
            .field("timer_en", &self.timer_en())
            .field("rtc_timer_en", &self.rtc_timer_en())
            .field("sample_cnt", &self.sample_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn sample_enable(&mut self) -> SAMPLE_ENABLE_W<'_, CFG_SPEC> {
        SAMPLE_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - configure rng timer clk div"]
    #[inline(always)]
    pub fn timer_pscale(&mut self) -> TIMER_PSCALE_W<'_, CFG_SPEC> {
        TIMER_PSCALE_W::new(self, 1)
    }
    #[doc = "Bit 9 - enable rng xor async rng timer"]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TIMER_EN_W<'_, CFG_SPEC> {
        TIMER_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn rtc_timer_en(&mut self) -> RTC_TIMER_EN_W<'_, CFG_SPEC> {
        RTC_TIMER_EN_W::new(self, 10)
    }
}
#[doc = "configure rng register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x0ffe"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: u32 = 0x0ffe;
}
