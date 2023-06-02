#[doc = "Register `RF_PWC` reader"]
pub struct R(crate::R<RF_PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF_PWC` writer"]
pub struct W(crate::W<RF_PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_PWC_SPEC>;
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
impl From<crate::W<RF_PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_PERIF_I2C` reader - need_des"]
pub type XPD_PERIF_I2C_R = crate::BitReader;
#[doc = "Field `XPD_PERIF_I2C` writer - need_des"]
pub type XPD_PERIF_I2C_W<'a, const O: u8> = crate::BitWriter<'a, RF_PWC_SPEC, O>;
#[doc = "Field `XPD_RFTX_I2C` reader - need_des"]
pub type XPD_RFTX_I2C_R = crate::BitReader;
#[doc = "Field `XPD_RFTX_I2C` writer - need_des"]
pub type XPD_RFTX_I2C_W<'a, const O: u8> = crate::BitWriter<'a, RF_PWC_SPEC, O>;
#[doc = "Field `XPD_RFRX_I2C` reader - need_des"]
pub type XPD_RFRX_I2C_R = crate::BitReader;
#[doc = "Field `XPD_RFRX_I2C` writer - need_des"]
pub type XPD_RFRX_I2C_W<'a, const O: u8> = crate::BitWriter<'a, RF_PWC_SPEC, O>;
#[doc = "Field `XPD_RFPLL` reader - need_des"]
pub type XPD_RFPLL_R = crate::BitReader;
#[doc = "Field `XPD_RFPLL` writer - need_des"]
pub type XPD_RFPLL_W<'a, const O: u8> = crate::BitWriter<'a, RF_PWC_SPEC, O>;
#[doc = "Field `XPD_FORCE_RFPLL` reader - need_des"]
pub type XPD_FORCE_RFPLL_R = crate::BitReader;
#[doc = "Field `XPD_FORCE_RFPLL` writer - need_des"]
pub type XPD_FORCE_RFPLL_W<'a, const O: u8> = crate::BitWriter<'a, RF_PWC_SPEC, O>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn xpd_perif_i2c(&self) -> XPD_PERIF_I2C_R {
        XPD_PERIF_I2C_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn xpd_rftx_i2c(&self) -> XPD_RFTX_I2C_R {
        XPD_RFTX_I2C_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn xpd_rfrx_i2c(&self) -> XPD_RFRX_I2C_R {
        XPD_RFRX_I2C_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn xpd_rfpll(&self) -> XPD_RFPLL_R {
        XPD_RFPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn xpd_force_rfpll(&self) -> XPD_FORCE_RFPLL_R {
        XPD_FORCE_RFPLL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RF_PWC")
            .field(
                "xpd_perif_i2c",
                &format_args!("{}", self.xpd_perif_i2c().bit()),
            )
            .field(
                "xpd_rftx_i2c",
                &format_args!("{}", self.xpd_rftx_i2c().bit()),
            )
            .field(
                "xpd_rfrx_i2c",
                &format_args!("{}", self.xpd_rfrx_i2c().bit()),
            )
            .field("xpd_rfpll", &format_args!("{}", self.xpd_rfpll().bit()))
            .field(
                "xpd_force_rfpll",
                &format_args!("{}", self.xpd_force_rfpll().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RF_PWC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_perif_i2c(&mut self) -> XPD_PERIF_I2C_W<27> {
        XPD_PERIF_I2C_W::new(self)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rftx_i2c(&mut self) -> XPD_RFTX_I2C_W<28> {
        XPD_RFTX_I2C_W::new(self)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rfrx_i2c(&mut self) -> XPD_RFRX_I2C_W<29> {
        XPD_RFRX_I2C_W::new(self)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_rfpll(&mut self) -> XPD_RFPLL_W<30> {
        XPD_RFPLL_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_force_rfpll(&mut self) -> XPD_FORCE_RFPLL_W<31> {
        XPD_FORCE_RFPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_pwc](index.html) module"]
pub struct RF_PWC_SPEC;
impl crate::RegisterSpec for RF_PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_pwc::R](R) reader structure"]
impl crate::Readable for RF_PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_pwc::W](W) writer structure"]
impl crate::Writable for RF_PWC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RF_PWC to value 0x0800_0000"]
impl crate::Resettable for RF_PWC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
