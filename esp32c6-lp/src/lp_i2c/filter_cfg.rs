#[doc = "Register `FILTER_CFG` reader"]
pub struct R(crate::R<FILTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_CFG` writer"]
pub struct W(crate::W<FILTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_CFG_SPEC>;
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
impl From<crate::W<FILTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_FILTER_THRES` reader - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type SCL_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `SCL_FILTER_THRES` writer - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type SCL_FILTER_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, FILTER_CFG_SPEC, 4, O>;
#[doc = "Field `SDA_FILTER_THRES` reader - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type SDA_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `SDA_FILTER_THRES` writer - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type SDA_FILTER_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, FILTER_CFG_SPEC, 4, O>;
#[doc = "Field `SCL_FILTER_EN` reader - This is the filter enable bit for SCL."]
pub type SCL_FILTER_EN_R = crate::BitReader;
#[doc = "Field `SCL_FILTER_EN` writer - This is the filter enable bit for SCL."]
pub type SCL_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, FILTER_CFG_SPEC, O>;
#[doc = "Field `SDA_FILTER_EN` reader - This is the filter enable bit for SDA."]
pub type SDA_FILTER_EN_R = crate::BitReader;
#[doc = "Field `SDA_FILTER_EN` writer - This is the filter enable bit for SDA."]
pub type SDA_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, FILTER_CFG_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    pub fn scl_filter_thres(&self) -> SCL_FILTER_THRES_R {
        SCL_FILTER_THRES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    pub fn sda_filter_thres(&self) -> SDA_FILTER_THRES_R {
        SDA_FILTER_THRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - This is the filter enable bit for SCL."]
    #[inline(always)]
    pub fn scl_filter_en(&self) -> SCL_FILTER_EN_R {
        SCL_FILTER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn sda_filter_en(&self) -> SDA_FILTER_EN_R {
        SDA_FILTER_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CFG")
            .field(
                "scl_filter_thres",
                &format_args!("{}", self.scl_filter_thres().bits()),
            )
            .field(
                "sda_filter_thres",
                &format_args!("{}", self.sda_filter_thres().bits()),
            )
            .field(
                "scl_filter_en",
                &format_args!("{}", self.scl_filter_en().bit()),
            )
            .field(
                "sda_filter_en",
                &format_args!("{}", self.sda_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    #[must_use]
    pub fn scl_filter_thres(&mut self) -> SCL_FILTER_THRES_W<0> {
        SCL_FILTER_THRES_W::new(self)
    }
    #[doc = "Bits 4:7 - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    #[must_use]
    pub fn sda_filter_thres(&mut self) -> SDA_FILTER_THRES_W<4> {
        SDA_FILTER_THRES_W::new(self)
    }
    #[doc = "Bit 8 - This is the filter enable bit for SCL."]
    #[inline(always)]
    #[must_use]
    pub fn scl_filter_en(&mut self) -> SCL_FILTER_EN_W<8> {
        SCL_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 9 - This is the filter enable bit for SDA."]
    #[inline(always)]
    #[must_use]
    pub fn sda_filter_en(&mut self) -> SDA_FILTER_EN_W<9> {
        SDA_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCL and SDA filter configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_cfg](index.html) module"]
pub struct FILTER_CFG_SPEC;
impl crate::RegisterSpec for FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_cfg::R](R) reader structure"]
impl crate::Readable for FILTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_cfg::W](W) writer structure"]
impl crate::Writable for FILTER_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER_CFG to value 0x0300"]
impl crate::Resettable for FILTER_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
