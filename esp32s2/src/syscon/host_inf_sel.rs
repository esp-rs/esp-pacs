#[doc = "Register `HOST_INF_SEL` reader"]
pub struct R(crate::R<HOST_INF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_INF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_INF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_INF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_INF_SEL` writer"]
pub struct W(crate::W<HOST_INF_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_INF_SEL_SPEC>;
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
impl From<crate::W<HOST_INF_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_INF_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_IO_SWAP` reader - "]
pub type PERI_IO_SWAP_R = crate::FieldReader;
#[doc = "Field `PERI_IO_SWAP` writer - "]
pub type PERI_IO_SWAP_W<'a, const O: u8> = crate::FieldWriter<'a, HOST_INF_SEL_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn peri_io_swap(&self) -> PERI_IO_SWAP_R {
        PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_INF_SEL")
            .field(
                "peri_io_swap",
                &format_args!("{}", self.peri_io_swap().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_INF_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn peri_io_swap(&mut self) -> PERI_IO_SWAP_W<0> {
        PERI_IO_SWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_inf_sel](index.html) module"]
pub struct HOST_INF_SEL_SPEC;
impl crate::RegisterSpec for HOST_INF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_inf_sel::R](R) reader structure"]
impl crate::Readable for HOST_INF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_inf_sel::W](W) writer structure"]
impl crate::Writable for HOST_INF_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_INF_SEL to value 0"]
impl crate::Resettable for HOST_INF_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
