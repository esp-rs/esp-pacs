#[doc = "Register `Core_0_STATUSTABLE2` reader"]
pub struct R(crate::R<CORE_0_STATUSTABLE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_STATUSTABLE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_STATUSTABLE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_STATUSTABLE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_0_STATUSTABLE2` writer"]
pub struct W(crate::W<CORE_0_STATUSTABLE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_STATUSTABLE2_SPEC>;
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
impl From<crate::W<CORE_0_STATUSTABLE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_STATUSTABLE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_FROM_WORLD_2` reader - This bit is used to confirm world before enter entry 2"]
pub type CORE_0_FROM_WORLD_2_R = crate::BitReader;
#[doc = "Field `CORE_0_FROM_WORLD_2` writer - This bit is used to confirm world before enter entry 2"]
pub type CORE_0_FROM_WORLD_2_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_STATUSTABLE2_SPEC, O>;
#[doc = "Field `CORE_0_FROM_ENTRY_2` reader - This filed is used to confirm in which entry before enter entry 2"]
pub type CORE_0_FROM_ENTRY_2_R = crate::FieldReader;
#[doc = "Field `CORE_0_FROM_ENTRY_2` writer - This filed is used to confirm in which entry before enter entry 2"]
pub type CORE_0_FROM_ENTRY_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_0_STATUSTABLE2_SPEC, 4, O>;
#[doc = "Field `CORE_0_CURRENT_2` reader - This bit is used to confirm whether the current state is in entry 2"]
pub type CORE_0_CURRENT_2_R = crate::BitReader;
#[doc = "Field `CORE_0_CURRENT_2` writer - This bit is used to confirm whether the current state is in entry 2"]
pub type CORE_0_CURRENT_2_W<'a, const O: u8> = crate::BitWriter<'a, CORE_0_STATUSTABLE2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 2"]
    #[inline(always)]
    pub fn core_0_from_world_2(&self) -> CORE_0_FROM_WORLD_2_R {
        CORE_0_FROM_WORLD_2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 2"]
    #[inline(always)]
    pub fn core_0_from_entry_2(&self) -> CORE_0_FROM_ENTRY_2_R {
        CORE_0_FROM_ENTRY_2_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 2"]
    #[inline(always)]
    pub fn core_0_current_2(&self) -> CORE_0_CURRENT_2_R {
        CORE_0_CURRENT_2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_STATUSTABLE2")
            .field(
                "core_0_from_world_2",
                &format_args!("{}", self.core_0_from_world_2().bit()),
            )
            .field(
                "core_0_from_entry_2",
                &format_args!("{}", self.core_0_from_entry_2().bits()),
            )
            .field(
                "core_0_current_2",
                &format_args!("{}", self.core_0_current_2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_STATUSTABLE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 2"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_world_2(&mut self) -> CORE_0_FROM_WORLD_2_W<0> {
        CORE_0_FROM_WORLD_2_W::new(self)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 2"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_from_entry_2(&mut self) -> CORE_0_FROM_ENTRY_2_W<1> {
        CORE_0_FROM_ENTRY_2_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 2"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_current_2(&mut self) -> CORE_0_CURRENT_2_W<5> {
        CORE_0_CURRENT_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register of world switch of entry 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_statustable2](index.html) module"]
pub struct CORE_0_STATUSTABLE2_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_statustable2::R](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_statustable2::W](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE2 to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
