#[doc = "Register `Core_1_STATUSTABLE9` reader"]
pub struct R(crate::R<CORE_1_STATUSTABLE9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_STATUSTABLE9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_STATUSTABLE9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_STATUSTABLE9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_STATUSTABLE9` writer"]
pub struct W(crate::W<CORE_1_STATUSTABLE9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_STATUSTABLE9_SPEC>;
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
impl From<crate::W<CORE_1_STATUSTABLE9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_STATUSTABLE9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_FROM_WORLD_9` reader - This bit is used to confirm world before enter entry 9"]
pub type CORE_1_FROM_WORLD_9_R = crate::BitReader;
#[doc = "Field `CORE_1_FROM_WORLD_9` writer - This bit is used to confirm world before enter entry 9"]
pub type CORE_1_FROM_WORLD_9_W<'a, const O: u8> = crate::BitWriter<'a, CORE_1_STATUSTABLE9_SPEC, O>;
#[doc = "Field `CORE_1_FROM_ENTRY_9` reader - This filed is used to confirm in which entry before enter entry 9"]
pub type CORE_1_FROM_ENTRY_9_R = crate::FieldReader;
#[doc = "Field `CORE_1_FROM_ENTRY_9` writer - This filed is used to confirm in which entry before enter entry 9"]
pub type CORE_1_FROM_ENTRY_9_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_STATUSTABLE9_SPEC, 4, O>;
#[doc = "Field `CORE_1_CURRENT_9` reader - This bit is used to confirm whether the current state is in entry 9"]
pub type CORE_1_CURRENT_9_R = crate::BitReader;
#[doc = "Field `CORE_1_CURRENT_9` writer - This bit is used to confirm whether the current state is in entry 9"]
pub type CORE_1_CURRENT_9_W<'a, const O: u8> = crate::BitWriter<'a, CORE_1_STATUSTABLE9_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 9"]
    #[inline(always)]
    pub fn core_1_from_world_9(&self) -> CORE_1_FROM_WORLD_9_R {
        CORE_1_FROM_WORLD_9_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 9"]
    #[inline(always)]
    pub fn core_1_from_entry_9(&self) -> CORE_1_FROM_ENTRY_9_R {
        CORE_1_FROM_ENTRY_9_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 9"]
    #[inline(always)]
    pub fn core_1_current_9(&self) -> CORE_1_CURRENT_9_R {
        CORE_1_CURRENT_9_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE9")
            .field(
                "core_1_from_world_9",
                &format_args!("{}", self.core_1_from_world_9().bit()),
            )
            .field(
                "core_1_from_entry_9",
                &format_args!("{}", self.core_1_from_entry_9().bits()),
            )
            .field(
                "core_1_current_9",
                &format_args!("{}", self.core_1_current_9().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_STATUSTABLE9_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 9"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_world_9(&mut self) -> CORE_1_FROM_WORLD_9_W<0> {
        CORE_1_FROM_WORLD_9_W::new(self)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 9"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_entry_9(&mut self) -> CORE_1_FROM_ENTRY_9_W<1> {
        CORE_1_FROM_ENTRY_9_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 9"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_current_9(&mut self) -> CORE_1_CURRENT_9_W<5> {
        CORE_1_CURRENT_9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register of world switch of entry 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_statustable9](index.html) module"]
pub struct CORE_1_STATUSTABLE9_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_statustable9::R](R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_statustable9::W](W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE9 to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
