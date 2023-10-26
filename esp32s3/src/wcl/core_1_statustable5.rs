#[doc = "Register `Core_1_STATUSTABLE5` reader"]
pub type R = crate::R<CORE_1_STATUSTABLE5_SPEC>;
#[doc = "Register `Core_1_STATUSTABLE5` writer"]
pub type W = crate::W<CORE_1_STATUSTABLE5_SPEC>;
#[doc = "Field `CORE_1_FROM_WORLD_5` reader - This bit is used to confirm world before enter entry 5"]
pub type CORE_1_FROM_WORLD_5_R = crate::BitReader;
#[doc = "Field `CORE_1_FROM_WORLD_5` writer - This bit is used to confirm world before enter entry 5"]
pub type CORE_1_FROM_WORLD_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CORE_1_FROM_ENTRY_5` reader - This filed is used to confirm in which entry before enter entry 5"]
pub type CORE_1_FROM_ENTRY_5_R = crate::FieldReader;
#[doc = "Field `CORE_1_FROM_ENTRY_5` writer - This filed is used to confirm in which entry before enter entry 5"]
pub type CORE_1_FROM_ENTRY_5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CORE_1_CURRENT_5` reader - This bit is used to confirm whether the current state is in entry 5"]
pub type CORE_1_CURRENT_5_R = crate::BitReader;
#[doc = "Field `CORE_1_CURRENT_5` writer - This bit is used to confirm whether the current state is in entry 5"]
pub type CORE_1_CURRENT_5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 5"]
    #[inline(always)]
    pub fn core_1_from_world_5(&self) -> CORE_1_FROM_WORLD_5_R {
        CORE_1_FROM_WORLD_5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 5"]
    #[inline(always)]
    pub fn core_1_from_entry_5(&self) -> CORE_1_FROM_ENTRY_5_R {
        CORE_1_FROM_ENTRY_5_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 5"]
    #[inline(always)]
    pub fn core_1_current_5(&self) -> CORE_1_CURRENT_5_R {
        CORE_1_CURRENT_5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE5")
            .field(
                "core_1_from_world_5",
                &format_args!("{}", self.core_1_from_world_5().bit()),
            )
            .field(
                "core_1_from_entry_5",
                &format_args!("{}", self.core_1_from_entry_5().bits()),
            )
            .field(
                "core_1_current_5",
                &format_args!("{}", self.core_1_current_5().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_STATUSTABLE5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 5"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_world_5(&mut self) -> CORE_1_FROM_WORLD_5_W<CORE_1_STATUSTABLE5_SPEC, 0> {
        CORE_1_FROM_WORLD_5_W::new(self)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 5"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_entry_5(&mut self) -> CORE_1_FROM_ENTRY_5_W<CORE_1_STATUSTABLE5_SPEC, 1> {
        CORE_1_FROM_ENTRY_5_W::new(self)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 5"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_current_5(&mut self) -> CORE_1_CURRENT_5_W<CORE_1_STATUSTABLE5_SPEC, 5> {
        CORE_1_CURRENT_5_W::new(self)
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
#[doc = "Status register of world switch of entry 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_statustable5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_statustable5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_STATUSTABLE5_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_statustable5::R`](R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_statustable5::W`](W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE5 to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
