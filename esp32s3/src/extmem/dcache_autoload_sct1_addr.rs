#[doc = "Register `DCACHE_AUTOLOAD_SCT1_ADDR` reader"]
pub struct R(crate::R<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_AUTOLOAD_SCT1_ADDR` writer"]
pub struct W(crate::W<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
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
impl From<crate::W<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_AUTOLOAD_SCT1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_AUTOLOAD_SCT1_ADDR` reader - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with dcache_autoload_sct1_ena."]
pub type DCACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DCACHE_AUTOLOAD_SCT1_ADDR` writer - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with dcache_autoload_sct1_ena."]
pub type DCACHE_AUTOLOAD_SCT1_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCACHE_AUTOLOAD_SCT1_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with dcache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn dcache_autoload_sct1_addr(&self) -> DCACHE_AUTOLOAD_SCT1_ADDR_R {
        DCACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with dcache_autoload_sct1_ena."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_autoload_sct1_addr(&mut self) -> DCACHE_AUTOLOAD_SCT1_ADDR_W<0> {
        DCACHE_AUTOLOAD_SCT1_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_autoload_sct1_addr](index.html) module"]
pub struct DCACHE_AUTOLOAD_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for DCACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_autoload_sct1_addr::R](R) reader structure"]
impl crate::Readable for DCACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_autoload_sct1_addr::W](W) writer structure"]
impl crate::Writable for DCACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for DCACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
