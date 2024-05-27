///Register `DATA_12` reader
pub type R = crate::R<DATA_12_SPEC>;
///Register `DATA_12` writer
pub type W = crate::W<DATA_12_SPEC>;
///Field `TX_BYTE_12` reader - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
pub type TX_BYTE_12_R = crate::FieldReader;
///Field `TX_BYTE_12` writer - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
pub type TX_BYTE_12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    #[inline(always)]
    pub fn tx_byte_12(&self) -> TX_BYTE_12_R {
        TX_BYTE_12_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_12")
            .field("tx_byte_12", &self.tx_byte_12())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - In reset mode, reserved with RO. In operation mode, when software initiate write operation, it is tx data register 12 and when software initiate read operation, it is rx data register 12.
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_12(&mut self) -> TX_BYTE_12_W<DATA_12_SPEC> {
        TX_BYTE_12_W::new(self, 0)
    }
}
/**Data register 12.

You can [`read`](crate::generic::Reg::read) this register and get [`data_12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_12_SPEC;
impl crate::RegisterSpec for DATA_12_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data_12::R`](R) reader structure
impl crate::Readable for DATA_12_SPEC {}
///`write(|w| ..)` method takes [`data_12::W`](W) writer structure
impl crate::Writable for DATA_12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA_12 to value 0
impl crate::Resettable for DATA_12_SPEC {
    const RESET_VALUE: u32 = 0;
}
