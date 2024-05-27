///Register `INTER_MEM_START_ADDR1` reader
pub type R = crate::R<INTER_MEM_START_ADDR1_SPEC>;
///Register `INTER_MEM_START_ADDR1` writer
pub type W = crate::W<INTER_MEM_START_ADDR1_SPEC>;
///Field `ACCESS_INTER_MEM_START_ADDR1` reader - The start address of accessible address space.
pub type ACCESS_INTER_MEM_START_ADDR1_R = crate::FieldReader<u32>;
///Field `ACCESS_INTER_MEM_START_ADDR1` writer - The start address of accessible address space.
pub type ACCESS_INTER_MEM_START_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The start address of accessible address space.
    #[inline(always)]
    pub fn access_inter_mem_start_addr1(&self) -> ACCESS_INTER_MEM_START_ADDR1_R {
        ACCESS_INTER_MEM_START_ADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTER_MEM_START_ADDR1")
            .field(
                "access_inter_mem_start_addr1",
                &self.access_inter_mem_start_addr1(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The start address of accessible address space.
    #[inline(always)]
    #[must_use]
    pub fn access_inter_mem_start_addr1(
        &mut self,
    ) -> ACCESS_INTER_MEM_START_ADDR1_W<INTER_MEM_START_ADDR1_SPEC> {
        ACCESS_INTER_MEM_START_ADDR1_W::new(self, 0)
    }
}
/**Start address of inter memory range1 register

You can [`read`](crate::generic::Reg::read) this register and get [`inter_mem_start_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inter_mem_start_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTER_MEM_START_ADDR1_SPEC;
impl crate::RegisterSpec for INTER_MEM_START_ADDR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`inter_mem_start_addr1::R`](R) reader structure
impl crate::Readable for INTER_MEM_START_ADDR1_SPEC {}
///`write(|w| ..)` method takes [`inter_mem_start_addr1::W`](W) writer structure
impl crate::Writable for INTER_MEM_START_ADDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTER_MEM_START_ADDR1 to value 0x3010_0000
impl crate::Resettable for INTER_MEM_START_ADDR1_SPEC {
    const RESET_VALUE: u32 = 0x3010_0000;
}
