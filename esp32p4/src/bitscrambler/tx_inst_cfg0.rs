#[doc = "Register `TX_INST_CFG0` reader"]
pub type R = crate::R<TX_INST_CFG0_SPEC>;
#[doc = "Register `TX_INST_CFG0` writer"]
pub type W = crate::W<TX_INST_CFG0_SPEC>;
#[doc = "Field `TX_INST_IDX` reader - write this bits to specify the one of 8 instruction"]
pub type TX_INST_IDX_R = crate::FieldReader;
#[doc = "Field `TX_INST_IDX` writer - write this bits to specify the one of 8 instruction"]
pub type TX_INST_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TX_INST_POS` reader - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type TX_INST_POS_R = crate::FieldReader;
#[doc = "Field `TX_INST_POS` writer - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type TX_INST_POS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn tx_inst_idx(&self) -> TX_INST_IDX_R {
        TX_INST_IDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
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
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn tx_inst_idx(&mut self) -> TX_INST_IDX_W<'_, TX_INST_CFG0_SPEC> {
        TX_INST_IDX_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    pub fn tx_inst_pos(&mut self) -> TX_INST_POS_W<'_, TX_INST_CFG0_SPEC> {
        TX_INST_POS_W::new(self, 3)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_inst_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_inst_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_INST_CFG0_SPEC;
impl crate::RegisterSpec for TX_INST_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_inst_cfg0::R`](R) reader structure"]
impl crate::Readable for TX_INST_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_inst_cfg0::W`](W) writer structure"]
impl crate::Writable for TX_INST_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_INST_CFG0 to value 0"]
impl crate::Resettable for TX_INST_CFG0_SPEC {}
