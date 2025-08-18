#[doc = "Register `IDLE_CONF` reader"]
pub type R = crate::R<IDLE_CONF_SPEC>;
#[doc = "Register `IDLE_CONF` writer"]
pub type W = crate::W<IDLE_CONF_SPEC>;
#[doc = "Field `RX_IDLE_THRHD` reader - when receiver takes more time than this register value to receive a byte data. it will produce frame end signal for uhci to stop receiving data."]
pub type RX_IDLE_THRHD_R = crate::FieldReader<u16>;
#[doc = "Field `RX_IDLE_THRHD` writer - when receiver takes more time than this register value to receive a byte data. it will produce frame end signal for uhci to stop receiving data."]
pub type RX_IDLE_THRHD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_IDLE_NUM` reader - This register is used to configure the duration time between transfers."]
pub type TX_IDLE_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_IDLE_NUM` writer - This register is used to configure the duration time between transfers."]
pub type TX_IDLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TX_BRK_NUM` reader - This register is used to configure the num of 0 send after the process of sending data is done. it is active when txd_brk is set to 1."]
pub type TX_BRK_NUM_R = crate::FieldReader;
#[doc = "Field `TX_BRK_NUM` writer - This register is used to configure the num of 0 send after the process of sending data is done. it is active when txd_brk is set to 1."]
pub type TX_BRK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:9 - when receiver takes more time than this register value to receive a byte data. it will produce frame end signal for uhci to stop receiving data."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&self) -> RX_IDLE_THRHD_R {
        RX_IDLE_THRHD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    pub fn tx_idle_num(&self) -> TX_IDLE_NUM_R {
        TX_IDLE_NUM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:27 - This register is used to configure the num of 0 send after the process of sending data is done. it is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&self) -> TX_BRK_NUM_R {
        TX_BRK_NUM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE_CONF")
            .field("rx_idle_thrhd", &self.rx_idle_thrhd())
            .field("tx_idle_num", &self.tx_idle_num())
            .field("tx_brk_num", &self.tx_brk_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - when receiver takes more time than this register value to receive a byte data. it will produce frame end signal for uhci to stop receiving data."]
    #[inline(always)]
    pub fn rx_idle_thrhd(&mut self) -> RX_IDLE_THRHD_W<'_, IDLE_CONF_SPEC> {
        RX_IDLE_THRHD_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - This register is used to configure the duration time between transfers."]
    #[inline(always)]
    pub fn tx_idle_num(&mut self) -> TX_IDLE_NUM_W<'_, IDLE_CONF_SPEC> {
        TX_IDLE_NUM_W::new(self, 10)
    }
    #[doc = "Bits 20:27 - This register is used to configure the num of 0 send after the process of sending data is done. it is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&mut self) -> TX_BRK_NUM_W<'_, IDLE_CONF_SPEC> {
        TX_BRK_NUM_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`idle_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idle_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLE_CONF_SPEC;
impl crate::RegisterSpec for IDLE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_conf::R`](R) reader structure"]
impl crate::Readable for IDLE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idle_conf::W`](W) writer structure"]
impl crate::Writable for IDLE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDLE_CONF to value 0x00a4_0100"]
impl crate::Resettable for IDLE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x00a4_0100;
}
