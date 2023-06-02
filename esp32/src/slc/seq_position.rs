#[doc = "Register `SEQ_POSITION` reader"]
pub struct R(crate::R<SEQ_POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_POSITION` writer"]
pub struct W(crate::W<SEQ_POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_POSITION_SPEC>;
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
impl From<crate::W<SEQ_POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_SEQ_POSITION` reader - "]
pub type SLC0_SEQ_POSITION_R = crate::FieldReader;
#[doc = "Field `SLC0_SEQ_POSITION` writer - "]
pub type SLC0_SEQ_POSITION_W<'a, const O: u8> = crate::FieldWriter<'a, SEQ_POSITION_SPEC, 8, O>;
#[doc = "Field `SLC1_SEQ_POSITION` reader - "]
pub type SLC1_SEQ_POSITION_R = crate::FieldReader;
#[doc = "Field `SLC1_SEQ_POSITION` writer - "]
pub type SLC1_SEQ_POSITION_W<'a, const O: u8> = crate::FieldWriter<'a, SEQ_POSITION_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc0_seq_position(&self) -> SLC0_SEQ_POSITION_R {
        SLC0_SEQ_POSITION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc1_seq_position(&self) -> SLC1_SEQ_POSITION_R {
        SLC1_SEQ_POSITION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEQ_POSITION")
            .field(
                "slc0_seq_position",
                &format_args!("{}", self.slc0_seq_position().bits()),
            )
            .field(
                "slc1_seq_position",
                &format_args!("{}", self.slc1_seq_position().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SEQ_POSITION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_seq_position(&mut self) -> SLC0_SEQ_POSITION_W<0> {
        SLC0_SEQ_POSITION_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_seq_position(&mut self) -> SLC1_SEQ_POSITION_W<8> {
        SLC1_SEQ_POSITION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_position](index.html) module"]
pub struct SEQ_POSITION_SPEC;
impl crate::RegisterSpec for SEQ_POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_position::R](R) reader structure"]
impl crate::Readable for SEQ_POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_position::W](W) writer structure"]
impl crate::Writable for SEQ_POSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ_POSITION to value 0x0509"]
impl crate::Resettable for SEQ_POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0509;
}
