#[doc = "Register `TX_PCM2PDM_CONF1` reader"]
pub struct R(crate::R<TX_PCM2PDM_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PCM2PDM_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PCM2PDM_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PCM2PDM_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PCM2PDM_CONF1` writer"]
pub struct W(crate::W<TX_PCM2PDM_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PCM2PDM_CONF1_SPEC>;
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
impl From<crate::W<TX_PCM2PDM_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PCM2PDM_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PDM_FP` reader - I2S TX PDM Fp"]
pub type TX_PDM_FP_R = crate::FieldReader<u16>;
#[doc = "Field `TX_PDM_FP` writer - I2S TX PDM Fp"]
pub type TX_PDM_FP_W<'a, const O: u8> = crate::FieldWriter<'a, TX_PCM2PDM_CONF1_SPEC, 10, O, u16>;
#[doc = "Field `TX_PDM_FS` reader - I2S TX PDM Fs"]
pub type TX_PDM_FS_R = crate::FieldReader<u16>;
#[doc = "Field `TX_PDM_FS` writer - I2S TX PDM Fs"]
pub type TX_PDM_FS_W<'a, const O: u8> = crate::FieldWriter<'a, TX_PCM2PDM_CONF1_SPEC, 10, O, u16>;
#[doc = "Field `TX_IIR_HP_MULT12_5` reader - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
pub type TX_IIR_HP_MULT12_5_R = crate::FieldReader;
#[doc = "Field `TX_IIR_HP_MULT12_5` writer - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
pub type TX_IIR_HP_MULT12_5_W<'a, const O: u8> =
    crate::FieldWriter<'a, TX_PCM2PDM_CONF1_SPEC, 3, O>;
#[doc = "Field `TX_IIR_HP_MULT12_0` reader - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
pub type TX_IIR_HP_MULT12_0_R = crate::FieldReader;
#[doc = "Field `TX_IIR_HP_MULT12_0` writer - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
pub type TX_IIR_HP_MULT12_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, TX_PCM2PDM_CONF1_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:9 - I2S TX PDM Fp"]
    #[inline(always)]
    pub fn tx_pdm_fp(&self) -> TX_PDM_FP_R {
        TX_PDM_FP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - I2S TX PDM Fs"]
    #[inline(always)]
    pub fn tx_pdm_fs(&self) -> TX_PDM_FS_R {
        TX_PDM_FS_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:22 - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_5(&self) -> TX_IIR_HP_MULT12_5_R {
        TX_IIR_HP_MULT12_5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn tx_iir_hp_mult12_0(&self) -> TX_IIR_HP_MULT12_0_R {
        TX_IIR_HP_MULT12_0_R::new(((self.bits >> 23) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_PCM2PDM_CONF1")
            .field("tx_pdm_fp", &format_args!("{}", self.tx_pdm_fp().bits()))
            .field("tx_pdm_fs", &format_args!("{}", self.tx_pdm_fs().bits()))
            .field(
                "tx_iir_hp_mult12_5",
                &format_args!("{}", self.tx_iir_hp_mult12_5().bits()),
            )
            .field(
                "tx_iir_hp_mult12_0",
                &format_args!("{}", self.tx_iir_hp_mult12_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_PCM2PDM_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2S TX PDM Fp"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_fp(&mut self) -> TX_PDM_FP_W<0> {
        TX_PDM_FP_W::new(self)
    }
    #[doc = "Bits 10:19 - I2S TX PDM Fs"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pdm_fs(&mut self) -> TX_PDM_FS_W<10> {
        TX_PDM_FS_W::new(self)
    }
    #[doc = "Bits 20:22 - The fourth parameter of PDM TX IIR_HP filter stage 2 is (504 + I2S_TX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iir_hp_mult12_5(&mut self) -> TX_IIR_HP_MULT12_5_W<20> {
        TX_IIR_HP_MULT12_5_W::new(self)
    }
    #[doc = "Bits 23:25 - The fourth parameter of PDM TX IIR_HP filter stage 1 is (504 + I2S_TX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    #[must_use]
    pub fn tx_iir_hp_mult12_0(&mut self) -> TX_IIR_HP_MULT12_0_W<23> {
        TX_IIR_HP_MULT12_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX PCM2PDM configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pcm2pdm_conf1](index.html) module"]
pub struct TX_PCM2PDM_CONF1_SPEC;
impl crate::RegisterSpec for TX_PCM2PDM_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pcm2pdm_conf1::R](R) reader structure"]
impl crate::Readable for TX_PCM2PDM_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_pcm2pdm_conf1::W](W) writer structure"]
impl crate::Writable for TX_PCM2PDM_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_PCM2PDM_CONF1 to value 0x03f7_83c0"]
impl crate::Resettable for TX_PCM2PDM_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x03f7_83c0;
}
