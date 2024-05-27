///Register `RX_START_CFG` reader
pub type R = crate::R<RX_START_CFG_SPEC>;
///Register `RX_START_CFG` writer
pub type W = crate::W<RX_START_CFG_SPEC>;
///Field `RX_START` reader - Set this bit to start rx data sampling.
pub type RX_START_R = crate::BitReader;
///Field `RX_START` writer - Set this bit to start rx data sampling.
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - Set this bit to start rx data sampling.
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_START_CFG")
            .field("rx_start", &self.rx_start())
            .finish()
    }
}
impl W {
    ///Bit 31 - Set this bit to start rx data sampling.
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<RX_START_CFG_SPEC> {
        RX_START_W::new(self, 31)
    }
}
/**Parallel RX Start configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`rx_start_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_start_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_START_CFG_SPEC;
impl crate::RegisterSpec for RX_START_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_start_cfg::R`](R) reader structure
impl crate::Readable for RX_START_CFG_SPEC {}
///`write(|w| ..)` method takes [`rx_start_cfg::W`](W) writer structure
impl crate::Writable for RX_START_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_START_CFG to value 0
impl crate::Resettable for RX_START_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
