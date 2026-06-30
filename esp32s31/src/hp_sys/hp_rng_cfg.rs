#[doc = "Register `HP_RNG_CFG` reader"]
pub type R = crate::R<HP_RNG_CFG_SPEC>;
#[doc = "Register `HP_RNG_CFG` writer"]
pub type W = crate::W<HP_RNG_CFG_SPEC>;
#[doc = "Field `HP_RNG_CHAIN_CLK_DIV_NUM` reader - chain clk div num to pad for debug"]
pub type HP_RNG_CHAIN_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `HP_RNG_CHAIN_CLK_DIV_NUM` writer - chain clk div num to pad for debug"]
pub type HP_RNG_CHAIN_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - chain clk div num to pad for debug"]
    #[inline(always)]
    pub fn hp_rng_chain_clk_div_num(&self) -> HP_RNG_CHAIN_CLK_DIV_NUM_R {
        HP_RNG_CHAIN_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_RNG_CFG")
            .field("hp_rng_chain_clk_div_num", &self.hp_rng_chain_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - chain clk div num to pad for debug"]
    #[inline(always)]
    pub fn hp_rng_chain_clk_div_num(&mut self) -> HP_RNG_CHAIN_CLK_DIV_NUM_W<'_, HP_RNG_CFG_SPEC> {
        HP_RNG_CHAIN_CLK_DIV_NUM_W::new(self, 16)
    }
}
#[doc = "rng cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_rng_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_rng_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_RNG_CFG_SPEC;
impl crate::RegisterSpec for HP_RNG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_rng_cfg::R`](R) reader structure"]
impl crate::Readable for HP_RNG_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_rng_cfg::W`](W) writer structure"]
impl crate::Writable for HP_RNG_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_RNG_CFG to value 0"]
impl crate::Resettable for HP_RNG_CFG_SPEC {}
