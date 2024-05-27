///Register `OCCUPY_3` reader
pub type R = crate::R<OCCUPY_3_SPEC>;
///Register `OCCUPY_3` writer
pub type W = crate::W<OCCUPY_3_SPEC>;
///Field `OCCUPY_PRO_TRACE` reader - Configure one block of SRAM Block 4-21 is used as trace memory.
pub type OCCUPY_PRO_TRACE_R = crate::FieldReader<u32>;
///Field `OCCUPY_PRO_TRACE` writer - Configure one block of SRAM Block 4-21 is used as trace memory.
pub type OCCUPY_PRO_TRACE_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    ///Bits 0:17 - Configure one block of SRAM Block 4-21 is used as trace memory.
    #[inline(always)]
    pub fn occupy_pro_trace(&self) -> OCCUPY_PRO_TRACE_R {
        OCCUPY_PRO_TRACE_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OCCUPY_3")
            .field("occupy_pro_trace", &self.occupy_pro_trace())
            .finish()
    }
}
impl W {
    ///Bits 0:17 - Configure one block of SRAM Block 4-21 is used as trace memory.
    #[inline(always)]
    #[must_use]
    pub fn occupy_pro_trace(&mut self) -> OCCUPY_PRO_TRACE_W<OCCUPY_3_SPEC> {
        OCCUPY_PRO_TRACE_W::new(self, 0)
    }
}
/**Occupy permission control register 3.

You can [`read`](crate::generic::Reg::read) this register and get [`occupy_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`occupy_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OCCUPY_3_SPEC;
impl crate::RegisterSpec for OCCUPY_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`occupy_3::R`](R) reader structure
impl crate::Readable for OCCUPY_3_SPEC {}
///`write(|w| ..)` method takes [`occupy_3::W`](W) writer structure
impl crate::Writable for OCCUPY_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OCCUPY_3 to value 0
impl crate::Resettable for OCCUPY_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
