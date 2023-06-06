#[doc = "Register `PDM_FREQ_CONF` reader"]
pub struct R(crate::R<PDM_FREQ_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_FREQ_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_FREQ_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_FREQ_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_FREQ_CONF` writer"]
pub struct W(crate::W<PDM_FREQ_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_FREQ_CONF_SPEC>;
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
impl From<crate::W<PDM_FREQ_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_FREQ_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PDM_FS` reader - "]
pub type TX_PDM_FS_R = crate::FieldReader<u16>;
#[doc = "Field `TX_PDM_FS` writer - "]
pub type TX_PDM_FS_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_FREQ_CONF_SPEC, 10, O, u16>;
#[doc = "Field `TX_PDM_FP` reader - "]
pub type TX_PDM_FP_R = crate::FieldReader<u16>;
#[doc = "Field `TX_PDM_FP` writer - "]
pub type TX_PDM_FP_W<'a, const O: u8> = crate::FieldWriter<'a, PDM_FREQ_CONF_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> TX_PDM_FS_R {
        TX_PDM_FS_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> TX_PDM_FP_R {
        TX_PDM_FP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDM_FREQ_CONF")
            .field("tx_pdm_fs", &format_args!("{}", self.tx_pdm_fs().bits()))
            .field("tx_pdm_fp", &format_args!("{}", self.tx_pdm_fp().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PDM_FREQ_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_fs(&mut self) -> TX_PDM_FS_W<0> {
        TX_PDM_FS_W::new(self)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_fp(&mut self) -> TX_PDM_FP_W<10> {
        TX_PDM_FP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_freq_conf](index.html) module"]
pub struct PDM_FREQ_CONF_SPEC;
impl crate::RegisterSpec for PDM_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_freq_conf::R](R) reader structure"]
impl crate::Readable for PDM_FREQ_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_freq_conf::W](W) writer structure"]
impl crate::Writable for PDM_FREQ_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDM_FREQ_CONF to value 0x000f_01e0"]
impl crate::Resettable for PDM_FREQ_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_01e0;
}
