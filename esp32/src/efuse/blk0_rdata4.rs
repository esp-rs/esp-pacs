#[doc = "Register `BLK0_RDATA4` reader"]
pub struct R(crate::R<BLK0_RDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_RDATA4` writer"]
pub struct W(crate::W<BLK0_RDATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_RDATA4_SPEC>;
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
impl From<crate::W<BLK0_RDATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_RDATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_CK8M_FREQ` reader - "]
pub type RD_CK8M_FREQ_R = crate::FieldReader;
#[doc = "Field `RD_ADC_VREF` reader - True ADC reference voltage"]
pub type RD_ADC_VREF_R = crate::FieldReader;
#[doc = "Field `RD_ADC_VREF` writer - True ADC reference voltage"]
pub type RD_ADC_VREF_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_RDATA4_SPEC, 5, O>;
#[doc = "Field `RD_SDIO_DREFH` reader - "]
pub type RD_SDIO_DREFH_R = crate::FieldReader;
#[doc = "Field `RD_SDIO_DREFM` reader - "]
pub type RD_SDIO_DREFM_R = crate::FieldReader;
#[doc = "Field `RD_SDIO_DREFL` reader - "]
pub type RD_SDIO_DREFL_R = crate::FieldReader;
#[doc = "Field `RD_XPD_SDIO` reader - read for XPD_SDIO_REG"]
pub type RD_XPD_SDIO_R = crate::BitReader;
#[doc = "Field `RD_SDIO_TIEH` reader - read for SDIO_TIEH"]
pub type RD_SDIO_TIEH_R = crate::BitReader;
#[doc = "Field `RD_SDIO_FORCE` reader - read for sdio_force"]
pub type RD_SDIO_FORCE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_ck8m_freq(&self) -> RD_CK8M_FREQ_R {
        RD_CK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn rd_adc_vref(&self) -> RD_ADC_VREF_R {
        RD_ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rd_sdio_drefh(&self) -> RD_SDIO_DREFH_R {
        RD_SDIO_DREFH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn rd_sdio_drefm(&self) -> RD_SDIO_DREFM_R {
        RD_SDIO_DREFM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn rd_sdio_drefl(&self) -> RD_SDIO_DREFL_R {
        RD_SDIO_DREFL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - read for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn rd_xpd_sdio(&self) -> RD_XPD_SDIO_R {
        RD_XPD_SDIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - read for SDIO_TIEH"]
    #[inline(always)]
    pub fn rd_sdio_tieh(&self) -> RD_SDIO_TIEH_R {
        RD_SDIO_TIEH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - read for sdio_force"]
    #[inline(always)]
    pub fn rd_sdio_force(&self) -> RD_SDIO_FORCE_R {
        RD_SDIO_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA4")
            .field(
                "rd_ck8m_freq",
                &format_args!("{}", self.rd_ck8m_freq().bits()),
            )
            .field(
                "rd_adc_vref",
                &format_args!("{}", self.rd_adc_vref().bits()),
            )
            .field(
                "rd_sdio_drefh",
                &format_args!("{}", self.rd_sdio_drefh().bits()),
            )
            .field(
                "rd_sdio_drefm",
                &format_args!("{}", self.rd_sdio_drefm().bits()),
            )
            .field(
                "rd_sdio_drefl",
                &format_args!("{}", self.rd_sdio_drefl().bits()),
            )
            .field("rd_xpd_sdio", &format_args!("{}", self.rd_xpd_sdio().bit()))
            .field(
                "rd_sdio_tieh",
                &format_args!("{}", self.rd_sdio_tieh().bit()),
            )
            .field(
                "rd_sdio_force",
                &format_args!("{}", self.rd_sdio_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc_vref(&mut self) -> RD_ADC_VREF_W<8> {
        RD_ADC_VREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata4](index.html) module"]
pub struct BLK0_RDATA4_SPEC;
impl crate::RegisterSpec for BLK0_RDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata4::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_rdata4::W](W) writer structure"]
impl crate::Writable for BLK0_RDATA4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_RDATA4 to value 0"]
impl crate::Resettable for BLK0_RDATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
