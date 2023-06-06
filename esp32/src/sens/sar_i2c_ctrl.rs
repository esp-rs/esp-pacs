#[doc = "Register `SAR_I2C_CTRL` reader"]
pub struct R(crate::R<SAR_I2C_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_I2C_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_I2C_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_I2C_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_I2C_CTRL` writer"]
pub struct W(crate::W<SAR_I2C_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_I2C_CTRL_SPEC>;
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
impl From<crate::W<SAR_I2C_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_I2C_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR_I2C_CTRL` reader - I2C control data only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_I2C_CTRL` writer - I2C control data only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_I2C_CTRL_SPEC, 28, O, u32>;
#[doc = "Field `SAR_I2C_START` reader - start I2C only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_START_R = crate::BitReader;
#[doc = "Field `SAR_I2C_START` writer - start I2C only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_START_W<'a, const O: u8> = crate::BitWriter<'a, SAR_I2C_CTRL_SPEC, O>;
#[doc = "Field `SAR_I2C_START_FORCE` reader - 1: I2C started by SW 0: I2C started by FSM"]
pub type SAR_I2C_START_FORCE_R = crate::BitReader;
#[doc = "Field `SAR_I2C_START_FORCE` writer - 1: I2C started by SW 0: I2C started by FSM"]
pub type SAR_I2C_START_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_I2C_CTRL_SPEC, O>;
impl R {
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_ctrl(&self) -> SAR_I2C_CTRL_R {
        SAR_I2C_CTRL_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_start(&self) -> SAR_I2C_START_R {
        SAR_I2C_START_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    pub fn sar_i2c_start_force(&self) -> SAR_I2C_START_FORCE_R {
        SAR_I2C_START_FORCE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_I2C_CTRL")
            .field(
                "sar_i2c_ctrl",
                &format_args!("{}", self.sar_i2c_ctrl().bits()),
            )
            .field(
                "sar_i2c_start",
                &format_args!("{}", self.sar_i2c_start().bit()),
            )
            .field(
                "sar_i2c_start_force",
                &format_args!("{}", self.sar_i2c_start_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_I2C_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_ctrl(&mut self) -> SAR_I2C_CTRL_W<0> {
        SAR_I2C_CTRL_W::new(self)
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_start(&mut self) -> SAR_I2C_START_W<28> {
        SAR_I2C_START_W::new(self)
    }
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_start_force(&mut self) -> SAR_I2C_START_FORCE_W<29> {
        SAR_I2C_START_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_i2c_ctrl](index.html) module"]
pub struct SAR_I2C_CTRL_SPEC;
impl crate::RegisterSpec for SAR_I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_i2c_ctrl::R](R) reader structure"]
impl crate::Readable for SAR_I2C_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_i2c_ctrl::W](W) writer structure"]
impl crate::Writable for SAR_I2C_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_I2C_CTRL to value 0"]
impl crate::Resettable for SAR_I2C_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
