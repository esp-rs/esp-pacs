#[doc = "Register `CH%sDATA` reader"]
pub struct R(crate::R<CHDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sDATA` writer"]
pub struct W(crate::W<CHDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDATA_SPEC>;
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
impl From<crate::W<CHDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_DATA` reader - The read and write data register for CHANNEL%s by apb fifo access."]
pub type CH_DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CH_DATA` writer - The read and write data register for CHANNEL%s by apb fifo access."]
pub type CH_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, CHDATA_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    pub fn ch_data(&self) -> CH_DATA_R {
        CH_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHDATA")
            .field("ch_data", &format_args!("{}", self.ch_data().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The read and write data register for CHANNEL%s by apb fifo access."]
    #[inline(always)]
    #[must_use]
    pub fn ch_data(&mut self) -> CH_DATA_W<0> {
        CH_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The read and write data register for CHANNEL%s by apb fifo access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdata](index.html) module"]
pub struct CHDATA_SPEC;
impl crate::RegisterSpec for CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdata::R](R) reader structure"]
impl crate::Readable for CHDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chdata::W](W) writer structure"]
impl crate::Writable for CHDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sDATA to value 0"]
impl crate::Resettable for CHDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
