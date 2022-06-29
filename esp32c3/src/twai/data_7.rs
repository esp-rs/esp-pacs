#[doc = "Register `DATA_7` writer"]
pub struct W(crate::W<DATA_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_7_SPEC>;
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
impl From<crate::W<DATA_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_7` writer - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, it stores the 7th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_7_W<'a> = crate::FieldWriter<'a, u32, DATA_7_SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, it stores the 7th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_7(&mut self) -> TX_BYTE_7_W {
        TX_BYTE_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 7\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_7](index.html) module"]
pub struct DATA_7_SPEC;
impl crate::RegisterSpec for DATA_7_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_7::W](W) writer structure"]
impl crate::Writable for DATA_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_7 to value 0"]
impl crate::Resettable for DATA_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
