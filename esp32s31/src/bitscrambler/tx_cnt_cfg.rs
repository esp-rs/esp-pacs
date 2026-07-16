#[doc = "Register `TX_CNT_CFG` reader"]
pub type R = crate::R<TX_CNT_CFG_SPEC>;
#[doc = "Register `TX_CNT_CFG` writer"]
pub type W = crate::W<TX_CNT_CFG_SPEC>;
#[doc = "Field `RX_CNT_A_CFG_VALUE` reader - config the tx counter a value"]
pub type RX_CNT_A_CFG_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `RX_CNT_A_CFG_VALUE` writer - config the tx counter a value"]
pub type RX_CNT_A_CFG_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_CNT_B_CFG_VALUE` reader - config the tx counter b value"]
pub type RX_CNT_B_CFG_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `RX_CNT_B_CFG_VALUE` writer - config the tx counter b value"]
pub type RX_CNT_B_CFG_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - config the tx counter a value"]
    #[inline(always)]
    pub fn rx_cnt_a_cfg_value(&self) -> RX_CNT_A_CFG_VALUE_R {
        RX_CNT_A_CFG_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - config the tx counter b value"]
    #[inline(always)]
    pub fn rx_cnt_b_cfg_value(&self) -> RX_CNT_B_CFG_VALUE_R {
        RX_CNT_B_CFG_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CNT_CFG")
            .field("rx_cnt_a_cfg_value", &self.rx_cnt_a_cfg_value())
            .field("rx_cnt_b_cfg_value", &self.rx_cnt_b_cfg_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - config the tx counter a value"]
    #[inline(always)]
    pub fn rx_cnt_a_cfg_value(&mut self) -> RX_CNT_A_CFG_VALUE_W<'_, TX_CNT_CFG_SPEC> {
        RX_CNT_A_CFG_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - config the tx counter b value"]
    #[inline(always)]
    pub fn rx_cnt_b_cfg_value(&mut self) -> RX_CNT_B_CFG_VALUE_W<'_, TX_CNT_CFG_SPEC> {
        RX_CNT_B_CFG_VALUE_W::new(self, 16)
    }
}
#[doc = "TX Counter cfg registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cnt_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cnt_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CNT_CFG_SPEC;
impl crate::RegisterSpec for TX_CNT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cnt_cfg::R`](R) reader structure"]
impl crate::Readable for TX_CNT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_cnt_cfg::W`](W) writer structure"]
impl crate::Writable for TX_CNT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CNT_CFG to value 0"]
impl crate::Resettable for TX_CNT_CFG_SPEC {}
