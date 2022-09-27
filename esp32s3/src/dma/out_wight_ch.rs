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
#[doc = "Field `TX_WEIGHT_CH` reader - The weight of Tx channel 0."]
pub type TX_WEIGHT_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_WEIGHT_CH` writer - The weight of Tx channel 0."]
pub type TX_WEIGHT_CH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUT_WIGHT_CH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight_ch(&self) -> TX_WEIGHT_CH_R {
        TX_WEIGHT_CH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Tx channel 0."]
    #[inline(always)]
    pub fn tx_weight_ch(&mut self) -> TX_WEIGHT_CH_W<8> {
        TX_WEIGHT_CH_W::new(self)
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
}
#[doc = "`reset()` method sets OUT_WIGHT_CH%s to value 0x0f00"]
impl crate::Resettable for OUT_WIGHT_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00
    }
}
