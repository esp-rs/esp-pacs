#[doc = "Register `HW_SYNC_DATA` reader"]
pub type R = crate::R<HW_SYNC_DATA_SPEC>;
#[doc = "Register `HW_SYNC_DATA` writer"]
pub type W = crate::W<HW_SYNC_DATA_SPEC>;
#[doc = "Field `TX_HW_SYNC_SUPPL_DATA` reader - Configure the i2s tx hardware sync supplementation data when I2S_TX_HW_SYNC_SUPPL_MODE is 1."]
pub type TX_HW_SYNC_SUPPL_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TX_HW_SYNC_SUPPL_DATA` writer - Configure the i2s tx hardware sync supplementation data when I2S_TX_HW_SYNC_SUPPL_MODE is 1."]
pub type TX_HW_SYNC_SUPPL_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure the i2s tx hardware sync supplementation data when I2S_TX_HW_SYNC_SUPPL_MODE is 1."]
    #[inline(always)]
    pub fn tx_hw_sync_suppl_data(&self) -> TX_HW_SYNC_SUPPL_DATA_R {
        TX_HW_SYNC_SUPPL_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_SYNC_DATA")
            .field("tx_hw_sync_suppl_data", &self.tx_hw_sync_suppl_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure the i2s tx hardware sync supplementation data when I2S_TX_HW_SYNC_SUPPL_MODE is 1."]
    #[inline(always)]
    pub fn tx_hw_sync_suppl_data(&mut self) -> TX_HW_SYNC_SUPPL_DATA_W<'_, HW_SYNC_DATA_SPEC> {
        TX_HW_SYNC_SUPPL_DATA_W::new(self, 0)
    }
}
#[doc = "I2S TX hardware sync function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_sync_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_sync_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_SYNC_DATA_SPEC;
impl crate::RegisterSpec for HW_SYNC_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_sync_data::R`](R) reader structure"]
impl crate::Readable for HW_SYNC_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_sync_data::W`](W) writer structure"]
impl crate::Writable for HW_SYNC_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_SYNC_DATA to value 0"]
impl crate::Resettable for HW_SYNC_DATA_SPEC {}
