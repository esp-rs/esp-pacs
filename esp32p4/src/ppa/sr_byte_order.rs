#[doc = "Register `SR_BYTE_ORDER` reader"]
pub type R = crate::R<SR_BYTE_ORDER_SPEC>;
#[doc = "Register `SR_BYTE_ORDER` writer"]
pub type W = crate::W<SR_BYTE_ORDER_SPEC>;
#[doc = "Field `SR_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type SR_RX_BYTE_SWAP_EN_R = crate::BitReader;
#[doc = "Field `SR_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type SR_RX_BYTE_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type SR_RX_RGB_SWAP_EN_R = crate::BitReader;
#[doc = "Field `SR_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type SR_RX_RGB_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_MACRO_BK_RO_BYPASS` reader - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
pub type SR_MACRO_BK_RO_BYPASS_R = crate::BitReader;
#[doc = "Field `SR_MACRO_BK_RO_BYPASS` writer - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
pub type SR_MACRO_BK_RO_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn sr_rx_byte_swap_en(&self) -> SR_RX_BYTE_SWAP_EN_R {
        SR_RX_BYTE_SWAP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn sr_rx_rgb_swap_en(&self) -> SR_RX_RGB_SWAP_EN_R {
        SR_RX_RGB_SWAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
    #[inline(always)]
    pub fn sr_macro_bk_ro_bypass(&self) -> SR_MACRO_BK_RO_BYPASS_R {
        SR_MACRO_BK_RO_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR_BYTE_ORDER")
            .field(
                "sr_rx_byte_swap_en",
                &format_args!("{}", self.sr_rx_byte_swap_en().bit()),
            )
            .field(
                "sr_rx_rgb_swap_en",
                &format_args!("{}", self.sr_rx_rgb_swap_en().bit()),
            )
            .field(
                "sr_macro_bk_ro_bypass",
                &format_args!("{}", self.sr_macro_bk_ro_bypass().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_BYTE_ORDER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    #[must_use]
    pub fn sr_rx_byte_swap_en(&mut self) -> SR_RX_BYTE_SWAP_EN_W<SR_BYTE_ORDER_SPEC> {
        SR_RX_BYTE_SWAP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    #[must_use]
    pub fn sr_rx_rgb_swap_en(&mut self) -> SR_RX_RGB_SWAP_EN_W<SR_BYTE_ORDER_SPEC> {
        SR_RX_RGB_SWAP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
    #[inline(always)]
    #[must_use]
    pub fn sr_macro_bk_ro_bypass(&mut self) -> SR_MACRO_BK_RO_BYPASS_W<SR_BYTE_ORDER_SPEC> {
        SR_MACRO_BK_RO_BYPASS_W::new(self, 2)
    }
}
#[doc = "Scaling and rotating engine byte order register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr_byte_order::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr_byte_order::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_BYTE_ORDER_SPEC;
impl crate::RegisterSpec for SR_BYTE_ORDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr_byte_order::R`](R) reader structure"]
impl crate::Readable for SR_BYTE_ORDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr_byte_order::W`](W) writer structure"]
impl crate::Writable for SR_BYTE_ORDER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR_BYTE_ORDER to value 0"]
impl crate::Resettable for SR_BYTE_ORDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
