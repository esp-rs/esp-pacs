///Register `DATA_3` reader
pub type R = crate::R<DATA_3_SPEC>;
///Register `DATA_3` writer
pub type W = crate::W<DATA_3_SPEC>;
///Field `TX_BYTE_3` reader - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode.
pub type TX_BYTE_3_R = crate::FieldReader;
///Field `TX_BYTE_3` writer - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode.
pub type TX_BYTE_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode.
    #[inline(always)]
    pub fn tx_byte_3(&self) -> TX_BYTE_3_R {
        TX_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_3")
            .field("tx_byte_3", &self.tx_byte_3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - In reset mode, it is acceptance code register 3 with R/W Permission. In operation mode, it stores the 3rd byte information of the data to be transmitted under operating mode.
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_3(&mut self) -> TX_BYTE_3_W<DATA_3_SPEC> {
        TX_BYTE_3_W::new(self, 0)
    }
}
/**Data register 3

You can [`read`](crate::generic::Reg::read) this register and get [`data_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_3_SPEC;
impl crate::RegisterSpec for DATA_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data_3::R`](R) reader structure
impl crate::Readable for DATA_3_SPEC {}
///`write(|w| ..)` method takes [`data_3::W`](W) writer structure
impl crate::Writable for DATA_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA_3 to value 0
impl crate::Resettable for DATA_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
