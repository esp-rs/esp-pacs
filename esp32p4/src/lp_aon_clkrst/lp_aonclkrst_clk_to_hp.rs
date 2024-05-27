#[doc = "Register `LP_AONCLKRST_CLK_TO_HP` reader"]
pub type R = crate::R<LP_AONCLKRST_CLK_TO_HP_SPEC>;
#[doc = "Register `LP_AONCLKRST_CLK_TO_HP` writer"]
pub type W = crate::W<LP_AONCLKRST_CLK_TO_HP_SPEC>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_XTAL32K` reader - reserved"]
pub type LP_AONCLKRST_ICG_HP_XTAL32K_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_XTAL32K` writer - reserved"]
pub type LP_AONCLKRST_ICG_HP_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_SOSC` reader - reserved"]
pub type LP_AONCLKRST_ICG_HP_SOSC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_SOSC` writer - reserved"]
pub type LP_AONCLKRST_ICG_HP_SOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_OSC32K` reader - reserved"]
pub type LP_AONCLKRST_ICG_HP_OSC32K_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_OSC32K` writer - reserved"]
pub type LP_AONCLKRST_ICG_HP_OSC32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ICG_HP_FOSC` reader - reserved"]
pub type LP_AONCLKRST_ICG_HP_FOSC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ICG_HP_FOSC` writer - reserved"]
pub type LP_AONCLKRST_ICG_HP_FOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_xtal32k(&self) -> LP_AONCLKRST_ICG_HP_XTAL32K_R {
        LP_AONCLKRST_ICG_HP_XTAL32K_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_sosc(&self) -> LP_AONCLKRST_ICG_HP_SOSC_R {
        LP_AONCLKRST_ICG_HP_SOSC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_osc32k(&self) -> LP_AONCLKRST_ICG_HP_OSC32K_R {
        LP_AONCLKRST_ICG_HP_OSC32K_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_icg_hp_fosc(&self) -> LP_AONCLKRST_ICG_HP_FOSC_R {
        LP_AONCLKRST_ICG_HP_FOSC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_CLK_TO_HP")
            .field(
                "lp_aonclkrst_icg_hp_xtal32k",
                &self.lp_aonclkrst_icg_hp_xtal32k(),
            )
            .field("lp_aonclkrst_icg_hp_sosc", &self.lp_aonclkrst_icg_hp_sosc())
            .field(
                "lp_aonclkrst_icg_hp_osc32k",
                &self.lp_aonclkrst_icg_hp_osc32k(),
            )
            .field("lp_aonclkrst_icg_hp_fosc", &self.lp_aonclkrst_icg_hp_fosc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_icg_hp_xtal32k(
        &mut self,
    ) -> LP_AONCLKRST_ICG_HP_XTAL32K_W<LP_AONCLKRST_CLK_TO_HP_SPEC> {
        LP_AONCLKRST_ICG_HP_XTAL32K_W::new(self, 28)
    }
    #[doc = "Bit 29 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_icg_hp_sosc(
        &mut self,
    ) -> LP_AONCLKRST_ICG_HP_SOSC_W<LP_AONCLKRST_CLK_TO_HP_SPEC> {
        LP_AONCLKRST_ICG_HP_SOSC_W::new(self, 29)
    }
    #[doc = "Bit 30 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_icg_hp_osc32k(
        &mut self,
    ) -> LP_AONCLKRST_ICG_HP_OSC32K_W<LP_AONCLKRST_CLK_TO_HP_SPEC> {
        LP_AONCLKRST_ICG_HP_OSC32K_W::new(self, 30)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_icg_hp_fosc(
        &mut self,
    ) -> LP_AONCLKRST_ICG_HP_FOSC_W<LP_AONCLKRST_CLK_TO_HP_SPEC> {
        LP_AONCLKRST_ICG_HP_FOSC_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_clk_to_hp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_clk_to_hp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_CLK_TO_HP_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_CLK_TO_HP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_clk_to_hp::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_CLK_TO_HP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_clk_to_hp::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_CLK_TO_HP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_CLK_TO_HP to value 0xf000_0000"]
impl crate::Resettable for LP_AONCLKRST_CLK_TO_HP_SPEC {
    const RESET_VALUE: u32 = 0xf000_0000;
}
