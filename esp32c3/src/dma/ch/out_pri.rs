///Register `OUT_PRI` reader
pub type R = crate::R<OUT_PRI_SPEC>;
///Register `OUT_PRI` writer
pub type W = crate::W<OUT_PRI_SPEC>;
///Field `TX_PRI` reader - The priority of Tx channel 0. The larger of the value, the higher of the priority.
pub type TX_PRI_R = crate::FieldReader;
///Field `TX_PRI` writer - The priority of Tx channel 0. The larger of the value, the higher of the priority.
pub type TX_PRI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority.
    #[inline(always)]
    pub fn tx_pri(&self) -> TX_PRI_R {
        TX_PRI_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PRI")
            .field("tx_pri", &self.tx_pri())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - The priority of Tx channel 0. The larger of the value, the higher of the priority.
    #[inline(always)]
    #[must_use]
    pub fn tx_pri(&mut self) -> TX_PRI_W<OUT_PRI_SPEC> {
        TX_PRI_W::new(self, 0)
    }
}
/**DMA_OUT_PRI_CH0_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`out_pri::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_pri::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_PRI_SPEC;
impl crate::RegisterSpec for OUT_PRI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_pri::R`](R) reader structure
impl crate::Readable for OUT_PRI_SPEC {}
///`write(|w| ..)` method takes [`out_pri::W`](W) writer structure
impl crate::Writable for OUT_PRI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT_PRI to value 0
impl crate::Resettable for OUT_PRI_SPEC {
    const RESET_VALUE: u32 = 0;
}
