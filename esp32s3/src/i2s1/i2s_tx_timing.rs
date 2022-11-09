#[doc = "Register `I2S_TX_TIMING` reader"]
pub struct R(crate::R<I2S_TX_TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_TX_TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_TX_TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_TX_TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_TX_TIMING` writer"]
pub struct W(crate::W<I2S_TX_TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_TX_TIMING_SPEC>;
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
impl From<crate::W<I2S_TX_TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_TX_TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S_TX_SD_OUT_DM` reader - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_SD_OUT_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_SD_OUT_DM` writer - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_SD_OUT_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S_TX_SD1_OUT_DM` reader - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_SD1_OUT_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_SD1_OUT_DM` writer - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_SD1_OUT_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S_TX_WS_OUT_DM` reader - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_WS_OUT_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_WS_OUT_DM` writer - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_WS_OUT_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S_TX_BCK_OUT_DM` reader - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_BCK_OUT_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_BCK_OUT_DM` writer - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_BCK_OUT_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S_TX_WS_IN_DM` reader - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_WS_IN_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_WS_IN_DM` writer - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_WS_IN_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_TIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2S_TX_BCK_IN_DM` reader - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_BCK_IN_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2S_TX_BCK_IN_DM` writer - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
pub type I2S_TX_BCK_IN_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_TX_TIMING_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn i2s_tx_sd_out_dm(&self) -> I2S_TX_SD_OUT_DM_R {
        I2S_TX_SD_OUT_DM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn i2s_tx_sd1_out_dm(&self) -> I2S_TX_SD1_OUT_DM_R {
        I2S_TX_SD1_OUT_DM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn i2s_tx_ws_out_dm(&self) -> I2S_TX_WS_OUT_DM_R {
        I2S_TX_WS_OUT_DM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn i2s_tx_bck_out_dm(&self) -> I2S_TX_BCK_OUT_DM_R {
        I2S_TX_BCK_OUT_DM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn i2s_tx_ws_in_dm(&self) -> I2S_TX_WS_IN_DM_R {
        I2S_TX_WS_IN_DM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    pub fn i2s_tx_bck_in_dm(&self) -> I2S_TX_BCK_IN_DM_R {
        I2S_TX_BCK_IN_DM_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The delay mode of I2S TX SD output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_sd_out_dm(&mut self) -> I2S_TX_SD_OUT_DM_W<0> {
        I2S_TX_SD_OUT_DM_W::new(self)
    }
    #[doc = "Bits 4:5 - The delay mode of I2S TX SD1 output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_sd1_out_dm(&mut self) -> I2S_TX_SD1_OUT_DM_W<4> {
        I2S_TX_SD1_OUT_DM_W::new(self)
    }
    #[doc = "Bits 16:17 - The delay mode of I2S TX WS output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_ws_out_dm(&mut self) -> I2S_TX_WS_OUT_DM_W<16> {
        I2S_TX_WS_OUT_DM_W::new(self)
    }
    #[doc = "Bits 20:21 - The delay mode of I2S TX BCK output signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_bck_out_dm(&mut self) -> I2S_TX_BCK_OUT_DM_W<20> {
        I2S_TX_BCK_OUT_DM_W::new(self)
    }
    #[doc = "Bits 24:25 - The delay mode of I2S TX WS input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_ws_in_dm(&mut self) -> I2S_TX_WS_IN_DM_W<24> {
        I2S_TX_WS_IN_DM_W::new(self)
    }
    #[doc = "Bits 28:29 - The delay mode of I2S TX BCK input signal. 0: bypass. 1: delay by pos edge. 2: delay by neg edge. 3: not used."]
    #[inline(always)]
    #[must_use]
    pub fn i2s_tx_bck_in_dm(&mut self) -> I2S_TX_BCK_IN_DM_W<28> {
        I2S_TX_BCK_IN_DM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S TX timing control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_tx_timing](index.html) module"]
pub struct I2S_TX_TIMING_SPEC;
impl crate::RegisterSpec for I2S_TX_TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_tx_timing::R](R) reader structure"]
impl crate::Readable for I2S_TX_TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_tx_timing::W](W) writer structure"]
impl crate::Writable for I2S_TX_TIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_TX_TIMING to value 0"]
impl crate::Resettable for I2S_TX_TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
