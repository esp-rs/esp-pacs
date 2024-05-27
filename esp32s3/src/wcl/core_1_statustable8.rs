///Register `Core_1_STATUSTABLE8` reader
pub type R = crate::R<CORE_1_STATUSTABLE8_SPEC>;
///Register `Core_1_STATUSTABLE8` writer
pub type W = crate::W<CORE_1_STATUSTABLE8_SPEC>;
///Field `CORE_1_FROM_WORLD_8` reader - This bit is used to confirm world before enter entry 8
pub type CORE_1_FROM_WORLD_8_R = crate::BitReader;
///Field `CORE_1_FROM_WORLD_8` writer - This bit is used to confirm world before enter entry 8
pub type CORE_1_FROM_WORLD_8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_FROM_ENTRY_8` reader - This filed is used to confirm in which entry before enter entry 8
pub type CORE_1_FROM_ENTRY_8_R = crate::FieldReader;
///Field `CORE_1_FROM_ENTRY_8` writer - This filed is used to confirm in which entry before enter entry 8
pub type CORE_1_FROM_ENTRY_8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CORE_1_CURRENT_8` reader - This bit is used to confirm whether the current state is in entry 8
pub type CORE_1_CURRENT_8_R = crate::BitReader;
///Field `CORE_1_CURRENT_8` writer - This bit is used to confirm whether the current state is in entry 8
pub type CORE_1_CURRENT_8_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This bit is used to confirm world before enter entry 8
    #[inline(always)]
    pub fn core_1_from_world_8(&self) -> CORE_1_FROM_WORLD_8_R {
        CORE_1_FROM_WORLD_8_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - This filed is used to confirm in which entry before enter entry 8
    #[inline(always)]
    pub fn core_1_from_entry_8(&self) -> CORE_1_FROM_ENTRY_8_R {
        CORE_1_FROM_ENTRY_8_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5 - This bit is used to confirm whether the current state is in entry 8
    #[inline(always)]
    pub fn core_1_current_8(&self) -> CORE_1_CURRENT_8_R {
        CORE_1_CURRENT_8_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE8")
            .field("core_1_from_world_8", &self.core_1_from_world_8())
            .field("core_1_from_entry_8", &self.core_1_from_entry_8())
            .field("core_1_current_8", &self.core_1_current_8())
            .finish()
    }
}
impl W {
    ///Bit 0 - This bit is used to confirm world before enter entry 8
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_world_8(&mut self) -> CORE_1_FROM_WORLD_8_W<CORE_1_STATUSTABLE8_SPEC> {
        CORE_1_FROM_WORLD_8_W::new(self, 0)
    }
    ///Bits 1:4 - This filed is used to confirm in which entry before enter entry 8
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_entry_8(&mut self) -> CORE_1_FROM_ENTRY_8_W<CORE_1_STATUSTABLE8_SPEC> {
        CORE_1_FROM_ENTRY_8_W::new(self, 1)
    }
    ///Bit 5 - This bit is used to confirm whether the current state is in entry 8
    #[inline(always)]
    #[must_use]
    pub fn core_1_current_8(&mut self) -> CORE_1_CURRENT_8_W<CORE_1_STATUSTABLE8_SPEC> {
        CORE_1_CURRENT_8_W::new(self, 5)
    }
}
/**Status register of world switch of entry 8

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_statustable8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_statustable8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_STATUSTABLE8_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_statustable8::R`](R) reader structure
impl crate::Readable for CORE_1_STATUSTABLE8_SPEC {}
///`write(|w| ..)` method takes [`core_1_statustable8::W`](W) writer structure
impl crate::Writable for CORE_1_STATUSTABLE8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets Core_1_STATUSTABLE8 to value 0
impl crate::Resettable for CORE_1_STATUSTABLE8_SPEC {
    const RESET_VALUE: u32 = 0;
}
