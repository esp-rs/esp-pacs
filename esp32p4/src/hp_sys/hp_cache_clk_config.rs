#[doc = "Register `HP_CACHE_CLK_CONFIG` reader"]
pub type R = crate::R<HP_CACHE_CLK_CONFIG_SPEC>;
#[doc = "Register `HP_CACHE_CLK_CONFIG` writer"]
pub type W = crate::W<HP_CACHE_CLK_CONFIG_SPEC>;
#[doc = "Field `HP_REG_L2_CACHE_CLK_ON` reader - l2 cahce clk enable"]
pub type HP_REG_L2_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `HP_REG_L2_CACHE_CLK_ON` writer - l2 cahce clk enable"]
pub type HP_REG_L2_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L1_D_CACHE_CLK_ON` reader - l1 dcahce clk enable"]
pub type HP_REG_L1_D_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `HP_REG_L1_D_CACHE_CLK_ON` writer - l1 dcahce clk enable"]
pub type HP_REG_L1_D_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L1_I1_CACHE_CLK_ON` reader - l1 icahce1 clk enable"]
pub type HP_REG_L1_I1_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `HP_REG_L1_I1_CACHE_CLK_ON` writer - l1 icahce1 clk enable"]
pub type HP_REG_L1_I1_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_REG_L1_I0_CACHE_CLK_ON` reader - l1 icahce0 clk enable"]
pub type HP_REG_L1_I0_CACHE_CLK_ON_R = crate::BitReader;
#[doc = "Field `HP_REG_L1_I0_CACHE_CLK_ON` writer - l1 icahce0 clk enable"]
pub type HP_REG_L1_I0_CACHE_CLK_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - l2 cahce clk enable"]
    #[inline(always)]
    pub fn hp_reg_l2_cache_clk_on(&self) -> HP_REG_L2_CACHE_CLK_ON_R {
        HP_REG_L2_CACHE_CLK_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - l1 dcahce clk enable"]
    #[inline(always)]
    pub fn hp_reg_l1_d_cache_clk_on(&self) -> HP_REG_L1_D_CACHE_CLK_ON_R {
        HP_REG_L1_D_CACHE_CLK_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - l1 icahce1 clk enable"]
    #[inline(always)]
    pub fn hp_reg_l1_i1_cache_clk_on(&self) -> HP_REG_L1_I1_CACHE_CLK_ON_R {
        HP_REG_L1_I1_CACHE_CLK_ON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - l1 icahce0 clk enable"]
    #[inline(always)]
    pub fn hp_reg_l1_i0_cache_clk_on(&self) -> HP_REG_L1_I0_CACHE_CLK_ON_R {
        HP_REG_L1_I0_CACHE_CLK_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CACHE_CLK_CONFIG")
            .field(
                "hp_reg_l2_cache_clk_on",
                &format_args!("{}", self.hp_reg_l2_cache_clk_on().bit()),
            )
            .field(
                "hp_reg_l1_d_cache_clk_on",
                &format_args!("{}", self.hp_reg_l1_d_cache_clk_on().bit()),
            )
            .field(
                "hp_reg_l1_i1_cache_clk_on",
                &format_args!("{}", self.hp_reg_l1_i1_cache_clk_on().bit()),
            )
            .field(
                "hp_reg_l1_i0_cache_clk_on",
                &format_args!("{}", self.hp_reg_l1_i0_cache_clk_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CACHE_CLK_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - l2 cahce clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l2_cache_clk_on(&mut self) -> HP_REG_L2_CACHE_CLK_ON_W<HP_CACHE_CLK_CONFIG_SPEC> {
        HP_REG_L2_CACHE_CLK_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - l1 dcahce clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l1_d_cache_clk_on(
        &mut self,
    ) -> HP_REG_L1_D_CACHE_CLK_ON_W<HP_CACHE_CLK_CONFIG_SPEC> {
        HP_REG_L1_D_CACHE_CLK_ON_W::new(self, 1)
    }
    #[doc = "Bit 4 - l1 icahce1 clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l1_i1_cache_clk_on(
        &mut self,
    ) -> HP_REG_L1_I1_CACHE_CLK_ON_W<HP_CACHE_CLK_CONFIG_SPEC> {
        HP_REG_L1_I1_CACHE_CLK_ON_W::new(self, 4)
    }
    #[doc = "Bit 5 - l1 icahce0 clk enable"]
    #[inline(always)]
    #[must_use]
    pub fn hp_reg_l1_i0_cache_clk_on(
        &mut self,
    ) -> HP_REG_L1_I0_CACHE_CLK_ON_W<HP_CACHE_CLK_CONFIG_SPEC> {
        HP_REG_L1_I0_CACHE_CLK_ON_W::new(self, 5)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cache_clk_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cache_clk_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CACHE_CLK_CONFIG_SPEC;
impl crate::RegisterSpec for HP_CACHE_CLK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cache_clk_config::R`](R) reader structure"]
impl crate::Readable for HP_CACHE_CLK_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cache_clk_config::W`](W) writer structure"]
impl crate::Writable for HP_CACHE_CLK_CONFIG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CACHE_CLK_CONFIG to value 0x33"]
impl crate::Resettable for HP_CACHE_CLK_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x33;
}
