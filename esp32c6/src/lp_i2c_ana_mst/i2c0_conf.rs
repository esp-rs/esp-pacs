#[doc = "Register `I2C0_CONF` reader"]
pub struct R(crate::R<I2C0_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_CONF` writer"]
pub struct W(crate::W<I2C0_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_CONF_SPEC>;
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
impl From<crate::W<I2C0_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CONF` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CONF_R = crate::FieldReader<u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CONF` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CONF_W<'a, const O: u8> =
    crate::FieldWriter<'a, I2C0_CONF_SPEC, 24, O, u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_STATUS` reader - reserved"]
pub type LP_I2C_ANA_MAST_I2C0_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_conf(&self) -> LP_I2C_ANA_MAST_I2C0_CONF_R {
        LP_I2C_ANA_MAST_I2C0_CONF_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_status(&self) -> LP_I2C_ANA_MAST_I2C0_STATUS_R {
        LP_I2C_ANA_MAST_I2C0_STATUS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CONF")
            .field(
                "lp_i2c_ana_mast_i2c0_conf",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_conf().bits()),
            )
            .field(
                "lp_i2c_ana_mast_i2c0_status",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_status().bits()),
            )
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
    #[doc = "Bits 0:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_conf(&mut self) -> LP_I2C_ANA_MAST_I2C0_CONF_W<0> {
        LP_I2C_ANA_MAST_I2C0_CONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_conf](index.html) module"]
pub struct I2C0_CONF_SPEC;
impl crate::RegisterSpec for I2C0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_conf::R](R) reader structure"]
impl crate::Readable for I2C0_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_conf::W](W) writer structure"]
impl crate::Writable for I2C0_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_CONF to value 0x0700_0000"]
impl crate::Resettable for I2C0_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0000;
}
