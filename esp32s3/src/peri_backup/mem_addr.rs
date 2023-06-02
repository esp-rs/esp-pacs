#[doc = "Register `MEM_ADDR` reader"]
pub struct R(crate::R<MEM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEM_ADDR` writer"]
pub struct W(crate::W<MEM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEM_ADDR_SPEC>;
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
impl From<crate::W<MEM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_START_ADDR` reader - x"]
pub type MEM_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MEM_START_ADDR` writer - x"]
pub type MEM_START_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, MEM_ADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn mem_start_addr(&self) -> MEM_START_ADDR_R {
        MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_ADDR")
            .field(
                "mem_start_addr",
                &format_args!("{}", self.mem_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn mem_start_addr(&mut self) -> MEM_START_ADDR_W<0> {
        MEM_START_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_addr](index.html) module"]
pub struct MEM_ADDR_SPEC;
impl crate::RegisterSpec for MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_addr::R](R) reader structure"]
impl crate::Readable for MEM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mem_addr::W](W) writer structure"]
impl crate::Writable for MEM_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_ADDR to value 0"]
impl crate::Resettable for MEM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
