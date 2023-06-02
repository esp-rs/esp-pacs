#[doc = "Register `HOST_SLCHOST_FUNC2_0` reader"]
pub struct R(crate::R<HOST_SLCHOST_FUNC2_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_FUNC2_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_FUNC2_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_FUNC2_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLCHOST_FUNC2_0` writer"]
pub struct W(crate::W<HOST_SLCHOST_FUNC2_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLCHOST_FUNC2_0_SPEC>;
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
impl From<crate::W<HOST_SLCHOST_FUNC2_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLCHOST_FUNC2_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC_FUNC2_INT` reader - "]
pub type HOST_SLC_FUNC2_INT_R = crate::BitReader;
#[doc = "Field `HOST_SLC_FUNC2_INT` writer - "]
pub type HOST_SLC_FUNC2_INT_W<'a, const O: u8> = crate::BitWriter<'a, HOST_SLCHOST_FUNC2_0_SPEC, O>;
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc_func2_int(&self) -> HOST_SLC_FUNC2_INT_R {
        HOST_SLC_FUNC2_INT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_FUNC2_0")
            .field(
                "host_slc_func2_int",
                &format_args!("{}", self.host_slc_func2_int().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_FUNC2_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc_func2_int(&mut self) -> HOST_SLC_FUNC2_INT_W<24> {
        HOST_SLC_FUNC2_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_func2_0](index.html) module"]
pub struct HOST_SLCHOST_FUNC2_0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_FUNC2_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_func2_0::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_0::W](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_FUNC2_0 to value 0"]
impl crate::Resettable for HOST_SLCHOST_FUNC2_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
