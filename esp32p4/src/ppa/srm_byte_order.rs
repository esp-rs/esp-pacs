#[doc = "Register `SRM_BYTE_ORDER` reader"]
pub type R = crate::R<SRM_BYTE_ORDER_SPEC>;
#[doc = "Register `SRM_BYTE_ORDER` writer"]
pub type W = crate::W<SRM_BYTE_ORDER_SPEC>;
#[doc = "Field `SRM_RX_BYTE_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type SRM_RX_BYTE_SWAP_EN_R = crate::BitReader;
#[doc = "Field `SRM_RX_BYTE_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
pub type SRM_RX_BYTE_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_RX_RGB_SWAP_EN` reader - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type SRM_RX_RGB_SWAP_EN_R = crate::BitReader;
#[doc = "Field `SRM_RX_RGB_SWAP_EN` writer - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
pub type SRM_RX_RGB_SWAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_MACRO_BK_RO_BYPASS` reader - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
pub type SRM_MACRO_BK_RO_BYPASS_R = crate::BitReader;
#[doc = "Field `SRM_MACRO_BK_RO_BYPASS` writer - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
pub type SRM_MACRO_BK_RO_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_BK_SIZE_SEL` reader - sel srm pix_blk size, 0:32x32, 1:16x16"]
pub type SRM_BK_SIZE_SEL_R = crate::BitReader;
#[doc = "Field `SRM_BK_SIZE_SEL` writer - sel srm pix_blk size, 0:32x32, 1:16x16"]
pub type SRM_BK_SIZE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn srm_rx_byte_swap_en(&self) -> SRM_RX_BYTE_SWAP_EN_R {
        SRM_RX_BYTE_SWAP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn srm_rx_rgb_swap_en(&self) -> SRM_RX_RGB_SWAP_EN_R {
        SRM_RX_RGB_SWAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
    #[inline(always)]
    pub fn srm_macro_bk_ro_bypass(&self) -> SRM_MACRO_BK_RO_BYPASS_R {
        SRM_MACRO_BK_RO_BYPASS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sel srm pix_blk size, 0:32x32, 1:16x16"]
    #[inline(always)]
    pub fn srm_bk_size_sel(&self) -> SRM_BK_SIZE_SEL_R {
        SRM_BK_SIZE_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRM_BYTE_ORDER")
            .field("srm_rx_byte_swap_en", &self.srm_rx_byte_swap_en())
            .field("srm_rx_rgb_swap_en", &self.srm_rx_rgb_swap_en())
            .field("srm_macro_bk_ro_bypass", &self.srm_macro_bk_ro_bypass())
            .field("srm_bk_size_sel", &self.srm_bk_size_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 the data into Rx channel 0 would be swapped in byte. The Byte0 and Byte1 would be swapped while byte 2 and byte 3 would be swappped."]
    #[inline(always)]
    pub fn srm_rx_byte_swap_en(&mut self) -> SRM_RX_BYTE_SWAP_EN_W<'_, SRM_BYTE_ORDER_SPEC> {
        SRM_RX_BYTE_SWAP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 the data into Rx channel 0 would be swapped in rgb. It means rgb would be swap to bgr."]
    #[inline(always)]
    pub fn srm_rx_rgb_swap_en(&mut self) -> SRM_RX_RGB_SWAP_EN_W<'_, SRM_BYTE_ORDER_SPEC> {
        SRM_RX_RGB_SWAP_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to 1 to bypass the macro block order function. This function is used to improve efficient accessing external memory."]
    #[inline(always)]
    pub fn srm_macro_bk_ro_bypass(&mut self) -> SRM_MACRO_BK_RO_BYPASS_W<'_, SRM_BYTE_ORDER_SPEC> {
        SRM_MACRO_BK_RO_BYPASS_W::new(self, 2)
    }
    #[doc = "Bit 3 - sel srm pix_blk size, 0:32x32, 1:16x16"]
    #[inline(always)]
    pub fn srm_bk_size_sel(&mut self) -> SRM_BK_SIZE_SEL_W<'_, SRM_BYTE_ORDER_SPEC> {
        SRM_BK_SIZE_SEL_W::new(self, 3)
    }
}
#[doc = "Scaling and rotating engine byte order register\n\nYou can [`read`](crate::Reg::read) this register and get [`srm_byte_order::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srm_byte_order::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRM_BYTE_ORDER_SPEC;
impl crate::RegisterSpec for SRM_BYTE_ORDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srm_byte_order::R`](R) reader structure"]
impl crate::Readable for SRM_BYTE_ORDER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srm_byte_order::W`](W) writer structure"]
impl crate::Writable for SRM_BYTE_ORDER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRM_BYTE_ORDER to value 0"]
impl crate::Resettable for SRM_BYTE_ORDER_SPEC {}
