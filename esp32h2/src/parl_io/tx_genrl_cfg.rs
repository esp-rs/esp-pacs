///Register `TX_GENRL_CFG` reader
pub type R = crate::R<TX_GENRL_CFG_SPEC>;
///Register `TX_GENRL_CFG` writer
pub type W = crate::W<TX_GENRL_CFG_SPEC>;
///Field `TX_IDLE_VALUE` reader - Configures bus value of transmitter in IDLE state.
pub type TX_IDLE_VALUE_R = crate::FieldReader<u16>;
///Field `TX_IDLE_VALUE` writer - Configures bus value of transmitter in IDLE state.
pub type TX_IDLE_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `TX_GATING_EN` reader - Set this bit to enable the clock gating of output tx clock.
pub type TX_GATING_EN_R = crate::BitReader;
///Field `TX_GATING_EN` writer - Set this bit to enable the clock gating of output tx clock.
pub type TX_GATING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_VALID_OUTPUT_EN` reader - Set this bit to enable the output of tx data valid signal.
pub type TX_VALID_OUTPUT_EN_R = crate::BitReader;
///Field `TX_VALID_OUTPUT_EN` writer - Set this bit to enable the output of tx data valid signal.
pub type TX_VALID_OUTPUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 14:29 - Configures bus value of transmitter in IDLE state.
    #[inline(always)]
    pub fn tx_idle_value(&self) -> TX_IDLE_VALUE_R {
        TX_IDLE_VALUE_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    ///Bit 30 - Set this bit to enable the clock gating of output tx clock.
    #[inline(always)]
    pub fn tx_gating_en(&self) -> TX_GATING_EN_R {
        TX_GATING_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Set this bit to enable the output of tx data valid signal.
    #[inline(always)]
    pub fn tx_valid_output_en(&self) -> TX_VALID_OUTPUT_EN_R {
        TX_VALID_OUTPUT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_GENRL_CFG")
            .field("tx_idle_value", &self.tx_idle_value())
            .field("tx_gating_en", &self.tx_gating_en())
            .field("tx_valid_output_en", &self.tx_valid_output_en())
            .finish()
    }
}
impl W {
    ///Bits 14:29 - Configures bus value of transmitter in IDLE state.
    #[inline(always)]
    #[must_use]
    pub fn tx_idle_value(&mut self) -> TX_IDLE_VALUE_W<TX_GENRL_CFG_SPEC> {
        TX_IDLE_VALUE_W::new(self, 14)
    }
    ///Bit 30 - Set this bit to enable the clock gating of output tx clock.
    #[inline(always)]
    #[must_use]
    pub fn tx_gating_en(&mut self) -> TX_GATING_EN_W<TX_GENRL_CFG_SPEC> {
        TX_GATING_EN_W::new(self, 30)
    }
    ///Bit 31 - Set this bit to enable the output of tx data valid signal.
    #[inline(always)]
    #[must_use]
    pub fn tx_valid_output_en(&mut self) -> TX_VALID_OUTPUT_EN_W<TX_GENRL_CFG_SPEC> {
        TX_VALID_OUTPUT_EN_W::new(self, 31)
    }
}
/**Parallel TX general configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`tx_genrl_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_genrl_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_GENRL_CFG_SPEC;
impl crate::RegisterSpec for TX_GENRL_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_genrl_cfg::R`](R) reader structure
impl crate::Readable for TX_GENRL_CFG_SPEC {}
///`write(|w| ..)` method takes [`tx_genrl_cfg::W`](W) writer structure
impl crate::Writable for TX_GENRL_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_GENRL_CFG to value 0
impl crate::Resettable for TX_GENRL_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
