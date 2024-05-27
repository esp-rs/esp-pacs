///Register `ROM_TABLE_LOCK` reader
pub type R = crate::R<ROM_TABLE_LOCK_SPEC>;
///Register `ROM_TABLE_LOCK` writer
pub type W = crate::W<ROM_TABLE_LOCK_SPEC>;
///Field `ROM_TABLE_LOCK` reader - XXXX
pub type ROM_TABLE_LOCK_R = crate::BitReader;
///Field `ROM_TABLE_LOCK` writer - XXXX
pub type ROM_TABLE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - XXXX
    #[inline(always)]
    pub fn rom_table_lock(&self) -> ROM_TABLE_LOCK_R {
        ROM_TABLE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_TABLE_LOCK")
            .field("rom_table_lock", &self.rom_table_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - XXXX
    #[inline(always)]
    #[must_use]
    pub fn rom_table_lock(&mut self) -> ROM_TABLE_LOCK_W<ROM_TABLE_LOCK_SPEC> {
        ROM_TABLE_LOCK_W::new(self, 0)
    }
}
/**Rom-Table lock register

You can [`read`](crate::generic::Reg::read) this register and get [`rom_table_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_table_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ROM_TABLE_LOCK_SPEC;
impl crate::RegisterSpec for ROM_TABLE_LOCK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rom_table_lock::R`](R) reader structure
impl crate::Readable for ROM_TABLE_LOCK_SPEC {}
///`write(|w| ..)` method takes [`rom_table_lock::W`](W) writer structure
impl crate::Writable for ROM_TABLE_LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROM_TABLE_LOCK to value 0
impl crate::Resettable for ROM_TABLE_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
