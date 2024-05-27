///Register `DSTATAR0` reader
pub type R = crate::R<DSTATAR0_SPEC>;
///Register `DSTATAR0` writer
pub type W = crate::W<DSTATAR0_SPEC>;
///Field `CH1_DSTATAR0` reader - NA
pub type CH1_DSTATAR0_R = crate::FieldReader<u32>;
///Field `CH1_DSTATAR0` writer - NA
pub type CH1_DSTATAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - NA
    #[inline(always)]
    pub fn ch1_dstatar0(&self) -> CH1_DSTATAR0_R {
        CH1_DSTATAR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTATAR0")
            .field("ch1_dstatar0", &self.ch1_dstatar0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_dstatar0(&mut self) -> CH1_DSTATAR0_W<DSTATAR0_SPEC> {
        CH1_DSTATAR0_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`dstatar0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dstatar0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DSTATAR0_SPEC;
impl crate::RegisterSpec for DSTATAR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dstatar0::R`](R) reader structure
impl crate::Readable for DSTATAR0_SPEC {}
///`write(|w| ..)` method takes [`dstatar0::W`](W) writer structure
impl crate::Writable for DSTATAR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DSTATAR0 to value 0
impl crate::Resettable for DSTATAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
