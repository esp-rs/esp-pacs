#[doc = "Register `OUT_WIGHT_CH%s` reader"]
pub struct R(crate::R<OUT_WIGHT_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_WIGHT_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_WIGHT_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_WIGHT_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_WIGHT_CH%s` writer"]
pub struct W(crate::W<OUT_WIGHT_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_WIGHT_CH_SPEC>;
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
impl From<crate::W<OUT_WIGHT_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_WIGHT_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WEIGHT` reader - The weight of Tx channel 0."]
pub type TX_WEIGHT_R = crate::FieldReader;
#[doc = "Field `TX_WEIGHT` writer - The weight of Tx channel 0."]
pub type TX_WEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_WIGHT_CH_SPEC, 4, O>;
impl R {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight(&self) -> TX_WEIGHT_R {
        TX_WEIGHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_WIGHT_CH")
            .field("tx_weight", &format_args!("{}", self.tx_weight().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_WIGHT_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_weight(&mut self) -> TX_WEIGHT_W<8> {
        TX_WEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Weight register of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_wight_ch](index.html) module"]
pub struct OUT_WIGHT_CH_SPEC;
impl crate::RegisterSpec for OUT_WIGHT_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_wight_ch::R](R) reader structure"]
impl crate::Readable for OUT_WIGHT_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_wight_ch::W](W) writer structure"]
impl crate::Writable for OUT_WIGHT_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_WIGHT_CH%s to value 0x0f00"]
impl crate::Resettable for OUT_WIGHT_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00;
}
