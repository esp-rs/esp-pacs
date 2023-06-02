#[doc = "Register `ACK_NUM` reader"]
pub struct R(crate::R<ACK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACK_NUM` writer"]
pub struct W(crate::W<ACK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACK_NUM_SPEC>;
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
impl From<crate::W<ACK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_NUM` reader - a"]
pub type ACK_NUM_R = crate::FieldReader;
#[doc = "Field `ACK_NUM` writer - a"]
pub type ACK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, ACK_NUM_SPEC, 3, O>;
#[doc = "Field `LOAD` writer - a"]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, ACK_NUM_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    pub fn ack_num(&self) -> ACK_NUM_R {
        ACK_NUM_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_NUM")
            .field("ack_num", &format_args!("{}", self.ack_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ACK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - a"]
    #[inline(always)]
    #[must_use]
    pub fn ack_num(&mut self) -> ACK_NUM_W<0> {
        ACK_NUM_W::new(self)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<3> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ack_num](index.html) module"]
pub struct ACK_NUM_SPEC;
impl crate::RegisterSpec for ACK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ack_num::R](R) reader structure"]
impl crate::Readable for ACK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ack_num::W](W) writer structure"]
impl crate::Writable for ACK_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACK_NUM to value 0x08"]
impl crate::Resettable for ACK_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
