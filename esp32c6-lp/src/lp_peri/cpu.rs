///Register `CPU` reader
pub type R = crate::R<CPU_SPEC>;
///Register `CPU` writer
pub type W = crate::W<CPU_SPEC>;
///Field `LPCORE_DBGM_UNAVALIABLE` reader - need_des
pub type LPCORE_DBGM_UNAVALIABLE_R = crate::BitReader;
///Field `LPCORE_DBGM_UNAVALIABLE` writer - need_des
pub type LPCORE_DBGM_UNAVALIABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn lpcore_dbgm_unavaliable(&self) -> LPCORE_DBGM_UNAVALIABLE_R {
        LPCORE_DBGM_UNAVALIABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU")
            .field("lpcore_dbgm_unavaliable", &self.lpcore_dbgm_unavaliable())
            .finish()
    }
}
impl W {
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lpcore_dbgm_unavaliable(&mut self) -> LPCORE_DBGM_UNAVALIABLE_W<CPU_SPEC> {
        LPCORE_DBGM_UNAVALIABLE_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`cpu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CPU_SPEC;
impl crate::RegisterSpec for CPU_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cpu::R`](R) reader structure
impl crate::Readable for CPU_SPEC {}
///`write(|w| ..)` method takes [`cpu::W`](W) writer structure
impl crate::Writable for CPU_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CPU to value 0x8000_0000
impl crate::Resettable for CPU_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
