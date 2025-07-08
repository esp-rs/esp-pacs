#[doc = "Register `RX_LUT_CFG0` reader"]
pub type R = crate::R<RX_LUT_CFG0_SPEC>;
#[doc = "Register `RX_LUT_CFG0` writer"]
pub type W = crate::W<RX_LUT_CFG0_SPEC>;
#[doc = "Field `RX_LUT_IDX` reader - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
pub type RX_LUT_IDX_R = crate::FieldReader<u16>;
#[doc = "Field `RX_LUT_IDX` writer - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
pub type RX_LUT_IDX_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RX_LUT_MODE` reader - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
pub type RX_LUT_MODE_R = crate::FieldReader;
#[doc = "Field `RX_LUT_MODE` writer - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
pub type RX_LUT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:10 - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
    #[inline(always)]
    pub fn rx_lut_idx(&self) -> RX_LUT_IDX_R {
        RX_LUT_IDX_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
    #[inline(always)]
    pub fn rx_lut_mode(&self) -> RX_LUT_MODE_R {
        RX_LUT_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_LUT_CFG0")
            .field("rx_lut_idx", &self.rx_lut_idx())
            .field("rx_lut_mode", &self.rx_lut_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - write this bits to specify the bytes position of LUT RAM based on reg_bitscrambler_rx_lut_mode"]
    #[inline(always)]
    pub fn rx_lut_idx(&mut self) -> RX_LUT_IDX_W<RX_LUT_CFG0_SPEC> {
        RX_LUT_IDX_W::new(self, 0)
    }
    #[doc = "Bits 11:12 - write this bits to specify the bytes mode of LUT RAM, 0: 1 byte,1: 2bytes, 2: 4 bytes"]
    #[inline(always)]
    pub fn rx_lut_mode(&mut self) -> RX_LUT_MODE_W<RX_LUT_CFG0_SPEC> {
        RX_LUT_MODE_W::new(self, 11)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_lut_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_lut_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_LUT_CFG0_SPEC;
impl crate::RegisterSpec for RX_LUT_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_lut_cfg0::R`](R) reader structure"]
impl crate::Readable for RX_LUT_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_lut_cfg0::W`](W) writer structure"]
impl crate::Writable for RX_LUT_CFG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_LUT_CFG0 to value 0"]
impl crate::Resettable for RX_LUT_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
