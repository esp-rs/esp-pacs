#[doc = "Register `DATA_10` writer"]
pub struct W(crate::W<DATA_10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_10_SPEC>;
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
impl From<crate::W<DATA_10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_10` writer - Stored the 10th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_10_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Stored the 10th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_10(&mut self) -> TX_BYTE_10_W<0> {
        TX_BYTE_10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 10\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_10](index.html) module"]
pub struct DATA_10_SPEC;
impl crate::RegisterSpec for DATA_10_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_10::W](W) writer structure"]
impl crate::Writable for DATA_10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_10 to value 0"]
impl crate::Resettable for DATA_10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
