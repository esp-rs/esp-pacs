#[doc = "Register `RX_INST_CFG0` reader"]
pub type R = crate::R<RX_INST_CFG0_SPEC>;
#[doc = "Register `RX_INST_CFG0` writer"]
pub type W = crate::W<RX_INST_CFG0_SPEC>;
#[doc = "Field `RX_INST_IDX` reader - write this bits to specify the one of 8 instruction"]
pub type RX_INST_IDX_R = crate::FieldReader;
#[doc = "Field `RX_INST_IDX` writer - write this bits to specify the one of 8 instruction"]
pub type RX_INST_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_INST_POS` reader - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type RX_INST_POS_R = crate::FieldReader;
#[doc = "Field `RX_INST_POS` writer - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
pub type RX_INST_POS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    pub fn rx_inst_idx(&self) -> RX_INST_IDX_R {
        RX_INST_IDX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    pub fn rx_inst_pos(&self) -> RX_INST_POS_R {
        RX_INST_POS_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_INST_CFG0")
            .field("rx_inst_idx", &self.rx_inst_idx())
            .field("rx_inst_pos", &self.rx_inst_pos())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - write this bits to specify the one of 8 instruction"]
    #[inline(always)]
    #[must_use]
    pub fn rx_inst_idx(&mut self) -> RX_INST_IDX_W<RX_INST_CFG0_SPEC> {
        RX_INST_IDX_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - write this bits to specify the bit position of 257 bit instruction which in units of 32 bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx_inst_pos(&mut self) -> RX_INST_POS_W<RX_INST_CFG0_SPEC> {
        RX_INST_POS_W::new(self, 3)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_inst_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_inst_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_INST_CFG0_SPEC;
impl crate::RegisterSpec for RX_INST_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_inst_cfg0::R`](R) reader structure"]
impl crate::Readable for RX_INST_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_inst_cfg0::W`](W) writer structure"]
impl crate::Writable for RX_INST_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_INST_CFG0 to value 0"]
impl crate::Resettable for RX_INST_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
