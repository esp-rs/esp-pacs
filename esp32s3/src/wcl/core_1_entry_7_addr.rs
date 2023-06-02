#[doc = "Register `Core_1_ENTRY_7_ADDR` reader"]
pub struct R(crate::R<CORE_1_ENTRY_7_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_ENTRY_7_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_ENTRY_7_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_ENTRY_7_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_ENTRY_7_ADDR` writer"]
pub struct W(crate::W<CORE_1_ENTRY_7_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_ENTRY_7_ADDR_SPEC>;
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
impl From<crate::W<CORE_1_ENTRY_7_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_ENTRY_7_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_ENTRY_7_ADDR` reader - Core_1 Entry 7 address from WORLD1 to WORLD0"]
pub type CORE_1_ENTRY_7_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_1_ENTRY_7_ADDR` writer - Core_1 Entry 7 address from WORLD1 to WORLD0"]
pub type CORE_1_ENTRY_7_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_ENTRY_7_ADDR_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core_1 Entry 7 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn core_1_entry_7_addr(&self) -> CORE_1_ENTRY_7_ADDR_R {
        CORE_1_ENTRY_7_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_ENTRY_7_ADDR")
            .field(
                "core_1_entry_7_addr",
                &format_args!("{}", self.core_1_entry_7_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_ENTRY_7_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core_1 Entry 7 address from WORLD1 to WORLD0"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_entry_7_addr(&mut self) -> CORE_1_ENTRY_7_ADDR_W<0> {
        CORE_1_ENTRY_7_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_1 Entry 7 address configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_entry_7_addr](index.html) module"]
pub struct CORE_1_ENTRY_7_ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_ENTRY_7_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_entry_7_addr::R](R) reader structure"]
impl crate::Readable for CORE_1_ENTRY_7_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_entry_7_addr::W](W) writer structure"]
impl crate::Writable for CORE_1_ENTRY_7_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_ENTRY_7_ADDR to value 0"]
impl crate::Resettable for CORE_1_ENTRY_7_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
