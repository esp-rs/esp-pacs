#[doc = "Register `I2C_SCL_HIGH_PERIOD` reader"]
pub struct R(crate::R<I2C_SCL_HIGH_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_SCL_HIGH_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_SCL_HIGH_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_SCL_HIGH_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_SCL_HIGH_PERIOD` writer"]
pub struct W(crate::W<I2C_SCL_HIGH_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_SCL_HIGH_PERIOD_SPEC>;
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
impl From<crate::W<I2C_SCL_HIGH_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_SCL_HIGH_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SCL_HIGH_PERIOD` reader - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
pub type I2C_SCL_HIGH_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SCL_HIGH_PERIOD` writer - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
pub type I2C_SCL_HIGH_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, I2C_SCL_HIGH_PERIOD_SPEC, 9, O, u16>;
#[doc = "Field `I2C_SCL_WAIT_HIGH_PERIOD` reader - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
pub type I2C_SCL_WAIT_HIGH_PERIOD_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_WAIT_HIGH_PERIOD` writer - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
pub type I2C_SCL_WAIT_HIGH_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, I2C_SCL_HIGH_PERIOD_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_high_period(&self) -> I2C_SCL_HIGH_PERIOD_R {
        I2C_SCL_HIGH_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_wait_high_period(&self) -> I2C_SCL_WAIT_HIGH_PERIOD_R {
        I2C_SCL_WAIT_HIGH_PERIOD_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SCL_HIGH_PERIOD")
            .field(
                "i2c_scl_high_period",
                &format_args!("{}", self.i2c_scl_high_period().bits()),
            )
            .field(
                "i2c_scl_wait_high_period",
                &format_args!("{}", self.i2c_scl_wait_high_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_SCL_HIGH_PERIOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_high_period(&mut self) -> I2C_SCL_HIGH_PERIOD_W<0> {
        I2C_SCL_HIGH_PERIOD_W::new(self)
    }
    #[doc = "Bits 9:15 - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_wait_high_period(&mut self) -> I2C_SCL_WAIT_HIGH_PERIOD_W<9> {
        I2C_SCL_WAIT_HIGH_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the high level width of SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_scl_high_period](index.html) module"]
pub struct I2C_SCL_HIGH_PERIOD_SPEC;
impl crate::RegisterSpec for I2C_SCL_HIGH_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_scl_high_period::R](R) reader structure"]
impl crate::Readable for I2C_SCL_HIGH_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_scl_high_period::W](W) writer structure"]
impl crate::Writable for I2C_SCL_HIGH_PERIOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_SCL_HIGH_PERIOD to value 0"]
impl crate::Resettable for I2C_SCL_HIGH_PERIOD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
