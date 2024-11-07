#[doc = "Register `Core_1_STATUSTABLE12` reader"]
pub type R = crate::R<CORE_1_STATUSTABLE12_SPEC>;
#[doc = "Register `Core_1_STATUSTABLE12` writer"]
pub type W = crate::W<CORE_1_STATUSTABLE12_SPEC>;
#[doc = "Field `CORE_1_FROM_WORLD_12` reader - This bit is used to confirm world before enter entry 12"]
pub type CORE_1_FROM_WORLD_12_R = crate::BitReader;
#[doc = "Field `CORE_1_FROM_WORLD_12` writer - This bit is used to confirm world before enter entry 12"]
pub type CORE_1_FROM_WORLD_12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_1_FROM_ENTRY_12` reader - This filed is used to confirm in which entry before enter entry 12"]
pub type CORE_1_FROM_ENTRY_12_R = crate::FieldReader;
#[doc = "Field `CORE_1_FROM_ENTRY_12` writer - This filed is used to confirm in which entry before enter entry 12"]
pub type CORE_1_FROM_ENTRY_12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORE_1_CURRENT_12` reader - This bit is used to confirm whether the current state is in entry 12"]
pub type CORE_1_CURRENT_12_R = crate::BitReader;
#[doc = "Field `CORE_1_CURRENT_12` writer - This bit is used to confirm whether the current state is in entry 12"]
pub type CORE_1_CURRENT_12_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_world_12(&self) -> CORE_1_FROM_WORLD_12_R {
        CORE_1_FROM_WORLD_12_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_entry_12(&self) -> CORE_1_FROM_ENTRY_12_R {
        CORE_1_FROM_ENTRY_12_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 12"]
    #[inline(always)]
    pub fn core_1_current_12(&self) -> CORE_1_CURRENT_12_R {
        CORE_1_CURRENT_12_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE12")
            .field("core_1_from_world_12", &self.core_1_from_world_12())
            .field("core_1_from_entry_12", &self.core_1_from_entry_12())
            .field("core_1_current_12", &self.core_1_current_12())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_world_12(&mut self) -> CORE_1_FROM_WORLD_12_W<CORE_1_STATUSTABLE12_SPEC> {
        CORE_1_FROM_WORLD_12_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 12"]
    #[inline(always)]
    pub fn core_1_from_entry_12(&mut self) -> CORE_1_FROM_ENTRY_12_W<CORE_1_STATUSTABLE12_SPEC> {
        CORE_1_FROM_ENTRY_12_W::new(self, 1)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 12"]
    #[inline(always)]
    pub fn core_1_current_12(&mut self) -> CORE_1_CURRENT_12_W<CORE_1_STATUSTABLE12_SPEC> {
        CORE_1_CURRENT_12_W::new(self, 5)
    }
}
#[doc = "Status register of world switch of entry 12\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_statustable12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_statustable12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_STATUSTABLE12_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_statustable12::R`](R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_statustable12::W`](W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE12 to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE12_SPEC {
    const RESET_VALUE: u32 = 0;
}
