#[doc = "Register `HP_RNG_CFG` reader"]
pub type R = crate::R<HP_RNG_CFG_SPEC>;
#[doc = "Register `HP_RNG_CFG` writer"]
pub type W = crate::W<HP_RNG_CFG_SPEC>;
#[doc = "Field `HP_RNG_SAMPLE_ENABLE` reader - enable rng sample chain"]
pub type HP_RNG_SAMPLE_ENABLE_R = crate::BitReader;
#[doc = "Field `HP_RNG_SAMPLE_ENABLE` writer - enable rng sample chain"]
pub type HP_RNG_SAMPLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_RNG_CHAIN_CLK_DIV_NUM` reader - chain clk div num to pad for debug"]
pub type HP_RNG_CHAIN_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `HP_RNG_CHAIN_CLK_DIV_NUM` writer - chain clk div num to pad for debug"]
pub type HP_RNG_CHAIN_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_RNG_SAMPLE_CNT` reader - debug rng sample cnt"]
pub type HP_RNG_SAMPLE_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - enable rng sample chain"]
    #[inline(always)]
    pub fn hp_rng_sample_enable(&self) -> HP_RNG_SAMPLE_ENABLE_R {
        HP_RNG_SAMPLE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - chain clk div num to pad for debug"]
    #[inline(always)]
    pub fn hp_rng_chain_clk_div_num(&self) -> HP_RNG_CHAIN_CLK_DIV_NUM_R {
        HP_RNG_CHAIN_CLK_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - debug rng sample cnt"]
    #[inline(always)]
    pub fn hp_rng_sample_cnt(&self) -> HP_RNG_SAMPLE_CNT_R {
        HP_RNG_SAMPLE_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_RNG_CFG")
            .field(
                "hp_rng_sample_enable",
                &format_args!("{}", self.hp_rng_sample_enable().bit()),
            )
            .field(
                "hp_rng_chain_clk_div_num",
                &format_args!("{}", self.hp_rng_chain_clk_div_num().bits()),
            )
            .field(
                "hp_rng_sample_cnt",
                &format_args!("{}", self.hp_rng_sample_cnt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_RNG_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - enable rng sample chain"]
    #[inline(always)]
    #[must_use]
    pub fn hp_rng_sample_enable(&mut self) -> HP_RNG_SAMPLE_ENABLE_W<HP_RNG_CFG_SPEC> {
        HP_RNG_SAMPLE_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - chain clk div num to pad for debug"]
    #[inline(always)]
    #[must_use]
    pub fn hp_rng_chain_clk_div_num(&mut self) -> HP_RNG_CHAIN_CLK_DIV_NUM_W<HP_RNG_CFG_SPEC> {
        HP_RNG_CHAIN_CLK_DIV_NUM_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "rng cfg register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_rng_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_rng_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_RNG_CFG_SPEC;
impl crate::RegisterSpec for HP_RNG_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_rng_cfg::R`](R) reader structure"]
impl crate::Readable for HP_RNG_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_rng_cfg::W`](W) writer structure"]
impl crate::Writable for HP_RNG_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_RNG_CFG to value 0"]
impl crate::Resettable for HP_RNG_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
