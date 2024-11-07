#[doc = "Register `RX_INST_CFG1` reader"]
pub type R = crate::R<RX_INST_CFG1_SPEC>;
#[doc = "Register `RX_INST_CFG1` writer"]
pub type W = crate::W<RX_INST_CFG1_SPEC>;
#[doc = "Field `RX_INST` reader - write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG"]
pub type RX_INST_R = crate::FieldReader<u32>;
#[doc = "Field `RX_INST` writer - write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG"]
pub type RX_INST_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG"]
    #[inline(always)]
    pub fn rx_inst(&self) -> RX_INST_R {
        RX_INST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_INST_CFG1")
            .field("rx_inst", &self.rx_inst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - write this bits to update instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG, Read this bits to get instruction which specified by BITSCRAMBLER_RX_INST_CFG0_REG"]
    #[inline(always)]
    pub fn rx_inst(&mut self) -> RX_INST_W<RX_INST_CFG1_SPEC> {
        RX_INST_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_inst_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_inst_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_INST_CFG1_SPEC;
impl crate::RegisterSpec for RX_INST_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_inst_cfg1::R`](R) reader structure"]
impl crate::Readable for RX_INST_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_inst_cfg1::W`](W) writer structure"]
impl crate::Writable for RX_INST_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_INST_CFG1 to value 0x0c"]
impl crate::Resettable for RX_INST_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
