#[doc = "Register `I2C_SCL_LOW_PERIOD` reader"]
pub struct R(crate::R<I2C_SCL_LOW_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SCL_LOW_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SCL_LOW_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SCL_LOW_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_SCL_LOW_PERIOD` writer"]
pub struct W(crate::W<I2C_SCL_LOW_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_SCL_LOW_PERIOD_SPEC>;
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
impl From<crate::W<I2C_SCL_LOW_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_SCL_LOW_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SCL_LOW_PERIOD` reader - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
pub type I2C_SCL_LOW_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SCL_LOW_PERIOD` writer - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
pub type I2C_SCL_LOW_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, I2C_SCL_LOW_PERIOD_SPEC, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_low_period(&self) -> I2C_SCL_LOW_PERIOD_R {
        I2C_SCL_LOW_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SCL_LOW_PERIOD")
            .field(
                "i2c_scl_low_period",
                &format_args!("{}", self.i2c_scl_low_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_SCL_LOW_PERIOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_low_period(&mut self) -> I2C_SCL_LOW_PERIOD_W<0> {
        I2C_SCL_LOW_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the low level width of the SCL Clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_low_period](index.html) module"]
pub struct I2C_SCL_LOW_PERIOD_SPEC;
impl crate::RegisterSpec for I2C_SCL_LOW_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_scl_low_period::R](R) reader structure"]
impl crate::Readable for I2C_SCL_LOW_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_scl_low_period::W](W) writer structure"]
impl crate::Writable for I2C_SCL_LOW_PERIOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_SCL_LOW_PERIOD to value 0"]
impl crate::Resettable for I2C_SCL_LOW_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
