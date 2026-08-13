#[doc = "Register `TX_CS_CFG` reader"]
pub type R = crate::R<TX_CS_CFG_SPEC>;
#[doc = "Register `TX_CS_CFG` writer"]
pub type W = crate::W<TX_CS_CFG_SPEC>;
#[doc = "Field `TX_CS_STOP_DELAY` reader - configure the delay between data tx end and tx_cs_o posedge"]
pub type TX_CS_STOP_DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `TX_CS_STOP_DELAY` writer - configure the delay between data tx end and tx_cs_o posedge"]
pub type TX_CS_STOP_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX_CS_START_DELAY` reader - configure the delay between tx_cs_o negedge and data tx start"]
pub type TX_CS_START_DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `TX_CS_START_DELAY` writer - configure the delay between tx_cs_o negedge and data tx start"]
pub type TX_CS_START_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - configure the delay between data tx end and tx_cs_o posedge"]
    #[inline(always)]
    pub fn tx_cs_stop_delay(&self) -> TX_CS_STOP_DELAY_R {
        TX_CS_STOP_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure the delay between tx_cs_o negedge and data tx start"]
    #[inline(always)]
    pub fn tx_cs_start_delay(&self) -> TX_CS_START_DELAY_R {
        TX_CS_START_DELAY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CS_CFG")
            .field("tx_cs_stop_delay", &self.tx_cs_stop_delay())
            .field("tx_cs_start_delay", &self.tx_cs_start_delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - configure the delay between data tx end and tx_cs_o posedge"]
    #[inline(always)]
    pub fn tx_cs_stop_delay(&mut self) -> TX_CS_STOP_DELAY_W<'_, TX_CS_CFG_SPEC> {
        TX_CS_STOP_DELAY_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - configure the delay between tx_cs_o negedge and data tx start"]
    #[inline(always)]
    pub fn tx_cs_start_delay(&mut self) -> TX_CS_START_DELAY_W<'_, TX_CS_CFG_SPEC> {
        TX_CS_START_DELAY_W::new(self, 16)
    }
}
#[doc = "Parallel IO tx_cs_o generate configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CS_CFG_SPEC;
impl crate::RegisterSpec for TX_CS_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cs_cfg::R`](R) reader structure"]
impl crate::Readable for TX_CS_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_cs_cfg::W`](W) writer structure"]
impl crate::Writable for TX_CS_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CS_CFG to value 0"]
impl crate::Resettable for TX_CS_CFG_SPEC {}
