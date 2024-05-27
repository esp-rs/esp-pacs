///Register `TXEN_STOP_DELAY` reader
pub type R = crate::R<TXEN_STOP_DELAY_SPEC>;
///Register `TXEN_STOP_DELAY` writer
pub type W = crate::W<TXEN_STOP_DELAY_SPEC>;
///Field `TXEN_STOP_DLY` reader -
pub type TXEN_STOP_DLY_R = crate::FieldReader;
///Field `TXEN_STOP_DLY` writer -
pub type TXEN_STOP_DLY_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5
    #[inline(always)]
    pub fn txen_stop_dly(&self) -> TXEN_STOP_DLY_R {
        TXEN_STOP_DLY_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEN_STOP_DELAY")
            .field("txen_stop_dly", &self.txen_stop_dly())
            .finish()
    }
}
impl W {
    ///Bits 0:5
    #[inline(always)]
    #[must_use]
    pub fn txen_stop_dly(&mut self) -> TXEN_STOP_DLY_W<TXEN_STOP_DELAY_SPEC> {
        TXEN_STOP_DLY_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`txen_stop_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txen_stop_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXEN_STOP_DELAY_SPEC;
impl crate::RegisterSpec for TXEN_STOP_DELAY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`txen_stop_delay::R`](R) reader structure
impl crate::Readable for TXEN_STOP_DELAY_SPEC {}
///`write(|w| ..)` method takes [`txen_stop_delay::W`](W) writer structure
impl crate::Writable for TXEN_STOP_DELAY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXEN_STOP_DELAY to value 0
impl crate::Resettable for TXEN_STOP_DELAY_SPEC {
    const RESET_VALUE: u32 = 0;
}
