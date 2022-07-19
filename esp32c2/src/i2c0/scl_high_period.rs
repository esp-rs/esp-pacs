#[doc = "Register `SCL_HIGH_PERIOD` reader"]
pub struct R(crate::R<SCL_HIGH_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_HIGH_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_HIGH_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_HIGH_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_HIGH_PERIOD` writer"]
pub struct W(crate::W<SCL_HIGH_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_HIGH_PERIOD_SPEC>;
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
impl From<crate::W<SCL_HIGH_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_HIGH_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_HIGH_PERIOD` reader - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
pub type SCL_HIGH_PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCL_HIGH_PERIOD` writer - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
pub type SCL_HIGH_PERIOD_W<'a> = crate::FieldWriter<'a, u32, SCL_HIGH_PERIOD_SPEC, u16, u16, 9, 0>;
#[doc = "Field `SCL_WAIT_HIGH_PERIOD` reader - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
pub type SCL_WAIT_HIGH_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCL_WAIT_HIGH_PERIOD` writer - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
pub type SCL_WAIT_HIGH_PERIOD_W<'a> =
    crate::FieldWriter<'a, u32, SCL_HIGH_PERIOD_SPEC, u8, u8, 7, 9>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn scl_high_period(&self) -> SCL_HIGH_PERIOD_R {
        SCL_HIGH_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn scl_wait_high_period(&self) -> SCL_WAIT_HIGH_PERIOD_R {
        SCL_WAIT_HIGH_PERIOD_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL setup to high level and remains high in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn scl_high_period(&mut self) -> SCL_HIGH_PERIOD_W {
        SCL_HIGH_PERIOD_W::new(self)
    }
    #[doc = "Bits 9:15 - This register is used to configure for the SCL_FSM's waiting period for SCL high level in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn scl_wait_high_period(&mut self) -> SCL_WAIT_HIGH_PERIOD_W {
        SCL_WAIT_HIGH_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the high level width of SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_high_period](index.html) module"]
pub struct SCL_HIGH_PERIOD_SPEC;
impl crate::RegisterSpec for SCL_HIGH_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_high_period::R](R) reader structure"]
impl crate::Readable for SCL_HIGH_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_high_period::W](W) writer structure"]
impl crate::Writable for SCL_HIGH_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_HIGH_PERIOD to value 0"]
impl crate::Resettable for SCL_HIGH_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
