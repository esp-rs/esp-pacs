#[doc = "Register `RX_CNT_CFG` reader"]
pub type R = crate::R<RX_CNT_CFG_SPEC>;
#[doc = "Register `RX_CNT_CFG` writer"]
pub type W = crate::W<RX_CNT_CFG_SPEC>;
#[doc = "Field `TX_CNT_A_CFG_VALUE` reader - config the rx counter a value"]
pub type TX_CNT_A_CFG_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TX_CNT_A_CFG_VALUE` writer - config the rx counter a value"]
pub type TX_CNT_A_CFG_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX_CNT_B_CFG_VALUE` reader - config the rx counter b value"]
pub type TX_CNT_B_CFG_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TX_CNT_B_CFG_VALUE` writer - config the rx counter b value"]
pub type TX_CNT_B_CFG_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - config the rx counter a value"]
    #[inline(always)]
    pub fn tx_cnt_a_cfg_value(&self) -> TX_CNT_A_CFG_VALUE_R {
        TX_CNT_A_CFG_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - config the rx counter b value"]
    #[inline(always)]
    pub fn tx_cnt_b_cfg_value(&self) -> TX_CNT_B_CFG_VALUE_R {
        TX_CNT_B_CFG_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CNT_CFG")
            .field("tx_cnt_a_cfg_value", &self.tx_cnt_a_cfg_value())
            .field("tx_cnt_b_cfg_value", &self.tx_cnt_b_cfg_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - config the rx counter a value"]
    #[inline(always)]
    pub fn tx_cnt_a_cfg_value(&mut self) -> TX_CNT_A_CFG_VALUE_W<'_, RX_CNT_CFG_SPEC> {
        TX_CNT_A_CFG_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - config the rx counter b value"]
    #[inline(always)]
    pub fn tx_cnt_b_cfg_value(&mut self) -> TX_CNT_B_CFG_VALUE_W<'_, RX_CNT_CFG_SPEC> {
        TX_CNT_B_CFG_VALUE_W::new(self, 16)
    }
}
#[doc = "RX Counter cfg registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_cnt_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_cnt_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CNT_CFG_SPEC;
impl crate::RegisterSpec for RX_CNT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_cnt_cfg::R`](R) reader structure"]
impl crate::Readable for RX_CNT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_cnt_cfg::W`](W) writer structure"]
impl crate::Writable for RX_CNT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CNT_CFG to value 0"]
impl crate::Resettable for RX_CNT_CFG_SPEC {}
