#[doc = "Register `DESTINATION` reader"]
pub type R = crate::R<DESTINATION_SPEC>;
#[doc = "Register `DESTINATION` writer"]
pub type W = crate::W<DESTINATION_SPEC>;
#[doc = "Field `RX_DESTINATION` reader - Set this bit to configure the data destination of i2s rx. 1: ble. 0: gdma."]
pub type RX_DESTINATION_R = crate::BitReader;
#[doc = "Field `RX_DESTINATION` writer - Set this bit to configure the data destination of i2s rx. 1: ble. 0: gdma."]
pub type RX_DESTINATION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DESTINATION` reader - Set this bit to configure the data destination of i2s tx. 1: ble. 0: gdma."]
pub type TX_DESTINATION_R = crate::BitReader;
#[doc = "Field `TX_DESTINATION` writer - Set this bit to configure the data destination of i2s tx. 1: ble. 0: gdma."]
pub type TX_DESTINATION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to configure the data destination of i2s rx. 1: ble. 0: gdma."]
    #[inline(always)]
    pub fn rx_destination(&self) -> RX_DESTINATION_R {
        RX_DESTINATION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to configure the data destination of i2s tx. 1: ble. 0: gdma."]
    #[inline(always)]
    pub fn tx_destination(&self) -> TX_DESTINATION_R {
        TX_DESTINATION_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DESTINATION")
            .field("rx_destination", &self.rx_destination())
            .field("tx_destination", &self.tx_destination())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to configure the data destination of i2s rx. 1: ble. 0: gdma."]
    #[inline(always)]
    pub fn rx_destination(&mut self) -> RX_DESTINATION_W<'_, DESTINATION_SPEC> {
        RX_DESTINATION_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to configure the data destination of i2s tx. 1: ble. 0: gdma."]
    #[inline(always)]
    pub fn tx_destination(&mut self) -> TX_DESTINATION_W<'_, DESTINATION_SPEC> {
        TX_DESTINATION_W::new(self, 1)
    }
}
#[doc = "I2S TX status register\n\nYou can [`read`](crate::Reg::read) this register and get [`destination::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destination::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DESTINATION_SPEC;
impl crate::RegisterSpec for DESTINATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destination::R`](R) reader structure"]
impl crate::Readable for DESTINATION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`destination::W`](W) writer structure"]
impl crate::Writable for DESTINATION_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DESTINATION to value 0"]
impl crate::Resettable for DESTINATION_SPEC {}
