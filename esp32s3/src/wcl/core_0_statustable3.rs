#[doc = "Register `Core_0_STATUSTABLE3` reader"]
pub type R = crate::R<CORE_0_STATUSTABLE3_SPEC>;
#[doc = "Register `Core_0_STATUSTABLE3` writer"]
pub type W = crate::W<CORE_0_STATUSTABLE3_SPEC>;
#[doc = "Field `CORE_0_FROM_WORLD_3` reader - This bit is used to confirm world before enter entry 3"]
pub type CORE_0_FROM_WORLD_3_R = crate::BitReader;
#[doc = "Field `CORE_0_FROM_WORLD_3` writer - This bit is used to confirm world before enter entry 3"]
pub type CORE_0_FROM_WORLD_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_0_FROM_ENTRY_3` reader - This filed is used to confirm in which entry before enter entry 3"]
pub type CORE_0_FROM_ENTRY_3_R = crate::FieldReader;
#[doc = "Field `CORE_0_FROM_ENTRY_3` writer - This filed is used to confirm in which entry before enter entry 3"]
pub type CORE_0_FROM_ENTRY_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CORE_0_CURRENT_3` reader - This bit is used to confirm whether the current state is in entry 3"]
pub type CORE_0_CURRENT_3_R = crate::BitReader;
#[doc = "Field `CORE_0_CURRENT_3` writer - This bit is used to confirm whether the current state is in entry 3"]
pub type CORE_0_CURRENT_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 3"]
    #[inline(always)]
    pub fn core_0_from_world_3(&self) -> CORE_0_FROM_WORLD_3_R {
        CORE_0_FROM_WORLD_3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 3"]
    #[inline(always)]
    pub fn core_0_from_entry_3(&self) -> CORE_0_FROM_ENTRY_3_R {
        CORE_0_FROM_ENTRY_3_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 3"]
    #[inline(always)]
    pub fn core_0_current_3(&self) -> CORE_0_CURRENT_3_R {
        CORE_0_CURRENT_3_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_STATUSTABLE3")
            .field("core_0_from_world_3", &self.core_0_from_world_3())
            .field("core_0_from_entry_3", &self.core_0_from_entry_3())
            .field("core_0_current_3", &self.core_0_current_3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to confirm world before enter entry 3"]
    #[inline(always)]
    pub fn core_0_from_world_3(&mut self) -> CORE_0_FROM_WORLD_3_W<CORE_0_STATUSTABLE3_SPEC> {
        CORE_0_FROM_WORLD_3_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This filed is used to confirm in which entry before enter entry 3"]
    #[inline(always)]
    pub fn core_0_from_entry_3(&mut self) -> CORE_0_FROM_ENTRY_3_W<CORE_0_STATUSTABLE3_SPEC> {
        CORE_0_FROM_ENTRY_3_W::new(self, 1)
    }
    #[doc = "Bit 5 - This bit is used to confirm whether the current state is in entry 3"]
    #[inline(always)]
    pub fn core_0_current_3(&mut self) -> CORE_0_CURRENT_3_W<CORE_0_STATUSTABLE3_SPEC> {
        CORE_0_CURRENT_3_W::new(self, 5)
    }
}
#[doc = "Status register of world switch of entry 3\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_statustable3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_statustable3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_STATUSTABLE3_SPEC;
impl crate::RegisterSpec for CORE_0_STATUSTABLE3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_statustable3::R`](R) reader structure"]
impl crate::Readable for CORE_0_STATUSTABLE3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_statustable3::W`](W) writer structure"]
impl crate::Writable for CORE_0_STATUSTABLE3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_0_STATUSTABLE3 to value 0"]
impl crate::Resettable for CORE_0_STATUSTABLE3_SPEC {}
