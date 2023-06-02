#[doc = "Register `DATA_0` reader"]
pub struct R(crate::R<DATA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_0` writer"]
pub struct W(crate::W<DATA_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_0_SPEC>;
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
impl From<crate::W<DATA_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_0` reader - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, it stores the 0th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_0_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_0` writer - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, it stores the 0th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_0_W<'a, const O: u8> = crate::FieldWriter<'a, DATA_0_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, it stores the 0th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_0(&self) -> TX_BYTE_0_R {
        TX_BYTE_0_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_0")
            .field("tx_byte_0", &format_args!("{}", self.tx_byte_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 0 with R/W Permission. In operation mode, it stores the 0th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_0(&mut self) -> TX_BYTE_0_W<0> {
        TX_BYTE_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_0](index.html) module"]
pub struct DATA_0_SPEC;
impl crate::RegisterSpec for DATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_0::R](R) reader structure"]
impl crate::Readable for DATA_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_0::W](W) writer structure"]
impl crate::Writable for DATA_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_0 to value 0"]
impl crate::Resettable for DATA_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
