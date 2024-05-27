///Register `TX_LUT_CFG1` reader
pub type R = crate::R<TX_LUT_CFG1_SPEC>;
///Register `TX_LUT_CFG1` writer
pub type W = crate::W<TX_LUT_CFG1_SPEC>;
///Field `TX_LUT` reader - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
pub type TX_LUT_R = crate::FieldReader<u32>;
///Field `TX_LUT` writer - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
pub type TX_LUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    #[inline(always)]
    pub fn tx_lut(&self) -> TX_LUT_R {
        TX_LUT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_LUT_CFG1")
            .field("tx_lut", &self.tx_lut())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - write this bits to update LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG, Read this bits to get LUT which specified by BITSCRAMBLER_TX_LUT_CFG0_REG
    #[inline(always)]
    #[must_use]
    pub fn tx_lut(&mut self) -> TX_LUT_W<TX_LUT_CFG1_SPEC> {
        TX_LUT_W::new(self, 0)
    }
}
/**Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_lut_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_lut_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_LUT_CFG1_SPEC;
impl crate::RegisterSpec for TX_LUT_CFG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_lut_cfg1::R`](R) reader structure
impl crate::Readable for TX_LUT_CFG1_SPEC {}
///`write(|w| ..)` method takes [`tx_lut_cfg1::W`](W) writer structure
impl crate::Writable for TX_LUT_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_LUT_CFG1 to value 0x14
impl crate::Resettable for TX_LUT_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
