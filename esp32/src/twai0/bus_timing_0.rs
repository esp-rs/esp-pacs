///Register `BUS_TIMING_0` reader
pub type R = crate::R<BUS_TIMING_0_SPEC>;
///Register `BUS_TIMING_0` writer
pub type W = crate::W<BUS_TIMING_0_SPEC>;
///Field `BAUD_PRESC` reader - Baud Rate Prescaler, determines the frequency dividing ratio.
pub type BAUD_PRESC_R = crate::FieldReader;
///Field `BAUD_PRESC` writer - Baud Rate Prescaler, determines the frequency dividing ratio.
pub type BAUD_PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SYNC_JUMP_WIDTH` reader - Synchronization Jump Width (SJW), 1 \verb+~+ 14 Tq wide.
pub type SYNC_JUMP_WIDTH_R = crate::FieldReader;
///Field `SYNC_JUMP_WIDTH` writer - Synchronization Jump Width (SJW), 1 \verb+~+ 14 Tq wide.
pub type SYNC_JUMP_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:5 - Baud Rate Prescaler, determines the frequency dividing ratio.
    #[inline(always)]
    pub fn baud_presc(&self) -> BAUD_PRESC_R {
        BAUD_PRESC_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Synchronization Jump Width (SJW), 1 \verb+~+ 14 Tq wide.
    #[inline(always)]
    pub fn sync_jump_width(&self) -> SYNC_JUMP_WIDTH_R {
        SYNC_JUMP_WIDTH_R::new(((self.bits >> 6) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMING_0")
            .field("baud_presc", &self.baud_presc())
            .field("sync_jump_width", &self.sync_jump_width())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Baud Rate Prescaler, determines the frequency dividing ratio.
    #[inline(always)]
    #[must_use]
    pub fn baud_presc(&mut self) -> BAUD_PRESC_W<BUS_TIMING_0_SPEC> {
        BAUD_PRESC_W::new(self, 0)
    }
    ///Bits 6:7 - Synchronization Jump Width (SJW), 1 \verb+~+ 14 Tq wide.
    #[inline(always)]
    #[must_use]
    pub fn sync_jump_width(&mut self) -> SYNC_JUMP_WIDTH_W<BUS_TIMING_0_SPEC> {
        SYNC_JUMP_WIDTH_W::new(self, 6)
    }
}
/**Bus Timing Register 0

You can [`read`](crate::generic::Reg::read) this register and get [`bus_timing_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timing_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUS_TIMING_0_SPEC;
impl crate::RegisterSpec for BUS_TIMING_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bus_timing_0::R`](R) reader structure
impl crate::Readable for BUS_TIMING_0_SPEC {}
///`write(|w| ..)` method takes [`bus_timing_0::W`](W) writer structure
impl crate::Writable for BUS_TIMING_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BUS_TIMING_0 to value 0
impl crate::Resettable for BUS_TIMING_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
