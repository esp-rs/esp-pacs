#[doc = "Register `PRO_TRACEMEM_ENA` reader"]
pub struct R(crate::R<PRO_TRACEMEM_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_TRACEMEM_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_TRACEMEM_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_TRACEMEM_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_TRACEMEM_ENA` writer"]
pub struct W(crate::W<PRO_TRACEMEM_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_TRACEMEM_ENA_SPEC>;
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
impl From<crate::W<PRO_TRACEMEM_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_TRACEMEM_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_TRACEMEM_ENA` reader - "]
pub type PRO_TRACEMEM_ENA_R = crate::BitReader;
#[doc = "Field `PRO_TRACEMEM_ENA` writer - "]
pub type PRO_TRACEMEM_ENA_W<'a, const O: u8> = crate::BitWriter<'a, PRO_TRACEMEM_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_tracemem_ena(&self) -> PRO_TRACEMEM_ENA_R {
        PRO_TRACEMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_TRACEMEM_ENA")
            .field(
                "pro_tracemem_ena",
                &format_args!("{}", self.pro_tracemem_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_TRACEMEM_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro_tracemem_ena(&mut self) -> PRO_TRACEMEM_ENA_W<0> {
        PRO_TRACEMEM_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_tracemem_ena](index.html) module"]
pub struct PRO_TRACEMEM_ENA_SPEC;
impl crate::RegisterSpec for PRO_TRACEMEM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_tracemem_ena::R](R) reader structure"]
impl crate::Readable for PRO_TRACEMEM_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_tracemem_ena::W](W) writer structure"]
impl crate::Writable for PRO_TRACEMEM_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_TRACEMEM_ENA to value 0"]
impl crate::Resettable for PRO_TRACEMEM_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
