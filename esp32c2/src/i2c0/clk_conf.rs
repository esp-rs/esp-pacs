#[doc = "Register `CLK_CONF` reader"]
pub struct R(crate::R<CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONF` writer"]
pub struct W(crate::W<CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLK_DIV_NUM` reader - the integral part of the fractional divisor for i2c module"]
pub type SCLK_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_NUM` writer - the integral part of the fractional divisor for i2c module"]
pub type SCLK_DIV_NUM_W<'a> = crate::FieldWriter<'a, u32, CLK_CONF_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SCLK_DIV_A` reader - the numerator of the fractional part of the fractional divisor for i2c module"]
pub type SCLK_DIV_A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_A` writer - the numerator of the fractional part of the fractional divisor for i2c module"]
pub type SCLK_DIV_A_W<'a> = crate::FieldWriter<'a, u32, CLK_CONF_SPEC, u8, u8, 6, 8>;
#[doc = "Field `SCLK_DIV_B` reader - the denominator of the fractional part of the fractional divisor for i2c module"]
pub type SCLK_DIV_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCLK_DIV_B` writer - the denominator of the fractional part of the fractional divisor for i2c module"]
pub type SCLK_DIV_B_W<'a> = crate::FieldWriter<'a, u32, CLK_CONF_SPEC, u8, u8, 6, 14>;
#[doc = "Field `SCLK_SEL` reader - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
pub type SCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_SEL` writer - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
pub type SCLK_SEL_W<'a> = crate::BitWriter<'a, u32, CLK_CONF_SPEC, bool, 20>;
#[doc = "Field `SCLK_ACTIVE` reader - The clock switch for i2c module"]
pub type SCLK_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_ACTIVE` writer - The clock switch for i2c module"]
pub type SCLK_ACTIVE_W<'a> = crate::BitWriter<'a, u32, CLK_CONF_SPEC, bool, 21>;
impl R {
    #[doc = "Bits 0:7 - the integral part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - the numerator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19 - the denominator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The clock switch for i2c module"]
    #[inline(always)]
    pub fn sclk_active(&self) -> SCLK_ACTIVE_R {
        SCLK_ACTIVE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - the integral part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W {
        SCLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:13 - the numerator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W {
        SCLK_DIV_A_W::new(self)
    }
    #[doc = "Bits 14:19 - the denominator of the fractional part of the fractional divisor for i2c module"]
    #[inline(always)]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W {
        SCLK_DIV_B_W::new(self)
    }
    #[doc = "Bit 20 - The clock selection for i2c module:0-XTAL,1-CLK_8MHz."]
    #[inline(always)]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W {
        SCLK_SEL_W::new(self)
    }
    #[doc = "Bit 21 - The clock switch for i2c module"]
    #[inline(always)]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W {
        SCLK_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_conf](index.html) module"]
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_conf::R](R) reader structure"]
impl crate::Readable for CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_conf::W](W) writer structure"]
impl crate::Writable for CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CONF to value 0x0020_0000"]
impl crate::Resettable for CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0000
    }
}
