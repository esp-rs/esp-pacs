#[doc = "Register `DATA_12` reader"]
pub struct R(crate::R<DATA_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_12` writer"]
pub struct W(crate::W<DATA_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_12_SPEC>;
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
impl From<crate::W<DATA_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_12` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
pub type DATA_12_R = crate::FieldReader;
#[doc = "Field `DATA_12` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
pub type DATA_12_W<'a, const O: u8> = crate::FieldWriter<'a, DATA_12_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
    #[inline(always)]
    pub fn data_12(&self) -> DATA_12_R {
        DATA_12_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_12")
            .field("data_12", &format_args!("{}", self.data_12().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_12_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12."]
    #[inline(always)]
    #[must_use]
    pub fn data_12(&mut self) -> DATA_12_W<0> {
        DATA_12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_12](index.html) module"]
pub struct DATA_12_SPEC;
impl crate::RegisterSpec for DATA_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_12::R](R) reader structure"]
impl crate::Readable for DATA_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_12::W](W) writer structure"]
impl crate::Writable for DATA_12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_12 to value 0"]
impl crate::Resettable for DATA_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
