#[doc = "Register `MEM_START_ADDR` reader"]
pub struct R(crate::R<MEM_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_START_ADDR` writer"]
pub struct W(crate::W<MEM_START_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_START_ADDR_SPEC>;
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
impl From<crate::W<MEM_START_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_START_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_STAET_ADDR` reader - The start address of trace memory"]
pub type MEM_STAET_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEM_STAET_ADDR` writer - The start address of trace memory"]
pub type MEM_STAET_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, MEM_START_ADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The start address of trace memory"]
    #[inline(always)]
    pub fn mem_staet_addr(&self) -> MEM_STAET_ADDR_R {
        MEM_STAET_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_START_ADDR")
            .field(
                "mem_staet_addr",
                &format_args!("{}", self.mem_staet_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The start address of trace memory"]
    #[inline(always)]
    #[must_use]
    pub fn mem_staet_addr(&mut self) -> MEM_STAET_ADDR_W<0> {
        MEM_STAET_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mem start addr\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_start_addr](index.html) module"]
pub struct MEM_START_ADDR_SPEC;
impl crate::RegisterSpec for MEM_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_start_addr::R](R) reader structure"]
impl crate::Readable for MEM_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_start_addr::W](W) writer structure"]
impl crate::Writable for MEM_START_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_START_ADDR to value 0"]
impl crate::Resettable for MEM_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
