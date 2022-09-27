#[doc = "Register `LINESIZE` reader"]
pub struct R(crate::R<LINESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINESIZE` writer"]
pub struct W(crate::W<LINESIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINESIZE_SPEC>;
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
impl From<crate::W<LINESIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINESIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINESIZE` reader - Configures the data size of one encryption."]
pub type LINESIZE_R = crate::BitReader<bool>;
#[doc = "Field `LINESIZE` writer - Configures the data size of one encryption."]
pub type LINESIZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LINESIZE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Configures the data size of one encryption."]
    #[inline(always)]
    pub fn linesize(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the data size of one encryption."]
    #[inline(always)]
    pub fn linesize(&mut self) -> LINESIZE_W<0> {
        LINESIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTS-AES line-size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linesize](index.html) module"]
pub struct LINESIZE_SPEC;
impl crate::RegisterSpec for LINESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [linesize::R](R) reader structure"]
impl crate::Readable for LINESIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linesize::W](W) writer structure"]
impl crate::Writable for LINESIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LINESIZE to value 0"]
impl crate::Resettable for LINESIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
