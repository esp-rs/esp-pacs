#[doc = "Register `BLK0_WDATA4` reader"]
pub struct R(crate::R<BLK0_WDATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_WDATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_WDATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_WDATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK0_WDATA4` writer"]
pub struct W(crate::W<BLK0_WDATA4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK0_WDATA4_SPEC>;
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
impl From<crate::W<BLK0_WDATA4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK0_WDATA4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK8M_FREQ` reader - "]
pub type CK8M_FREQ_R = crate::FieldReader;
#[doc = "Field `CK8M_FREQ` writer - "]
pub type CK8M_FREQ_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA4_SPEC, 8, O>;
#[doc = "Field `ADC_VREF` reader - True ADC reference voltage"]
pub type ADC_VREF_R = crate::FieldReader;
#[doc = "Field `ADC_VREF` writer - True ADC reference voltage"]
pub type ADC_VREF_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA4_SPEC, 5, O>;
#[doc = "Field `SDIO_DREFH` reader - "]
pub type SDIO_DREFH_R = crate::FieldReader;
#[doc = "Field `SDIO_DREFH` writer - "]
pub type SDIO_DREFH_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA4_SPEC, 2, O>;
#[doc = "Field `SDIO_DREFM` reader - "]
pub type SDIO_DREFM_R = crate::FieldReader;
#[doc = "Field `SDIO_DREFM` writer - "]
pub type SDIO_DREFM_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA4_SPEC, 2, O>;
#[doc = "Field `SDIO_DREFL` reader - "]
pub type SDIO_DREFL_R = crate::FieldReader;
#[doc = "Field `SDIO_DREFL` writer - "]
pub type SDIO_DREFL_W<'a, const O: u8> = crate::FieldWriter<'a, BLK0_WDATA4_SPEC, 2, O>;
#[doc = "Field `XPD_SDIO` reader - program for XPD_SDIO_REG"]
pub type XPD_SDIO_R = crate::BitReader;
#[doc = "Field `XPD_SDIO` writer - program for XPD_SDIO_REG"]
pub type XPD_SDIO_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA4_SPEC, O>;
#[doc = "Field `SDIO_TIEH` reader - program for SDIO_TIEH"]
pub type SDIO_TIEH_R = crate::BitReader;
#[doc = "Field `SDIO_TIEH` writer - program for SDIO_TIEH"]
pub type SDIO_TIEH_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA4_SPEC, O>;
#[doc = "Field `SDIO_FORCE` reader - program for sdio_force"]
pub type SDIO_FORCE_R = crate::BitReader;
#[doc = "Field `SDIO_FORCE` writer - program for sdio_force"]
pub type SDIO_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, BLK0_WDATA4_SPEC, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_freq(&self) -> CK8M_FREQ_R {
        CK8M_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    pub fn adc_vref(&self) -> ADC_VREF_R {
        ADC_VREF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdio_drefh(&self) -> SDIO_DREFH_R {
        SDIO_DREFH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn sdio_drefm(&self) -> SDIO_DREFM_R {
        SDIO_DREFM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sdio_drefl(&self) -> SDIO_DREFL_R {
        SDIO_DREFL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - program for XPD_SDIO_REG"]
    #[inline(always)]
    pub fn xpd_sdio(&self) -> XPD_SDIO_R {
        XPD_SDIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - program for SDIO_TIEH"]
    #[inline(always)]
    pub fn sdio_tieh(&self) -> SDIO_TIEH_R {
        SDIO_TIEH_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - program for sdio_force"]
    #[inline(always)]
    pub fn sdio_force(&self) -> SDIO_FORCE_R {
        SDIO_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA4")
            .field("ck8m_freq", &format_args!("{}", self.ck8m_freq().bits()))
            .field("adc_vref", &format_args!("{}", self.adc_vref().bits()))
            .field("sdio_drefh", &format_args!("{}", self.sdio_drefh().bits()))
            .field("sdio_drefm", &format_args!("{}", self.sdio_drefm().bits()))
            .field("sdio_drefl", &format_args!("{}", self.sdio_drefl().bits()))
            .field("xpd_sdio", &format_args!("{}", self.xpd_sdio().bit()))
            .field("sdio_tieh", &format_args!("{}", self.sdio_tieh().bit()))
            .field("sdio_force", &format_args!("{}", self.sdio_force().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_WDATA4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_freq(&mut self) -> CK8M_FREQ_W<0> {
        CK8M_FREQ_W::new(self)
    }
    #[doc = "Bits 8:12 - True ADC reference voltage"]
    #[inline(always)]
    #[must_use]
    pub fn adc_vref(&mut self) -> ADC_VREF_W<8> {
        ADC_VREF_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_drefh(&mut self) -> SDIO_DREFH_W<8> {
        SDIO_DREFH_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_drefm(&mut self) -> SDIO_DREFM_W<10> {
        SDIO_DREFM_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_drefl(&mut self) -> SDIO_DREFL_W<12> {
        SDIO_DREFL_W::new(self)
    }
    #[doc = "Bit 14 - program for XPD_SDIO_REG"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sdio(&mut self) -> XPD_SDIO_W<14> {
        XPD_SDIO_W::new(self)
    }
    #[doc = "Bit 15 - program for SDIO_TIEH"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_tieh(&mut self) -> SDIO_TIEH_W<15> {
        SDIO_TIEH_W::new(self)
    }
    #[doc = "Bit 16 - program for sdio_force"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_force(&mut self) -> SDIO_FORCE_W<16> {
        SDIO_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_wdata4](index.html) module"]
pub struct BLK0_WDATA4_SPEC;
impl crate::RegisterSpec for BLK0_WDATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_wdata4::R](R) reader structure"]
impl crate::Readable for BLK0_WDATA4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk0_wdata4::W](W) writer structure"]
impl crate::Writable for BLK0_WDATA4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA4 to value 0"]
impl crate::Resettable for BLK0_WDATA4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
