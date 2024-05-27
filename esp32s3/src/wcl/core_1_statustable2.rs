///Register `Core_1_STATUSTABLE2` reader
pub type R = crate::R<CORE_1_STATUSTABLE2_SPEC>;
///Register `Core_1_STATUSTABLE2` writer
pub type W = crate::W<CORE_1_STATUSTABLE2_SPEC>;
///Field `CORE_1_FROM_WORLD_2` reader - This bit is used to confirm world before enter entry 2
pub type CORE_1_FROM_WORLD_2_R = crate::BitReader;
///Field `CORE_1_FROM_WORLD_2` writer - This bit is used to confirm world before enter entry 2
pub type CORE_1_FROM_WORLD_2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORE_1_FROM_ENTRY_2` reader - This filed is used to confirm in which entry before enter entry 2
pub type CORE_1_FROM_ENTRY_2_R = crate::FieldReader;
///Field `CORE_1_FROM_ENTRY_2` writer - This filed is used to confirm in which entry before enter entry 2
pub type CORE_1_FROM_ENTRY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CORE_1_CURRENT_2` reader - This bit is used to confirm whether the current state is in entry 2
pub type CORE_1_CURRENT_2_R = crate::BitReader;
///Field `CORE_1_CURRENT_2` writer - This bit is used to confirm whether the current state is in entry 2
pub type CORE_1_CURRENT_2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This bit is used to confirm world before enter entry 2
    #[inline(always)]
    pub fn core_1_from_world_2(&self) -> CORE_1_FROM_WORLD_2_R {
        CORE_1_FROM_WORLD_2_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - This filed is used to confirm in which entry before enter entry 2
    #[inline(always)]
    pub fn core_1_from_entry_2(&self) -> CORE_1_FROM_ENTRY_2_R {
        CORE_1_FROM_ENTRY_2_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 5 - This bit is used to confirm whether the current state is in entry 2
    #[inline(always)]
    pub fn core_1_current_2(&self) -> CORE_1_CURRENT_2_R {
        CORE_1_CURRENT_2_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE2")
            .field("core_1_from_world_2", &self.core_1_from_world_2())
            .field("core_1_from_entry_2", &self.core_1_from_entry_2())
            .field("core_1_current_2", &self.core_1_current_2())
            .finish()
    }
}
impl W {
    ///Bit 0 - This bit is used to confirm world before enter entry 2
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_world_2(&mut self) -> CORE_1_FROM_WORLD_2_W<CORE_1_STATUSTABLE2_SPEC> {
        CORE_1_FROM_WORLD_2_W::new(self, 0)
    }
    ///Bits 1:4 - This filed is used to confirm in which entry before enter entry 2
    #[inline(always)]
    #[must_use]
    pub fn core_1_from_entry_2(&mut self) -> CORE_1_FROM_ENTRY_2_W<CORE_1_STATUSTABLE2_SPEC> {
        CORE_1_FROM_ENTRY_2_W::new(self, 1)
    }
    ///Bit 5 - This bit is used to confirm whether the current state is in entry 2
    #[inline(always)]
    #[must_use]
    pub fn core_1_current_2(&mut self) -> CORE_1_CURRENT_2_W<CORE_1_STATUSTABLE2_SPEC> {
        CORE_1_CURRENT_2_W::new(self, 5)
    }
}
/**Status register of world switch of entry 2

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_statustable2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_statustable2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_STATUSTABLE2_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_statustable2::R`](R) reader structure
impl crate::Readable for CORE_1_STATUSTABLE2_SPEC {}
///`write(|w| ..)` method takes [`core_1_statustable2::W`](W) writer structure
impl crate::Writable for CORE_1_STATUSTABLE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets Core_1_STATUSTABLE2 to value 0
impl crate::Resettable for CORE_1_STATUSTABLE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
