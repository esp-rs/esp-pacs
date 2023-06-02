#[doc = "Register `I2C_TO` reader"]
pub struct R(crate::R<I2C_TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TO` writer"]
pub struct W(crate::W<I2C_TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TO_SPEC>;
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
impl From<crate::W<I2C_TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_TIME_OUT_VALUE` reader - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
pub type I2C_TIME_OUT_VALUE_R = crate::FieldReader;
#[doc = "Field `I2C_TIME_OUT_VALUE` writer - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
pub type I2C_TIME_OUT_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, I2C_TO_SPEC, 5, O>;
#[doc = "Field `I2C_TIME_OUT_EN` reader - This is the enable bit for time out control."]
pub type I2C_TIME_OUT_EN_R = crate::BitReader;
#[doc = "Field `I2C_TIME_OUT_EN` writer - This is the enable bit for time out control."]
pub type I2C_TIME_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, I2C_TO_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
    #[inline(always)]
    pub fn i2c_time_out_value(&self) -> I2C_TIME_OUT_VALUE_R {
        I2C_TIME_OUT_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - This is the enable bit for time out control."]
    #[inline(always)]
    pub fn i2c_time_out_en(&self) -> I2C_TIME_OUT_EN_R {
        I2C_TIME_OUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TO")
            .field(
                "i2c_time_out_value",
                &format_args!("{}", self.i2c_time_out_value().bits()),
            )
            .field(
                "i2c_time_out_en",
                &format_args!("{}", self.i2c_time_out_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_TO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to configure the timeout for receiving a data bit in APB clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_time_out_value(&mut self) -> I2C_TIME_OUT_VALUE_W<0> {
        I2C_TIME_OUT_VALUE_W::new(self)
    }
    #[doc = "Bit 5 - This is the enable bit for time out control."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_time_out_en(&mut self) -> I2C_TIME_OUT_EN_W<5> {
        I2C_TIME_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Setting time out control for receiving data.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_to](index.html) module"]
pub struct I2C_TO_SPEC;
impl crate::RegisterSpec for I2C_TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_to::R](R) reader structure"]
impl crate::Readable for I2C_TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_to::W](W) writer structure"]
impl crate::Writable for I2C_TO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_TO to value 0x10"]
impl crate::Resettable for I2C_TO_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
