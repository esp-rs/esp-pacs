#[doc = "Register `ANA_CONF1` reader"]
pub struct R(crate::R<ANA_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CONF1` writer"]
pub struct W(crate::W<ANA_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CONF1_SPEC>;
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
impl From<crate::W<ANA_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_ANA_MAST_ANA_CONF1` reader - need_des"]
pub type LP_I2C_ANA_MAST_ANA_CONF1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LP_I2C_ANA_MAST_ANA_CONF1` writer - need_des"]
pub type LP_I2C_ANA_MAST_ANA_CONF1_W<'a, const O: u8> =
    crate::FieldWriter<'a, ANA_CONF1_SPEC, 24, O, u32, u32>;
impl R {
    #[doc = "Bits 0:23 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_ana_conf1(&self) -> LP_I2C_ANA_MAST_ANA_CONF1_R {
        LP_I2C_ANA_MAST_ANA_CONF1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_CONF1")
            .field(
                "lp_i2c_ana_mast_ana_conf1",
                &format_args!("{}", self.lp_i2c_ana_mast_ana_conf1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ANA_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_ana_conf1(&mut self) -> LP_I2C_ANA_MAST_ANA_CONF1_W<0> {
        LP_I2C_ANA_MAST_ANA_CONF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_conf1](index.html) module"]
pub struct ANA_CONF1_SPEC;
impl crate::RegisterSpec for ANA_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_conf1::R](R) reader structure"]
impl crate::Readable for ANA_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_conf1::W](W) writer structure"]
impl crate::Writable for ANA_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANA_CONF1 to value 0"]
impl crate::Resettable for ANA_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
