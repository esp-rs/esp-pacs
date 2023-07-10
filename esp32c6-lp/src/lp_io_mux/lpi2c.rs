#[doc = "Register `LPI2C` reader"]
pub struct R(crate::R<LPI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPI2C` writer"]
pub struct W(crate::W<LPI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPI2C_SPEC>;
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
impl From<crate::W<LPI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_SDA_IE` reader - need des"]
pub type LP_I2C_SDA_IE_R = crate::BitReader;
#[doc = "Field `LP_I2C_SDA_IE` writer - need des"]
pub type LP_I2C_SDA_IE_W<'a, const O: u8> = crate::BitWriter<'a, LPI2C_SPEC, O>;
#[doc = "Field `LP_I2C_SCL_IE` reader - need des"]
pub type LP_I2C_SCL_IE_R = crate::BitReader;
#[doc = "Field `LP_I2C_SCL_IE` writer - need des"]
pub type LP_I2C_SCL_IE_W<'a, const O: u8> = crate::BitWriter<'a, LPI2C_SPEC, O>;
impl R {
    #[doc = "Bit 30 - need des"]
    #[inline(always)]
    pub fn lp_i2c_sda_ie(&self) -> LP_I2C_SDA_IE_R {
        LP_I2C_SDA_IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need des"]
    #[inline(always)]
    pub fn lp_i2c_scl_ie(&self) -> LP_I2C_SCL_IE_R {
        LP_I2C_SCL_IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI2C")
            .field(
                "lp_i2c_sda_ie",
                &format_args!("{}", self.lp_i2c_sda_ie().bit()),
            )
            .field(
                "lp_i2c_scl_ie",
                &format_args!("{}", self.lp_i2c_scl_ie().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPI2C_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 30 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_sda_ie(&mut self) -> LP_I2C_SDA_IE_W<30> {
        LP_I2C_SDA_IE_W::new(self)
    }
    #[doc = "Bit 31 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_scl_ie(&mut self) -> LP_I2C_SCL_IE_W<31> {
        LP_I2C_SCL_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpi2c](index.html) module"]
pub struct LPI2C_SPEC;
impl crate::RegisterSpec for LPI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpi2c::R](R) reader structure"]
impl crate::Readable for LPI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpi2c::W](W) writer structure"]
impl crate::Writable for LPI2C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPI2C to value 0xc000_0000"]
impl crate::Resettable for LPI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
