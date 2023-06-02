#[doc = "Register `EXT_MEM_WRITEBACK_BYPASS` reader"]
pub struct R(crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_MEM_WRITEBACK_BYPASS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_MEM_WRITEBACK_BYPASS` writer"]
pub struct W(crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>;
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
impl From<crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_MEM_WRITEBACK_BYPASS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITEBACK_BYPASS` reader - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
pub type WRITEBACK_BYPASS_R = crate::BitReader;
#[doc = "Field `WRITEBACK_BYPASS` writer - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
pub type WRITEBACK_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, EXT_MEM_WRITEBACK_BYPASS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
    #[inline(always)]
    pub fn writeback_bypass(&self) -> WRITEBACK_BYPASS_R {
        WRITEBACK_BYPASS_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_MEM_WRITEBACK_BYPASS")
            .field(
                "writeback_bypass",
                &format_args!("{}", self.writeback_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT_MEM_WRITEBACK_BYPASS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to bypass cache writeback request to external memory so that spi will not check its attribute."]
    #[inline(always)]
    #[must_use]
    pub fn writeback_bypass(&mut self) -> WRITEBACK_BYPASS_W<0> {
        WRITEBACK_BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_mem_writeback_bypass](index.html) module"]
pub struct EXT_MEM_WRITEBACK_BYPASS_SPEC;
impl crate::RegisterSpec for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_mem_writeback_bypass::R](R) reader structure"]
impl crate::Readable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_mem_writeback_bypass::W](W) writer structure"]
impl crate::Writable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_MEM_WRITEBACK_BYPASS to value 0"]
impl crate::Resettable for EXT_MEM_WRITEBACK_BYPASS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
