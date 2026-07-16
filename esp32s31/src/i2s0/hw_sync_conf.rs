#[doc = "Register `HW_SYNC_CONF` reader"]
pub type R = crate::R<HW_SYNC_CONF_SPEC>;
#[doc = "Register `HW_SYNC_CONF` writer"]
pub type W = crate::W<HW_SYNC_CONF_SPEC>;
#[doc = "Field `TX_HW_SYNC_EN` reader - Configure whether enable i2s tx hardware sync function. 1: Enable. 0: Disable"]
pub type TX_HW_SYNC_EN_R = crate::BitReader;
#[doc = "Field `TX_HW_SYNC_EN` writer - Configure whether enable i2s tx hardware sync function. 1: Enable. 0: Disable"]
pub type TX_HW_SYNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HW_SYNC_SUPPL_MODE` reader - Configure the i2s tx hardware sync supplementation mode. 1: Supplement the data configured in I2S_TX_HW_SYNC_SUPPL_DATA\\[31:0\\]. 0: Supplement the last data."]
pub type TX_HW_SYNC_SUPPL_MODE_R = crate::BitReader;
#[doc = "Field `TX_HW_SYNC_SUPPL_MODE` writer - Configure the i2s tx hardware sync supplementation mode. 1: Supplement the data configured in I2S_TX_HW_SYNC_SUPPL_DATA\\[31:0\\]. 0: Supplement the last data."]
pub type TX_HW_SYNC_SUPPL_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether enable i2s tx hardware sync function. 1: Enable. 0: Disable"]
    #[inline(always)]
    pub fn tx_hw_sync_en(&self) -> TX_HW_SYNC_EN_R {
        TX_HW_SYNC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure the i2s tx hardware sync supplementation mode. 1: Supplement the data configured in I2S_TX_HW_SYNC_SUPPL_DATA\\[31:0\\]. 0: Supplement the last data."]
    #[inline(always)]
    pub fn tx_hw_sync_suppl_mode(&self) -> TX_HW_SYNC_SUPPL_MODE_R {
        TX_HW_SYNC_SUPPL_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_SYNC_CONF")
            .field("tx_hw_sync_en", &self.tx_hw_sync_en())
            .field("tx_hw_sync_suppl_mode", &self.tx_hw_sync_suppl_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether enable i2s tx hardware sync function. 1: Enable. 0: Disable"]
    #[inline(always)]
    pub fn tx_hw_sync_en(&mut self) -> TX_HW_SYNC_EN_W<'_, HW_SYNC_CONF_SPEC> {
        TX_HW_SYNC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure the i2s tx hardware sync supplementation mode. 1: Supplement the data configured in I2S_TX_HW_SYNC_SUPPL_DATA\\[31:0\\]. 0: Supplement the last data."]
    #[inline(always)]
    pub fn tx_hw_sync_suppl_mode(&mut self) -> TX_HW_SYNC_SUPPL_MODE_W<'_, HW_SYNC_CONF_SPEC> {
        TX_HW_SYNC_SUPPL_MODE_W::new(self, 1)
    }
}
#[doc = "I2S TX hardware sync function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_sync_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hw_sync_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_SYNC_CONF_SPEC;
impl crate::RegisterSpec for HW_SYNC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_sync_conf::R`](R) reader structure"]
impl crate::Readable for HW_SYNC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_sync_conf::W`](W) writer structure"]
impl crate::Writable for HW_SYNC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HW_SYNC_CONF to value 0"]
impl crate::Resettable for HW_SYNC_CONF_SPEC {}
