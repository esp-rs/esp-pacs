#[doc = "Register `M_MEM%s` reader"]
pub struct R(crate::R<M_MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M_MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M_MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M_MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M_MEM%s` writer"]
pub struct W(crate::W<M_MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M_MEM_SPEC>;
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
impl From<crate::W<M_MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M_MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M_0` reader - Stores the %sth 32-bit piece of the message."]
pub type M_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `M_0` writer - Stores the %sth 32-bit piece of the message."]
pub type M_0_W<'a, const O: u8> = crate::FieldWriter<'a, M_MEM_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the message."]
    #[inline(always)]
    pub fn m_0(&self) -> M_0_R {
        M_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M_MEM")
            .field("m_0", &format_args!("{}", self.m_0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the %sth 32-bit piece of the message."]
    #[inline(always)]
    #[must_use]
    pub fn m_0(&mut self) -> M_0_W<0> {
        M_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m_mem](index.html) module"]
pub struct M_MEM_SPEC;
impl crate::RegisterSpec for M_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m_mem::R](R) reader structure"]
impl crate::Readable for M_MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m_mem::W](W) writer structure"]
impl crate::Writable for M_MEM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets M_MEM%s to value 0"]
impl crate::Resettable for M_MEM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
