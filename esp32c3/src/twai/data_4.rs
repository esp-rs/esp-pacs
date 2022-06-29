#[doc = "Register `DATA_4` writer"]
pub struct W(crate::W<DATA_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_4_SPEC>;
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
impl From<crate::W<DATA_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_4` writer - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, it stores the 4th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_4_W<'a> = crate::FieldWriter<'a, u32, DATA_4_SPEC, u8, u8, 8, 0>;
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, it stores the 4th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_4(&mut self) -> TX_BYTE_4_W {
        TX_BYTE_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 4\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_4](index.html) module"]
pub struct DATA_4_SPEC;
impl crate::RegisterSpec for DATA_4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_4::W](W) writer structure"]
impl crate::Writable for DATA_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_4 to value 0"]
impl crate::Resettable for DATA_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
