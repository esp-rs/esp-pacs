#[doc = "Register `DATA_6` reader"]
pub struct R(crate::R<DATA_6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_6` writer"]
pub struct W(crate::W<DATA_6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_6_SPEC>;
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
impl From<crate::W<DATA_6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_6` reader - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
pub type DATA_6_R = crate::FieldReader;
#[doc = "Field `DATA_6` writer - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
pub type DATA_6_W<'a, const O: u8> = crate::FieldWriter<'a, DATA_6_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
    #[inline(always)]
    pub fn data_6(&self) -> DATA_6_R {
        DATA_6_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_6")
            .field("data_6", &format_args!("{}", self.data_6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6."]
    #[inline(always)]
    #[must_use]
    pub fn data_6(&mut self) -> DATA_6_W<0> {
        DATA_6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_6](index.html) module"]
pub struct DATA_6_SPEC;
impl crate::RegisterSpec for DATA_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_6::R](R) reader structure"]
impl crate::Readable for DATA_6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_6::W](W) writer structure"]
impl crate::Writable for DATA_6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_6 to value 0"]
impl crate::Resettable for DATA_6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
