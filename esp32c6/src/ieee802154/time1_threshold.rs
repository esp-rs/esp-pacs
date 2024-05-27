///Register `TIME1_THRESHOLD` reader
pub type R = crate::R<TIME1_THRESHOLD_SPEC>;
///Register `TIME1_THRESHOLD` writer
pub type W = crate::W<TIME1_THRESHOLD_SPEC>;
///Field `TIMER1_THRESHOLD` reader -
pub type TIMER1_THRESHOLD_R = crate::FieldReader<u32>;
///Field `TIMER1_THRESHOLD` writer -
pub type TIMER1_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn timer1_threshold(&self) -> TIMER1_THRESHOLD_R {
        TIMER1_THRESHOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME1_THRESHOLD")
            .field("timer1_threshold", &self.timer1_threshold())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn timer1_threshold(&mut self) -> TIMER1_THRESHOLD_W<TIME1_THRESHOLD_SPEC> {
        TIMER1_THRESHOLD_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`time1_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time1_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIME1_THRESHOLD_SPEC;
impl crate::RegisterSpec for TIME1_THRESHOLD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`time1_threshold::R`](R) reader structure
impl crate::Readable for TIME1_THRESHOLD_SPEC {}
///`write(|w| ..)` method takes [`time1_threshold::W`](W) writer structure
impl crate::Writable for TIME1_THRESHOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIME1_THRESHOLD to value 0
impl crate::Resettable for TIME1_THRESHOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
