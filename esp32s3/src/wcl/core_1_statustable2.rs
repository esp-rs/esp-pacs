#[doc = "Register `Core_1_STATUSTABLE2` reader"]
pub type R = crate::R<CORE_1_STATUSTABLE2_SPEC>;
#[doc = "Register `Core_1_STATUSTABLE2` writer"]
pub type W = crate::W<CORE_1_STATUSTABLE2_SPEC>;
#[doc = "Field `CORE_1_FROM_WORLD_2` reader - This bit is used to confirm world before enter entry 2"]
pub type CORE_1_FROM_WORLD_2_R = crate::BitReader;
#[doc = "Field `CORE_1_FROM_WORLD_2` writer - This bit is used to confirm world before enter entry 2"]
pub type CORE_1_FROM_WORLD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_FROM_ENTRY_2` reader - This filed is used to confirm in which entry before enter entry 2"]
pub type CORE_1_FROM_ENTRY_2_R = crate::FieldReader;
#[doc = "Field `CORE_1_FROM_ENTRY_2` writer - This filed is used to confirm in which entry before enter entry 2"]
pub type CORE_1_FROM_ENTRY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORE_1_CURRENT_2` reader - This bit is used to confirm whether the current state is in entry 2"]
pub type CORE_1_CURRENT_2_R = crate::BitReader;
#[doc = "Field `CORE_1_CURRENT_2` writer - This bit is used to confirm whether the current state is in entry 2"]
pub type CORE_1_CURRENT_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 2"]
    #[inline(always)]
    pub fn core_1_from_world_2(&self) -> CORE_1_FROM_WORLD_2_R {
        CORE_1_FROM_WORLD_2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 2"]
    #[inline(always)]
    pub fn core_1_from_entry_2(&self) -> CORE_1_FROM_ENTRY_2_R {
        CORE_1_FROM_ENTRY_2_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 2"]
    #[inline(always)]
    pub fn core_1_current_2(&self) -> CORE_1_CURRENT_2_R {
        CORE_1_CURRENT_2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE2")
            .field(
                "core_1_from_world_2",
                &format_args!("{}", self.core_1_from_world_2().bit()),
            )
            .field(
                "core_1_from_entry_2",
                &format_args!("{}", self.core_1_from_entry_2().bits()),
            )
            .field(
                "core_1_current_2",
                &format_args!("{}", self.core_1_current_2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_STATUSTABLE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 2"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_world_2(&mut self) -> CORE_1_FROM_WORLD_2_W<CORE_1_STATUSTABLE2_SPEC> {
        CORE_1_FROM_WORLD_2_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 2"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_entry_2(&mut self) -> CORE_1_FROM_ENTRY_2_W<CORE_1_STATUSTABLE2_SPEC> {
        CORE_1_FROM_ENTRY_2_W::new(self, 1)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 2"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_current_2(&mut self) -> CORE_1_CURRENT_2_W<CORE_1_STATUSTABLE2_SPEC> {
        CORE_1_CURRENT_2_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status register of world switch of entry 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_statustable2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_statustable2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_STATUSTABLE2_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_statustable2::R`](R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_statustable2::W`](W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE2 to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
