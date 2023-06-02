#[doc = "Register `NOUSE` reader"]
pub struct R(crate::R<NOUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOUSE` writer"]
pub struct W(crate::W<NOUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOUSE_SPEC>;
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
impl From<crate::W<NOUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_NOUSE` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C_MST_NOUSE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_NOUSE` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C_MST_NOUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, NOUSE_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c_mst_nouse(&self) -> LP_I2C_ANA_MAST_I2C_MST_NOUSE_R {
        LP_I2C_ANA_MAST_I2C_MST_NOUSE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NOUSE")
            .field(
                "lp_i2c_ana_mast_i2c_mst_nouse",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c_mst_nouse().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<NOUSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_mst_nouse(&mut self) -> LP_I2C_ANA_MAST_I2C_MST_NOUSE_W<0> {
        LP_I2C_ANA_MAST_I2C_MST_NOUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nouse](index.html) module"]
pub struct NOUSE_SPEC;
impl crate::RegisterSpec for NOUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nouse::R](R) reader structure"]
impl crate::Readable for NOUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nouse::W](W) writer structure"]
impl crate::Writable for NOUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOUSE to value 0"]
impl crate::Resettable for NOUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
