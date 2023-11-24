#[doc = "Register `CLK_CONF` reader"]
pub type R = crate::R<CLK_CONF_SPEC>;
#[doc = "Register `CLK_CONF` writer"]
pub type W = crate::W<CLK_CONF_SPEC>;
#[doc = "Field `CLK_WIFIPWR_EN` reader - "]
pub type CLK_WIFIPWR_EN_R = crate::BitReader;
#[doc = "Field `CLK_WIFIPWR_EN` writer - "]
pub type CLK_WIFIPWR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_COEX_EN` reader - "]
pub type CLK_COEX_EN_R = crate::BitReader;
#[doc = "Field `CLK_COEX_EN` writer - "]
pub type CLK_COEX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C_MST_EN` reader - "]
pub type CLK_I2C_MST_EN_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_EN` writer - "]
pub type CLK_I2C_MST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_LP_TIMER_EN` reader - "]
pub type CLK_LP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CLK_LP_TIMER_EN` writer - "]
pub type CLK_LP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_wifipwr_en(&self) -> CLK_WIFIPWR_EN_R {
        CLK_WIFIPWR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clk_coex_en(&self) -> CLK_COEX_EN_R {
        CLK_COEX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clk_i2c_mst_en(&self) -> CLK_I2C_MST_EN_R {
        CLK_I2C_MST_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clk_lp_timer_en(&self) -> CLK_LP_TIMER_EN_R {
        CLK_LP_TIMER_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field(
                "clk_wifipwr_en",
                &format_args!("{}", self.clk_wifipwr_en().bit()),
            )
            .field("clk_coex_en", &format_args!("{}", self.clk_coex_en().bit()))
            .field(
                "clk_i2c_mst_en",
                &format_args!("{}", self.clk_i2c_mst_en().bit()),
            )
            .field(
                "clk_lp_timer_en",
                &format_args!("{}", self.clk_lp_timer_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifipwr_en(&mut self) -> CLK_WIFIPWR_EN_W<CLK_CONF_SPEC> {
        CLK_WIFIPWR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_coex_en(&mut self) -> CLK_COEX_EN_W<CLK_CONF_SPEC> {
        CLK_COEX_EN_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_en(&mut self) -> CLK_I2C_MST_EN_W<CLK_CONF_SPEC> {
        CLK_I2C_MST_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_lp_timer_en(&mut self) -> CLK_LP_TIMER_EN_W<CLK_CONF_SPEC> {
        CLK_LP_TIMER_EN_W::new(self, 3)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF to value 0"]
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
