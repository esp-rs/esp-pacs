///Register `TX_INST_CFG0` reader
pub type R = crate::R<TX_INST_CFG0_SPEC>;
///Register `TX_INST_CFG0` writer
pub type W = crate::W<TX_INST_CFG0_SPEC>;
///Field `TX_INST_IDX` reader - write this bits to specify the one of 8 instruction
pub type TX_INST_IDX_R = crate::FieldReader;
///Field `TX_INST_IDX` writer - write this bits to specify the one of 8 instruction
pub type TX_INST_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TX_INST_POS` reader - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
pub type TX_INST_POS_R = crate::FieldReader;
///Field `TX_INST_POS` writer - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
pub type TX_INST_POS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:2 - write this bits to specify the one of 8 instruction
    #[inline(always)]
    pub fn tx_inst_idx(&self) -> TX_INST_IDX_R {
        TX_INST_IDX_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    pub fn tx_inst_pos(&self) -> TX_INST_POS_R {
        TX_INST_POS_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_INST_CFG0")
            .field("tx_inst_idx", &self.tx_inst_idx())
            .field("tx_inst_pos", &self.tx_inst_pos())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - write this bits to specify the one of 8 instruction
    #[inline(always)]
    #[must_use]
    pub fn tx_inst_idx(&mut self) -> TX_INST_IDX_W<TX_INST_CFG0_SPEC> {
        TX_INST_IDX_W::new(self, 0)
    }
    ///Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits
    #[inline(always)]
    #[must_use]
    pub fn tx_inst_pos(&mut self) -> TX_INST_POS_W<TX_INST_CFG0_SPEC> {
        TX_INST_POS_W::new(self, 3)
    }
}
/**Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`tx_inst_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_inst_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_INST_CFG0_SPEC;
impl crate::RegisterSpec for TX_INST_CFG0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_inst_cfg0::R`](R) reader structure
impl crate::Readable for TX_INST_CFG0_SPEC {}
///`write(|w| ..)` method takes [`tx_inst_cfg0::W`](W) writer structure
impl crate::Writable for TX_INST_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_INST_CFG0 to value 0
impl crate::Resettable for TX_INST_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
