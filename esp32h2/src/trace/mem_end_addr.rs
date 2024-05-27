///Register `MEM_END_ADDR` reader
pub type R = crate::R<MEM_END_ADDR_SPEC>;
///Register `MEM_END_ADDR` writer
pub type W = crate::W<MEM_END_ADDR_SPEC>;
///Field `MEM_END_ADDR` reader - The end address of trace memory
pub type MEM_END_ADDR_R = crate::FieldReader<u32>;
///Field `MEM_END_ADDR` writer - The end address of trace memory
pub type MEM_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The end address of trace memory
    #[inline(always)]
    pub fn mem_end_addr(&self) -> MEM_END_ADDR_R {
        MEM_END_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_END_ADDR")
            .field("mem_end_addr", &self.mem_end_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The end address of trace memory
    #[inline(always)]
    #[must_use]
    pub fn mem_end_addr(&mut self) -> MEM_END_ADDR_W<MEM_END_ADDR_SPEC> {
        MEM_END_ADDR_W::new(self, 0)
    }
}
/**mem end addr

You can [`read`](crate::generic::Reg::read) this register and get [`mem_end_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_end_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_END_ADDR_SPEC;
impl crate::RegisterSpec for MEM_END_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_end_addr::R`](R) reader structure
impl crate::Readable for MEM_END_ADDR_SPEC {}
///`write(|w| ..)` method takes [`mem_end_addr::W`](W) writer structure
impl crate::Writable for MEM_END_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_END_ADDR to value 0xffff_ffff
impl crate::Resettable for MEM_END_ADDR_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
