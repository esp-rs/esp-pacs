#[doc = "Register `ROOT_CLK_CONF` reader"]
pub type R = crate::R<ROOT_CLK_CONF_SPEC>;
#[doc = "Register `ROOT_CLK_CONF` writer"]
pub type W = crate::W<ROOT_CLK_CONF_SPEC>;
#[doc = "Field `SLOW_CLK_SEL` reader - need_des"]
pub type SLOW_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `SLOW_CLK_SEL` writer - need_des"]
pub type SLOW_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST_CLK_SEL` reader - need_des"]
pub type FAST_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `FAST_CLK_SEL` writer - need_des"]
pub type FAST_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL80M_CLK_FORCE_ON` reader - need_des"]
pub type PLL80M_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `PLL80M_CLK_FORCE_ON` writer - need_des"]
pub type PLL80M_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_CLK_FORCE_ON` reader - need_des"]
pub type XTAL_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `XTAL_CLK_FORCE_ON` writer - need_des"]
pub type XTAL_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOSC_CLK_FORCE_ON` reader - need_des"]
pub type FOSC_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `FOSC_CLK_FORCE_ON` writer - need_des"]
pub type FOSC_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_SEL_REF_PLL8M` reader - need_des"]
pub type ANA_SEL_REF_PLL8M_R = crate::BitReader;
#[doc = "Field `ANA_SEL_REF_PLL8M` writer - need_des"]
pub type ANA_SEL_REF_PLL8M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn slow_clk_sel(&self) -> SLOW_CLK_SEL_R {
        SLOW_CLK_SEL_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn fast_clk_sel(&self) -> FAST_CLK_SEL_R {
        FAST_CLK_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn pll80m_clk_force_on(&self) -> PLL80M_CLK_FORCE_ON_R {
        PLL80M_CLK_FORCE_ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xtal_clk_force_on(&self) -> XTAL_CLK_FORCE_ON_R {
        XTAL_CLK_FORCE_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn fosc_clk_force_on(&self) -> FOSC_CLK_FORCE_ON_R {
        FOSC_CLK_FORCE_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ana_sel_ref_pll8m(&self) -> ANA_SEL_REF_PLL8M_R {
        ANA_SEL_REF_PLL8M_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROOT_CLK_CONF")
            .field("slow_clk_sel", &self.slow_clk_sel())
            .field("fast_clk_sel", &self.fast_clk_sel())
            .field("pll80m_clk_force_on", &self.pll80m_clk_force_on())
            .field("xtal_clk_force_on", &self.xtal_clk_force_on())
            .field("fosc_clk_force_on", &self.fosc_clk_force_on())
            .field("ana_sel_ref_pll8m", &self.ana_sel_ref_pll8m())
            .finish()
    }
}
impl W {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn slow_clk_sel(&mut self) -> SLOW_CLK_SEL_W<'_, ROOT_CLK_CONF_SPEC> {
        SLOW_CLK_SEL_W::new(self, 23)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn fast_clk_sel(&mut self) -> FAST_CLK_SEL_W<'_, ROOT_CLK_CONF_SPEC> {
        FAST_CLK_SEL_W::new(self, 26)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn pll80m_clk_force_on(&mut self) -> PLL80M_CLK_FORCE_ON_W<'_, ROOT_CLK_CONF_SPEC> {
        PLL80M_CLK_FORCE_ON_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xtal_clk_force_on(&mut self) -> XTAL_CLK_FORCE_ON_W<'_, ROOT_CLK_CONF_SPEC> {
        XTAL_CLK_FORCE_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn fosc_clk_force_on(&mut self) -> FOSC_CLK_FORCE_ON_W<'_, ROOT_CLK_CONF_SPEC> {
        FOSC_CLK_FORCE_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ana_sel_ref_pll8m(&mut self) -> ANA_SEL_REF_PLL8M_W<'_, ROOT_CLK_CONF_SPEC> {
        ANA_SEL_REF_PLL8M_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`root_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`root_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROOT_CLK_CONF_SPEC;
impl crate::RegisterSpec for ROOT_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`root_clk_conf::R`](R) reader structure"]
impl crate::Readable for ROOT_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`root_clk_conf::W`](W) writer structure"]
impl crate::Writable for ROOT_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOT_CLK_CONF to value 0x0400_0000"]
impl crate::Resettable for ROOT_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0400_0000;
}
