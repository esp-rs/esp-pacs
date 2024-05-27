///Register `MEM_ADDR_UPDATE` writer
pub type W = crate::W<MEM_ADDR_UPDATE_SPEC>;
///Field `MEM_CURRENT_ADDR_UPDATE` writer - when set, the will \hyperref\[fielddesc:TRACEMEMCURRENTADDR\]{TRACE_MEM_CURRENT_ADDR} update to \hyperref\[fielddesc:TRACEMEMSTARTADDR\]{TRACE_MEM_START_ADDR}.
pub type MEM_CURRENT_ADDR_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_ADDR_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - when set, the will \hyperref\[fielddesc:TRACEMEMCURRENTADDR\]{TRACE_MEM_CURRENT_ADDR} update to \hyperref\[fielddesc:TRACEMEMSTARTADDR\]{TRACE_MEM_START_ADDR}.
    #[inline(always)]
    #[must_use]
    pub fn mem_current_addr_update(&mut self) -> MEM_CURRENT_ADDR_UPDATE_W<MEM_ADDR_UPDATE_SPEC> {
        MEM_CURRENT_ADDR_UPDATE_W::new(self, 0)
    }
}
/**mem addr update

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_addr_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_ADDR_UPDATE_SPEC;
impl crate::RegisterSpec for MEM_ADDR_UPDATE_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`mem_addr_update::W`](W) writer structure
impl crate::Writable for MEM_ADDR_UPDATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_ADDR_UPDATE to value 0
impl crate::Resettable for MEM_ADDR_UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
