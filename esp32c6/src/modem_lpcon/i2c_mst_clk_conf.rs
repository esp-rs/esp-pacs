#[doc = "Register `I2C_MST_CLK_CONF` reader"]
pub type R = crate::R<I2C_MST_CLK_CONF_SPEC>;
#[doc = "Register `I2C_MST_CLK_CONF` writer"]
pub type W = crate::W<I2C_MST_CLK_CONF_SPEC>;
#[doc = "Field `CLK_I2C_MST_SEL_160M` reader - "]
pub type CLK_I2C_MST_SEL_160M_R = crate::BitReader;
#[doc = "Field `CLK_I2C_MST_SEL_160M` writer - "]
pub type CLK_I2C_MST_SEL_160M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clk_i2c_mst_sel_160m(&self) -> CLK_I2C_MST_SEL_160M_R {
        CLK_I2C_MST_SEL_160M_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_MST_CLK_CONF")
            .field(
                "clk_i2c_mst_sel_160m",
                &format_args!("{}", self.clk_i2c_mst_sel_160m().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_MST_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c_mst_sel_160m(&mut self) -> CLK_I2C_MST_SEL_160M_W<I2C_MST_CLK_CONF_SPEC> {
        CLK_I2C_MST_SEL_160M_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_mst_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_mst_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_MST_CLK_CONF_SPEC;
impl crate::RegisterSpec for I2C_MST_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_mst_clk_conf::R`](R) reader structure"]
impl crate::Readable for I2C_MST_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_mst_clk_conf::W`](W) writer structure"]
impl crate::Writable for I2C_MST_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_MST_CLK_CONF to value 0"]
impl crate::Resettable for I2C_MST_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
