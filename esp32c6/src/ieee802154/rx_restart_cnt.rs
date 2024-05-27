///Register `RX_RESTART_CNT` reader
pub type R = crate::R<RX_RESTART_CNT_SPEC>;
///Register `RX_RESTART_CNT` writer
pub type W = crate::W<RX_RESTART_CNT_SPEC>;
///Field `RX_RESTART_CNT` reader -
pub type RX_RESTART_CNT_R = crate::FieldReader<u16>;
///Field `RX_RESTART_CNT` writer -
pub type RX_RESTART_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn rx_restart_cnt(&self) -> RX_RESTART_CNT_R {
        RX_RESTART_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_RESTART_CNT")
            .field("rx_restart_cnt", &self.rx_restart_cnt())
            .finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn rx_restart_cnt(&mut self) -> RX_RESTART_CNT_W<RX_RESTART_CNT_SPEC> {
        RX_RESTART_CNT_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`rx_restart_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_restart_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_RESTART_CNT_SPEC;
impl crate::RegisterSpec for RX_RESTART_CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_restart_cnt::R`](R) reader structure
impl crate::Readable for RX_RESTART_CNT_SPEC {}
///`write(|w| ..)` method takes [`rx_restart_cnt::W`](W) writer structure
impl crate::Writable for RX_RESTART_CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_RESTART_CNT to value 0
impl crate::Resettable for RX_RESTART_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
