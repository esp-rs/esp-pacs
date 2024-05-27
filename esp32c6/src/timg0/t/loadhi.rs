///Register `LOADHI` reader
pub type R = crate::R<LOADHI_SPEC>;
///Register `LOADHI` writer
pub type W = crate::W<LOADHI_SPEC>;
///Field `LOAD_HI` reader - High 22 bits of the value that a reload will load onto timer %s time-base counter.
pub type LOAD_HI_R = crate::FieldReader<u32>;
///Field `LOAD_HI` writer - High 22 bits of the value that a reload will load onto timer %s time-base counter.
pub type LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - High 22 bits of the value that a reload will load onto timer %s time-base counter.
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOADHI")
            .field("load_hi", &self.load_hi())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - High 22 bits of the value that a reload will load onto timer %s time-base counter.
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<LOADHI_SPEC> {
        LOAD_HI_W::new(self, 0)
    }
}
/**Timer %s reload value, high 22 bits

You can [`read`](crate::generic::Reg::read) this register and get [`loadhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loadhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOADHI_SPEC;
impl crate::RegisterSpec for LOADHI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`loadhi::R`](R) reader structure
impl crate::Readable for LOADHI_SPEC {}
///`write(|w| ..)` method takes [`loadhi::W`](W) writer structure
impl crate::Writable for LOADHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOADHI to value 0
impl crate::Resettable for LOADHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
