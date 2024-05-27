///Register `BLEND_BYTE_ORDER` reader
pub type R = crate::R<BLEND_BYTE_ORDER_SPEC>;
///Register `BLEND_BYTE_ORDER` writer
pub type W = crate::W<BLEND_BYTE_ORDER_SPEC>;
///Field `BLEND0_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
pub type BLEND0_RX_BYTE_SWAP_EN_R = crate::BitReader;
///Field `BLEND0_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
pub type BLEND0_RX_BYTE_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEND1_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
pub type BLEND1_RX_BYTE_SWAP_EN_R = crate::BitReader;
///Field `BLEND1_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
pub type BLEND1_RX_BYTE_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEND0_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
pub type BLEND0_RX_RGB_SWAP_EN_R = crate::BitReader;
///Field `BLEND0_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
pub type BLEND0_RX_RGB_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BLEND1_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
pub type BLEND1_RX_RGB_SWAP_EN_R = crate::BitReader;
///Field `BLEND1_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
pub type BLEND1_RX_RGB_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub fn blend0_rx_byte_swap_en(&self) -> BLEND0_RX_BYTE_SWAP_EN_R {
        BLEND0_RX_BYTE_SWAP_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    pub fn blend1_rx_byte_swap_en(&self) -> BLEND1_RX_BYTE_SWAP_EN_R {
        BLEND1_RX_BYTE_SWAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub fn blend0_rx_rgb_swap_en(&self) -> BLEND0_RX_RGB_SWAP_EN_R {
        BLEND0_RX_RGB_SWAP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    pub fn blend1_rx_rgb_swap_en(&self) -> BLEND1_RX_RGB_SWAP_EN_R {
        BLEND1_RX_RGB_SWAP_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_BYTE_ORDER")
            .field("blend0_rx_byte_swap_en", &self.blend0_rx_byte_swap_en())
            .field("blend1_rx_byte_swap_en", &self.blend1_rx_byte_swap_en())
            .field("blend0_rx_rgb_swap_en", &self.blend0_rx_rgb_swap_en())
            .field("blend1_rx_rgb_swap_en", &self.blend1_rx_rgb_swap_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    #[must_use]
    pub fn blend0_rx_byte_swap_en(&mut self) -> BLEND0_RX_BYTE_SWAP_EN_W<BLEND_BYTE_ORDER_SPEC> {
        BLEND0_RX_BYTE_SWAP_EN_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped.
    #[inline(always)]
    #[must_use]
    pub fn blend1_rx_byte_swap_en(&mut self) -> BLEND1_RX_BYTE_SWAP_EN_W<BLEND_BYTE_ORDER_SPEC> {
        BLEND1_RX_BYTE_SWAP_EN_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    #[must_use]
    pub fn blend0_rx_rgb_swap_en(&mut self) -> BLEND0_RX_RGB_SWAP_EN_W<BLEND_BYTE_ORDER_SPEC> {
        BLEND0_RX_RGB_SWAP_EN_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr.
    #[inline(always)]
    #[must_use]
    pub fn blend1_rx_rgb_swap_en(&mut self) -> BLEND1_RX_RGB_SWAP_EN_W<BLEND_BYTE_ORDER_SPEC> {
        BLEND1_RX_RGB_SWAP_EN_W::new(self, 3)
    }
}
/**Blending engine byte order register

You can [`read`](crate::generic::Reg::read) this register and get [`blend_byte_order::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blend_byte_order::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLEND_BYTE_ORDER_SPEC;
impl crate::RegisterSpec for BLEND_BYTE_ORDER_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blend_byte_order::R`](R) reader structure
impl crate::Readable for BLEND_BYTE_ORDER_SPEC {}
///`write(|w| ..)` method takes [`blend_byte_order::W`](W) writer structure
impl crate::Writable for BLEND_BYTE_ORDER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLEND_BYTE_ORDER to value 0
impl crate::Resettable for BLEND_BYTE_ORDER_SPEC {
    const RESET_VALUE: u32 = 0;
}
