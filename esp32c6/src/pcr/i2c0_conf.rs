#[doc = "Register `I2C0_CONF` reader"]
pub type R = crate::R<I2C0_CONF_SPEC>;
#[doc = "Register `I2C0_CONF` writer"]
pub type W = crate::W<I2C0_CONF_SPEC>;
#[doc = "Field `I2C0_CLK_EN` reader - Set 1 to enable i2c apb clock"]
pub type I2C0_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2C0_CLK_EN` writer - Set 1 to enable i2c apb clock"]
pub type I2C0_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C0_RST_EN` reader - Set 0 to reset i2c module"]
pub type I2C0_RST_EN_R = crate::BitReader;
#[doc = "Field `I2C0_RST_EN` writer - Set 0 to reset i2c module"]
pub type I2C0_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable i2c apb clock"]
    #[inline(always)]
    pub fn i2c0_clk_en(&self) -> I2C0_CLK_EN_R {
        I2C0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset i2c module"]
    #[inline(always)]
    pub fn i2c0_rst_en(&self) -> I2C0_RST_EN_R {
        I2C0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CONF")
            .field("i2c0_clk_en", &format_args!("{}", self.i2c0_clk_en().bit()))
            .field("i2c0_rst_en", &format_args!("{}", self.i2c0_rst_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C0_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable i2c apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_clk_en(&mut self) -> I2C0_CLK_EN_W<I2C0_CONF_SPEC, 0> {
        I2C0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset i2c module"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_rst_en(&mut self) -> I2C0_RST_EN_W<I2C0_CONF_SPEC, 1> {
        I2C0_RST_EN_W::new(self)
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
#[doc = "I2C configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_CONF_SPEC;
impl crate::RegisterSpec for I2C0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_conf::R`](R) reader structure"]
impl crate::Readable for I2C0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_conf::W`](W) writer structure"]
impl crate::Writable for I2C0_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_CONF to value 0x01"]
impl crate::Resettable for I2C0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
