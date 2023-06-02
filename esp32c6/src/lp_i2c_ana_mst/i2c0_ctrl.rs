#[doc = "Register `I2C0_CTRL` reader"]
pub struct R(crate::R<I2C0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_CTRL` writer"]
pub struct W(crate::W<I2C0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_CTRL_SPEC>;
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
impl From<crate::W<I2C0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CTRL` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CTRL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CTRL` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, I2C0_CTRL_SPEC, 25, O, u32, u32>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_BUSY` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:24 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_ctrl(&self) -> LP_I2C_ANA_MAST_I2C0_CTRL_R {
        LP_I2C_ANA_MAST_I2C0_CTRL_R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_busy(&self) -> LP_I2C_ANA_MAST_I2C0_BUSY_R {
        LP_I2C_ANA_MAST_I2C0_BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CTRL")
            .field(
                "lp_i2c_ana_mast_i2c0_ctrl",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_ctrl().bits()),
            )
            .field(
                "lp_i2c_ana_mast_i2c0_busy",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c0_busy().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C0_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_ctrl(&mut self) -> LP_I2C_ANA_MAST_I2C0_CTRL_W<0> {
        LP_I2C_ANA_MAST_I2C0_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_ctrl](index.html) module"]
pub struct I2C0_CTRL_SPEC;
impl crate::RegisterSpec for I2C0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_ctrl::R](R) reader structure"]
impl crate::Readable for I2C0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_ctrl::W](W) writer structure"]
impl crate::Writable for I2C0_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_CTRL to value 0"]
impl crate::Resettable for I2C0_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
