#[doc = "Register `DATA_10` reader"]
pub struct R(crate::R<DATA_10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_10_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `DATA_10` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10."]
pub type DATA_10_R = crate::FieldReader;
#[doc = "Field `DATA_10` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10."]
pub type DATA_10_W<'a, const O: u8> = crate::FieldWriter<'a, DATA_10_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10."]
    #[inline(always)]
    pub fn data_10(&self) -> DATA_10_R {
        DATA_10_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_10")
            .field("data_10", &format_args!("{}", self.data_10().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 10 and when software initiate read operation, it is rx data register 10."]
    #[inline(always)]
    #[must_use]
    pub fn data_10(&mut self) -> DATA_10_W<0> {
        DATA_10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 10.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_10](index.html) module"]
pub struct DATA_10_SPEC;
impl crate::RegisterSpec for DATA_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_10::R](R) reader structure"]
impl crate::Readable for DATA_10_SPEC {
    type Reader = R;
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
