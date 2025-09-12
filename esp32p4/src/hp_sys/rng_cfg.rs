#[doc = "Register `RNG_CFG` reader"]
pub type R = crate::R<RNG_CFG_SPEC>;
#[doc = "Register `RNG_CFG` writer"]
pub type W = crate::W<RNG_CFG_SPEC>;
#[doc = "Field `RNG_SAMPLE_ENABLE` reader - enable rng sample chain"]
pub type RNG_SAMPLE_ENABLE_R = crate::BitReader;
#[doc = "Field `RNG_SAMPLE_ENABLE` writer - enable rng sample chain"]
pub type RNG_SAMPLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_CHAIN_CLK_DIV_NUM` reader - chain clk div num to pad for debug"]
pub type RNG_CHAIN_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `RNG_CHAIN_CLK_DIV_NUM` writer - chain clk div num to pad for debug"]
pub type RNG_CHAIN_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RNG_SAMPLE_CNT` reader - debug rng sample cnt"]
pub type RNG_SAMPLE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - enable rng sample chain"]
    #[inline(always)]
    pub fn rng_sample_enable(&self) -> RNG_SAMPLE_ENABLE_R {
        RNG_SAMPLE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - chain clk div num to pad for debug"]
    #[inline(always)]
    pub fn rng_chain_clk_div_num(&self) -> RNG_CHAIN_CLK_DIV_NUM_R {
        RNG_CHAIN_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - debug rng sample cnt"]
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
            .field("rng_chain_clk_div_num", &self.rng_chain_clk_div_num())
            .field("rng_sample_cnt", &self.rng_sample_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable rng sample chain"]
    #[inline(always)]
    pub fn rng_sample_enable(&mut self) -> RNG_SAMPLE_ENABLE_W<'_, RNG_CFG_SPEC> {
        RNG_SAMPLE_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - chain clk div num to pad for debug"]
    #[inline(always)]
    pub fn rng_chain_clk_div_num(&mut self) -> RNG_CHAIN_CLK_DIV_NUM_W<'_, RNG_CFG_SPEC> {
        RNG_CHAIN_CLK_DIV_NUM_W::new(self, 16)
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
}
#[doc = "`reset()` method sets RNG_CFG to value 0"]
impl crate::Resettable for RNG_CFG_SPEC {}
