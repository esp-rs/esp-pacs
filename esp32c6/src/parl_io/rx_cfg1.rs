#[doc = "Register `RX_CFG1` reader"]
pub struct R(crate::R<RX_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CFG1` writer"]
pub struct W(crate::W<RX_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CFG1_SPEC>;
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
impl From<crate::W<RX_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_REG_UPDATE` writer - Write 1 to update rx register configuration signals."]
pub type RX_REG_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG1_SPEC, O>;
#[doc = "Field `RX_TIMEOUT_EN` reader - Write 1 to enable timeout count to generate error eof."]
pub type RX_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `RX_TIMEOUT_EN` writer - Write 1 to enable timeout count to generate error eof."]
pub type RX_TIMEOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG1_SPEC, O>;
#[doc = "Field `RX_EXT_EN_SEL` reader - Configures rx external enable signal selection from 16 data lines."]
pub type RX_EXT_EN_SEL_R = crate::FieldReader;
#[doc = "Field `RX_EXT_EN_SEL` writer - Configures rx external enable signal selection from 16 data lines."]
pub type RX_EXT_EN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CFG1_SPEC, 4, O>;
#[doc = "Field `RX_TIMEOUT_THRESHOLD` reader - Configures rx threshold of timeout counter."]
pub type RX_TIMEOUT_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_TIMEOUT_THRESHOLD` writer - Configures rx threshold of timeout counter."]
pub type RX_TIMEOUT_THRESHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CFG1_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 3 - Write 1 to enable timeout count to generate error eof."]
    #[inline(always)]
    pub fn rx_timeout_en(&self) -> RX_TIMEOUT_EN_R {
        RX_TIMEOUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Configures rx external enable signal selection from 16 data lines."]
    #[inline(always)]
    pub fn rx_ext_en_sel(&self) -> RX_EXT_EN_SEL_R {
        RX_EXT_EN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Configures rx threshold of timeout counter."]
    #[inline(always)]
    pub fn rx_timeout_threshold(&self) -> RX_TIMEOUT_THRESHOLD_R {
        RX_TIMEOUT_THRESHOLD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CFG1")
            .field(
                "rx_timeout_en",
                &format_args!("{}", self.rx_timeout_en().bit()),
            )
            .field(
                "rx_ext_en_sel",
                &format_args!("{}", self.rx_ext_en_sel().bits()),
            )
            .field(
                "rx_timeout_threshold",
                &format_args!("{}", self.rx_timeout_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - Write 1 to update rx register configuration signals."]
    #[inline(always)]
    #[must_use]
    pub fn rx_reg_update(&mut self) -> RX_REG_UPDATE_W<2> {
        RX_REG_UPDATE_W::new(self)
    }
    #[doc = "Bit 3 - Write 1 to enable timeout count to generate error eof."]
    #[inline(always)]
    #[must_use]
    pub fn rx_timeout_en(&mut self) -> RX_TIMEOUT_EN_W<3> {
        RX_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bits 12:15 - Configures rx external enable signal selection from 16 data lines."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ext_en_sel(&mut self) -> RX_EXT_EN_SEL_W<12> {
        RX_EXT_EN_SEL_W::new(self)
    }
    #[doc = "Bits 16:31 - Configures rx threshold of timeout counter."]
    #[inline(always)]
    #[must_use]
    pub fn rx_timeout_threshold(&mut self) -> RX_TIMEOUT_THRESHOLD_W<16> {
        RX_TIMEOUT_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel RX module configuration register1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_cfg1](index.html) module"]
pub struct RX_CFG1_SPEC;
impl crate::RegisterSpec for RX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_cfg1::R](R) reader structure"]
impl crate::Readable for RX_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_cfg1::W](W) writer structure"]
impl crate::Writable for RX_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CFG1 to value 0x0fff_f008"]
impl crate::Resettable for RX_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_f008;
}
