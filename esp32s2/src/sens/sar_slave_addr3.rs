#[doc = "Register `SAR_SLAVE_ADDR3` reader"]
pub struct R(crate::R<SAR_SLAVE_ADDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_SLAVE_ADDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_SLAVE_ADDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_SLAVE_ADDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_SLAVE_ADDR3` writer"]
pub struct W(crate::W<SAR_SLAVE_ADDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_SLAVE_ADDR3_SPEC>;
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
impl From<crate::W<SAR_SLAVE_ADDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_SLAVE_ADDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C_SLAVE_ADDR5` reader - RTC I2C slave address 5"]
pub type I2C_SLAVE_ADDR5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `I2C_SLAVE_ADDR5` writer - RTC I2C slave address 5"]
pub type I2C_SLAVE_ADDR5_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_SLAVE_ADDR3_SPEC, 11, O, u16, u16>;
#[doc = "Field `I2C_SLAVE_ADDR4` reader - RTC I2C slave address 4"]
pub type I2C_SLAVE_ADDR4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `I2C_SLAVE_ADDR4` writer - RTC I2C slave address 4"]
pub type I2C_SLAVE_ADDR4_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR_SLAVE_ADDR3_SPEC, 11, O, u16, u16>;
impl R {
    #[doc = "Bits 0:10 - RTC I2C slave address 5"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&self) -> I2C_SLAVE_ADDR5_R {
        I2C_SLAVE_ADDR5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 4"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&self) -> I2C_SLAVE_ADDR4_R {
        I2C_SLAVE_ADDR4_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR3")
            .field(
                "i2c_slave_addr5",
                &format_args!("{}", self.i2c_slave_addr5().bits()),
            )
            .field(
                "i2c_slave_addr4",
                &format_args!("{}", self.i2c_slave_addr4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_SLAVE_ADDR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - RTC I2C slave address 5"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr5(&mut self) -> I2C_SLAVE_ADDR5_W<0> {
        I2C_SLAVE_ADDR5_W::new(self)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr4(&mut self) -> I2C_SLAVE_ADDR4_W<11> {
        I2C_SLAVE_ADDR4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure slave addresses 4-5 of RTC I2C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_slave_addr3](index.html) module"]
pub struct SAR_SLAVE_ADDR3_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_slave_addr3::R](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_slave_addr3::W](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR3 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
