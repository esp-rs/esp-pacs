#[doc = "Register `IDLE_CONF_SYNC` reader"]
pub type R = crate::R<IDLE_CONF_SYNC_SPEC>;
#[doc = "Register `IDLE_CONF_SYNC` writer"]
pub type W = crate::W<IDLE_CONF_SYNC_SPEC>;
#[doc = "Field `RX_IDLE_THRHD` reader - Configures the threshold to generate a frame end signal when the receiver takes more time to receive one data byte data.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
pub type RX_IDLE_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_IDLE_THRHD` writer - Configures the threshold to generate a frame end signal when the receiver takes more time to receive one data byte data.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
pub type RX_IDLE_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_IDLE_NUM` reader - Configures the interval between two data transfers.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
pub type TX_IDLE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_IDLE_NUM` writer - Configures the interval between two data transfers.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
pub type TX_IDLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Configures the threshold to generate a frame end signal when the receiver takes more time to receive one data byte data.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RX_IDLE_THRHD_R {
        RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Configures the interval between two data transfers.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TX_IDLE_NUM_R {
        TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_CONF_SYNC")
            .field("rx_idle_thrhd", &self.rx_idle_thrhd())
            .field("tx_idle_num", &self.tx_idle_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Configures the threshold to generate a frame end signal when the receiver takes more time to receive one data byte data.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&mut self) -> RX_IDLE_THRHD_W<'_, IDLE_CONF_SYNC_SPEC> {
        RX_IDLE_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Configures the interval between two data transfers.\\\\Measurement unit: bit time (the time to transmit 1 bit)."]
    #[inline(always)]
    pub fn tx_idle_num(&mut self) -> TX_IDLE_NUM_W<'_, IDLE_CONF_SYNC_SPEC> {
        TX_IDLE_NUM_W::new(self, 10)
    }
}
#[doc = "Frame end idle time configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_conf_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_conf_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLE_CONF_SYNC_SPEC;
impl crate::RegisterSpec for IDLE_CONF_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_conf_sync::R`](R) reader structure"]
impl crate::Readable for IDLE_CONF_SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idle_conf_sync::W`](W) writer structure"]
impl crate::Writable for IDLE_CONF_SYNC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDLE_CONF_SYNC to value 0x0004_0100"]
impl crate::Resettable for IDLE_CONF_SYNC_SPEC {
    const RESET_VALUE: u32 = 0x0004_0100;
}
