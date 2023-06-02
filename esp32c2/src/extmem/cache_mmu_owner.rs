#[doc = "Register `CACHE_MMU_OWNER` reader"]
pub struct R(crate::R<CACHE_MMU_OWNER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_MMU_OWNER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_MMU_OWNER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_MMU_OWNER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_MMU_OWNER` writer"]
pub struct W(crate::W<CACHE_MMU_OWNER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_MMU_OWNER_SPEC>;
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
impl From<crate::W<CACHE_MMU_OWNER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_MMU_OWNER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_MMU_OWNER` reader - The bits are used to specify the owner of MMU.bit0/bit2: ibus, bit1/bit3: dbus"]
pub type CACHE_MMU_OWNER_R = crate::FieldReader;
#[doc = "Field `CACHE_MMU_OWNER` writer - The bits are used to specify the owner of MMU.bit0/bit2: ibus, bit1/bit3: dbus"]
pub type CACHE_MMU_OWNER_W<'a, const O: u8> = crate::FieldWriter<'a, CACHE_MMU_OWNER_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - The bits are used to specify the owner of MMU.bit0/bit2: ibus, bit1/bit3: dbus"]
    #[inline(always)]
    pub fn cache_mmu_owner(&self) -> CACHE_MMU_OWNER_R {
        CACHE_MMU_OWNER_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_MMU_OWNER")
            .field(
                "cache_mmu_owner",
                &format_args!("{}", self.cache_mmu_owner().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_MMU_OWNER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - The bits are used to specify the owner of MMU.bit0/bit2: ibus, bit1/bit3: dbus"]
    #[inline(always)]
    #[must_use]
    pub fn cache_mmu_owner(&mut self) -> CACHE_MMU_OWNER_W<0> {
        CACHE_MMU_OWNER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_mmu_owner](index.html) module"]
pub struct CACHE_MMU_OWNER_SPEC;
impl crate::RegisterSpec for CACHE_MMU_OWNER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_mmu_owner::R](R) reader structure"]
impl crate::Readable for CACHE_MMU_OWNER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_mmu_owner::W](W) writer structure"]
impl crate::Writable for CACHE_MMU_OWNER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_MMU_OWNER to value 0"]
impl crate::Resettable for CACHE_MMU_OWNER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
