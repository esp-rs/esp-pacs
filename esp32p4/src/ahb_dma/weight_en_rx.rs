///Register `WEIGHT_EN_RX` reader
pub type R = crate::R<WEIGHT_EN_RX_SPEC>;
///Register `WEIGHT_EN_RX` writer
pub type W = crate::W<WEIGHT_EN_RX_SPEC>;
///Field `WEIGHT_EN_RX` reader - This register is used to config arbiter weight function off/on
pub type WEIGHT_EN_RX_R = crate::BitReader;
///Field `WEIGHT_EN_RX` writer - This register is used to config arbiter weight function off/on
pub type WEIGHT_EN_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - This register is used to config arbiter weight function off/on
    #[inline(always)]
    pub fn weight_en_rx(&self) -> WEIGHT_EN_RX_R {
        WEIGHT_EN_RX_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WEIGHT_EN_RX")
            .field("weight_en_rx", &self.weight_en_rx())
            .finish()
    }
}
impl W {
    ///Bit 0 - This register is used to config arbiter weight function off/on
    #[inline(always)]
    #[must_use]
    pub fn weight_en_rx(&mut self) -> WEIGHT_EN_RX_W<WEIGHT_EN_RX_SPEC> {
        WEIGHT_EN_RX_W::new(self, 0)
    }
}
/**This register is used to config arbiter weigh function to on or off for rx dir

You can [`read`](crate::generic::Reg::read) this register and get [`weight_en_rx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weight_en_rx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WEIGHT_EN_RX_SPEC;
impl crate::RegisterSpec for WEIGHT_EN_RX_SPEC {
    type Ux = u32;
}
///`read()` method returns [`weight_en_rx::R`](R) reader structure
impl crate::Readable for WEIGHT_EN_RX_SPEC {}
///`write(|w| ..)` method takes [`weight_en_rx::W`](W) writer structure
impl crate::Writable for WEIGHT_EN_RX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WEIGHT_EN_RX to value 0
impl crate::Resettable for WEIGHT_EN_RX_SPEC {
    const RESET_VALUE: u32 = 0;
}
