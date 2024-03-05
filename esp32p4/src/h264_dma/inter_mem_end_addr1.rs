#[doc = "Register `INTER_MEM_END_ADDR1` reader"]
pub type R = crate::R<INTER_MEM_END_ADDR1_SPEC>;
#[doc = "Register `INTER_MEM_END_ADDR1` writer"]
pub type W = crate::W<INTER_MEM_END_ADDR1_SPEC>;
#[doc = "Field `ACCESS_INTER_MEM_END_ADDR1` reader - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type ACCESS_INTER_MEM_END_ADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_INTER_MEM_END_ADDR1` writer - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type ACCESS_INTER_MEM_END_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_inter_mem_end_addr1(&self) -> ACCESS_INTER_MEM_END_ADDR1_R {
        ACCESS_INTER_MEM_END_ADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTER_MEM_END_ADDR1")
            .field(
                "access_inter_mem_end_addr1",
                &format_args!("{}", self.access_inter_mem_end_addr1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTER_MEM_END_ADDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    #[must_use]
    pub fn access_inter_mem_end_addr1(
        &mut self,
    ) -> ACCESS_INTER_MEM_END_ADDR1_W<INTER_MEM_END_ADDR1_SPEC> {
        ACCESS_INTER_MEM_END_ADDR1_W::new(self, 0)
    }
}
#[doc = "end address of inter memory range1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inter_mem_end_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inter_mem_end_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTER_MEM_END_ADDR1_SPEC;
impl crate::RegisterSpec for INTER_MEM_END_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inter_mem_end_addr1::R`](R) reader structure"]
impl crate::Readable for INTER_MEM_END_ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inter_mem_end_addr1::W`](W) writer structure"]
impl crate::Writable for INTER_MEM_END_ADDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTER_MEM_END_ADDR1 to value 0x8fff_ffff"]
impl crate::Resettable for INTER_MEM_END_ADDR1_SPEC {
    const RESET_VALUE: u32 = 0x8fff_ffff;
}
