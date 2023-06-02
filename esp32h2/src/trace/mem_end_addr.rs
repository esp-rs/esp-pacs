#[doc = "Register `MEM_END_ADDR` reader"]
pub struct R(crate::R<MEM_END_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_END_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_END_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_END_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_END_ADDR` writer"]
pub struct W(crate::W<MEM_END_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_END_ADDR_SPEC>;
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
impl From<crate::W<MEM_END_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_END_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_END_ADDR` reader - The end address of trace memory"]
pub type MEM_END_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEM_END_ADDR` writer - The end address of trace memory"]
pub type MEM_END_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, MEM_END_ADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of trace memory"]
    #[inline(always)]
    pub fn mem_end_addr(&self) -> MEM_END_ADDR_R {
        MEM_END_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_END_ADDR")
            .field(
                "mem_end_addr",
                &format_args!("{}", self.mem_end_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_END_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of trace memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_end_addr(&mut self) -> MEM_END_ADDR_W<0> {
        MEM_END_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mem end addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_end_addr](index.html) module"]
pub struct MEM_END_ADDR_SPEC;
impl crate::RegisterSpec for MEM_END_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_end_addr::R](R) reader structure"]
impl crate::Readable for MEM_END_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_end_addr::W](W) writer structure"]
impl crate::Writable for MEM_END_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_END_ADDR to value 0xffff_ffff"]
impl crate::Resettable for MEM_END_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
