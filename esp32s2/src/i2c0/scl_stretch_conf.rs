#[doc = "Register `SCL_STRETCH_CONF` reader"]
pub struct R(crate::R<SCL_STRETCH_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_STRETCH_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_STRETCH_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_STRETCH_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_STRETCH_CONF` writer"]
pub struct W(crate::W<SCL_STRETCH_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_STRETCH_CONF_SPEC>;
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
impl From<crate::W<SCL_STRETCH_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_STRETCH_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRETCH_PROTECT_NUM` reader - Configure the period of I2C slave stretching SCL line."]
pub type STRETCH_PROTECT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `STRETCH_PROTECT_NUM` writer - Configure the period of I2C slave stretching SCL line."]
pub type STRETCH_PROTECT_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, SCL_STRETCH_CONF_SPEC, 10, O, u16>;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` reader - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
pub type SLAVE_SCL_STRETCH_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_SCL_STRETCH_EN` writer - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
pub type SLAVE_SCL_STRETCH_EN_W<'a, const O: u8> = crate::BitWriter<'a, SCL_STRETCH_CONF_SPEC, O>;
#[doc = "Field `SLAVE_SCL_STRETCH_CLR` writer - Set this bit to clear the I2C slave SCL stretch function."]
pub type SLAVE_SCL_STRETCH_CLR_W<'a, const O: u8> = crate::BitWriter<'a, SCL_STRETCH_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - Configure the period of I2C slave stretching SCL line."]
    #[inline(always)]
    pub fn stretch_protect_num(&self) -> STRETCH_PROTECT_NUM_R {
        STRETCH_PROTECT_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
    #[inline(always)]
    pub fn slave_scl_stretch_en(&self) -> SLAVE_SCL_STRETCH_EN_R {
        SLAVE_SCL_STRETCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_STRETCH_CONF")
            .field(
                "stretch_protect_num",
                &format_args!("{}", self.stretch_protect_num().bits()),
            )
            .field(
                "slave_scl_stretch_en",
                &format_args!("{}", self.slave_scl_stretch_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_STRETCH_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - Configure the period of I2C slave stretching SCL line."]
    #[inline(always)]
    #[must_use]
    pub fn stretch_protect_num(&mut self) -> STRETCH_PROTECT_NUM_W<0> {
        STRETCH_PROTECT_NUM_W::new(self)
    }
    #[doc = "Bit 10 - The enable bit for slave SCL stretch function. 1: Enable. 0: Disable. The SCL output line will be stretched low when I2C_SLAVE_SCL_STRETCH_EN is 1 and stretch event happens. The stretch cause can be seen in I2C_STRETCH_CAUSE."]
    #[inline(always)]
    #[must_use]
    pub fn slave_scl_stretch_en(&mut self) -> SLAVE_SCL_STRETCH_EN_W<10> {
        SLAVE_SCL_STRETCH_EN_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the I2C slave SCL stretch function."]
    #[inline(always)]
    #[must_use]
    pub fn slave_scl_stretch_clr(&mut self) -> SLAVE_SCL_STRETCH_CLR_W<11> {
        SLAVE_SCL_STRETCH_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set SCL stretch of I2C slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_stretch_conf](index.html) module"]
pub struct SCL_STRETCH_CONF_SPEC;
impl crate::RegisterSpec for SCL_STRETCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_stretch_conf::R](R) reader structure"]
impl crate::Readable for SCL_STRETCH_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_stretch_conf::W](W) writer structure"]
impl crate::Writable for SCL_STRETCH_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCL_STRETCH_CONF to value 0"]
impl crate::Resettable for SCL_STRETCH_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
