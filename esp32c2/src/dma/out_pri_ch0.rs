#[doc = "Register `OUT_PRI_CH0` reader"]
pub struct R(crate::R<OUT_PRI_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_PRI_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_PRI_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_PRI_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_PRI_CH0` writer"]
pub struct W(crate::W<OUT_PRI_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_PRI_CH0_SPEC>;
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
impl From<crate::W<OUT_PRI_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_PRI_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PRI_CH0` reader - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
pub type TX_PRI_CH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_PRI_CH0` writer - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
pub type TX_PRI_CH0_W<'a> = crate::FieldWriter<'a, u32, OUT_PRI_CH0_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri_ch0(&self) -> TX_PRI_CH0_R {
        TX_PRI_CH0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri_ch0(&mut self) -> TX_PRI_CH0_W {
        TX_PRI_CH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_OUT_PRI_CH0_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_pri_ch0](index.html) module"]
pub struct OUT_PRI_CH0_SPEC;
impl crate::RegisterSpec for OUT_PRI_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_pri_ch0::R](R) reader structure"]
impl crate::Readable for OUT_PRI_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_pri_ch0::W](W) writer structure"]
impl crate::Writable for OUT_PRI_CH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_PRI_CH0 to value 0"]
impl crate::Resettable for OUT_PRI_CH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
