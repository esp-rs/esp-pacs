#[doc = "Register `DATA_3` reader"]
pub struct R(crate::R<DATA_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_3` writer"]
pub struct W(crate::W<DATA_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_3_SPEC>;
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
impl From<crate::W<DATA_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_3` reader - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3."]
pub type TX_BYTE_3_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_3` writer - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3."]
pub type TX_BYTE_3_W<'a, const O: u8> = crate::FieldWriter<'a, DATA_3_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3."]
    #[inline(always)]
    pub fn tx_byte_3(&self) -> TX_BYTE_3_R {
        TX_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_3")
            .field("tx_byte_3", &format_args!("{}", self.tx_byte_3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 3 and when software initiate read operation, it is rx data register 3."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_3(&mut self) -> TX_BYTE_3_W<0> {
        TX_BYTE_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_3](index.html) module"]
pub struct DATA_3_SPEC;
impl crate::RegisterSpec for DATA_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_3::R](R) reader structure"]
impl crate::Readable for DATA_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_3::W](W) writer structure"]
impl crate::Writable for DATA_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_3 to value 0"]
impl crate::Resettable for DATA_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
