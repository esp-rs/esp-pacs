///Register `EXTER_MEM_START_ADDR0` reader
pub type R = crate::R<EXTER_MEM_START_ADDR0_SPEC>;
///Register `EXTER_MEM_START_ADDR0` writer
pub type W = crate::W<EXTER_MEM_START_ADDR0_SPEC>;
///Field `ACCESS_EXTER_MEM_START_ADDR0` reader - The start address of accessible address space.
pub type ACCESS_EXTER_MEM_START_ADDR0_R = crate::FieldReader<u32>;
///Field `ACCESS_EXTER_MEM_START_ADDR0` writer - The start address of accessible address space.
pub type ACCESS_EXTER_MEM_START_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The start address of accessible address space.
    #[inline(always)]
    pub fn access_exter_mem_start_addr0(&self) -> ACCESS_EXTER_MEM_START_ADDR0_R {
        ACCESS_EXTER_MEM_START_ADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTER_MEM_START_ADDR0")
            .field(
                "access_exter_mem_start_addr0",
                &self.access_exter_mem_start_addr0(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The start address of accessible address space.
    #[inline(always)]
    #[must_use]
    pub fn access_exter_mem_start_addr0(
        &mut self,
    ) -> ACCESS_EXTER_MEM_START_ADDR0_W<EXTER_MEM_START_ADDR0_SPEC> {
        ACCESS_EXTER_MEM_START_ADDR0_W::new(self, 0)
    }
}
/**Start address of exter memory range0 register

You can [`read`](crate::generic::Reg::read) this register and get [`exter_mem_start_addr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exter_mem_start_addr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXTER_MEM_START_ADDR0_SPEC;
impl crate::RegisterSpec for EXTER_MEM_START_ADDR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`exter_mem_start_addr0::R`](R) reader structure
impl crate::Readable for EXTER_MEM_START_ADDR0_SPEC {}
///`write(|w| ..)` method takes [`exter_mem_start_addr0::W`](W) writer structure
impl crate::Writable for EXTER_MEM_START_ADDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTER_MEM_START_ADDR0 to value 0x3010_0000
impl crate::Resettable for EXTER_MEM_START_ADDR0_SPEC {
    const RESET_VALUE: u32 = 0x3010_0000;
}
