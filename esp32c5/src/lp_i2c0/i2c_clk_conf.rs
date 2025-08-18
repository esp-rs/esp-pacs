#[doc = "Register `I2C_CLK_CONF` reader"]
pub type R = crate::R<I2C_CLK_CONF_SPEC>;
#[doc = "Register `I2C_CLK_CONF` writer"]
pub type W = crate::W<I2C_CLK_CONF_SPEC>;
#[doc = "Field `I2C_SCLK_DIV_NUM` reader - the integral part of the fractional divisor for i2c module"]
pub type I2C_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `I2C_SCLK_DIV_NUM` writer - the integral part of the fractional divisor for i2c module"]
pub type I2C_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `I2C_SCLK_DIV_A` reader - the numerator of the fractional part of the fractional divisor for i2c module"]
pub type I2C_SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `I2C_SCLK_DIV_A` writer - the numerator of the fractional part of the fractional divisor for i2c module"]
pub type I2C_SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `I2C_SCLK_DIV_B` reader - the denominator of the fractional part of the fractional divisor for i2c module"]
pub type I2C_SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `I2C_SCLK_DIV_B` writer - the denominator of the fractional part of the fractional divisor for i2c module"]
pub type I2C_SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `I2C_SCLK_SEL` reader - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
pub type I2C_SCLK_SEL_R = crate::BitReader;
#[doc = "Field `I2C_SCLK_SEL` writer - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
pub type I2C_SCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SCLK_ACTIVE` reader - The clock switch for i2c module"]
pub type I2C_SCLK_ACTIVE_R = crate::BitReader;
#[doc = "Field `I2C_SCLK_ACTIVE` writer - The clock switch for i2c module"]
pub type I2C_SCLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - the integral part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_div_num(&self) -> I2C_SCLK_DIV_NUM_R {
        I2C_SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - the numerator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_div_a(&self) -> I2C_SCLK_DIV_A_R {
        I2C_SCLK_DIV_A_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - the denominator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_div_b(&self) -> I2C_SCLK_DIV_B_R {
        I2C_SCLK_DIV_B_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
    #[inline(always)]
    pub fn i2c_sclk_sel(&self) -> I2C_SCLK_SEL_R {
        I2C_SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The clock switch for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_active(&self) -> I2C_SCLK_ACTIVE_R {
        I2C_SCLK_ACTIVE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CLK_CONF")
            .field("i2c_sclk_div_num", &self.i2c_sclk_div_num())
            .field("i2c_sclk_div_a", &self.i2c_sclk_div_a())
            .field("i2c_sclk_div_b", &self.i2c_sclk_div_b())
            .field("i2c_sclk_sel", &self.i2c_sclk_sel())
            .field("i2c_sclk_active", &self.i2c_sclk_active())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - the integral part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_div_num(&mut self) -> I2C_SCLK_DIV_NUM_W<'_, I2C_CLK_CONF_SPEC> {
        I2C_SCLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - the numerator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_div_a(&mut self) -> I2C_SCLK_DIV_A_W<'_, I2C_CLK_CONF_SPEC> {
        I2C_SCLK_DIV_A_W::new(self, 8)
    }
    #[doc = "Bits 14:19 - the denominator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_div_b(&mut self) -> I2C_SCLK_DIV_B_W<'_, I2C_CLK_CONF_SPEC> {
        I2C_SCLK_DIV_B_W::new(self, 14)
    }
    #[doc = "Bit 20 - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
    #[inline(always)]
    pub fn i2c_sclk_sel(&mut self) -> I2C_SCLK_SEL_W<'_, I2C_CLK_CONF_SPEC> {
        I2C_SCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 21 - The clock switch for i2c module"]
    #[inline(always)]
    pub fn i2c_sclk_active(&mut self) -> I2C_SCLK_ACTIVE_W<'_, I2C_CLK_CONF_SPEC> {
        I2C_SCLK_ACTIVE_W::new(self, 21)
    }
}
#[doc = "I2C CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_CLK_CONF_SPEC;
impl crate::RegisterSpec for I2C_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_clk_conf::R`](R) reader structure"]
impl crate::Readable for I2C_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_clk_conf::W`](W) writer structure"]
impl crate::Writable for I2C_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CLK_CONF to value 0x0020_0000"]
impl crate::Resettable for I2C_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
