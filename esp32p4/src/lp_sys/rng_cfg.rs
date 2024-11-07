#[doc = "Register `RNG_CFG` reader"]
pub type R = crate::R<RNG_CFG_SPEC>;
#[doc = "Register `RNG_CFG` writer"]
pub type W = crate::W<RNG_CFG_SPEC>;
#[doc = "Field `RNG_TIMER_EN` reader - enable rng timer"]
pub type RNG_TIMER_EN_R = crate::BitReader;
#[doc = "Field `RNG_TIMER_EN` writer - enable rng timer"]
pub type RNG_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_TIMER_PSCALE` reader - configure ng timer pscale"]
pub type RNG_TIMER_PSCALE_R = crate::FieldReader;
#[doc = "Field `RNG_TIMER_PSCALE` writer - configure ng timer pscale"]
pub type RNG_TIMER_PSCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RNG_SAR_ENABLE` reader - enable rng_saradc"]
pub type RNG_SAR_ENABLE_R = crate::BitReader;
#[doc = "Field `RNG_SAR_ENABLE` writer - enable rng_saradc"]
pub type RNG_SAR_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_SAR_DATA` reader - debug rng sar sample cnt"]
pub type RNG_SAR_DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - enable rng timer"]
    #[inline(always)]
    pub fn rng_timer_en(&self) -> RNG_TIMER_EN_R {
        RNG_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - configure ng timer pscale"]
    #[inline(always)]
    pub fn rng_timer_pscale(&self) -> RNG_TIMER_PSCALE_R {
        RNG_TIMER_PSCALE_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - enable rng_saradc"]
    #[inline(always)]
    pub fn rng_sar_enable(&self) -> RNG_SAR_ENABLE_R {
        RNG_SAR_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:28 - debug rng sar sample cnt"]
    #[inline(always)]
    pub fn rng_sar_data(&self) -> RNG_SAR_DATA_R {
        RNG_SAR_DATA_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG_CFG")
            .field("rng_timer_en", &self.rng_timer_en())
            .field("rng_timer_pscale", &self.rng_timer_pscale())
            .field("rng_sar_enable", &self.rng_sar_enable())
            .field("rng_sar_data", &self.rng_sar_data())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable rng timer"]
    #[inline(always)]
    pub fn rng_timer_en(&mut self) -> RNG_TIMER_EN_W<RNG_CFG_SPEC> {
        RNG_TIMER_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - configure ng timer pscale"]
    #[inline(always)]
    pub fn rng_timer_pscale(&mut self) -> RNG_TIMER_PSCALE_W<RNG_CFG_SPEC> {
        RNG_TIMER_PSCALE_W::new(self, 1)
    }
    #[doc = "Bit 9 - enable rng_saradc"]
    #[inline(always)]
    pub fn rng_sar_enable(&mut self) -> RNG_SAR_ENABLE_W<RNG_CFG_SPEC> {
        RNG_SAR_ENABLE_W::new(self, 9)
    }
}
#[doc = "rng cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNG_CFG_SPEC;
impl crate::RegisterSpec for RNG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_cfg::R`](R) reader structure"]
impl crate::Readable for RNG_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rng_cfg::W`](W) writer structure"]
impl crate::Writable for RNG_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNG_CFG to value 0x03"]
impl crate::Resettable for RNG_CFG_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
