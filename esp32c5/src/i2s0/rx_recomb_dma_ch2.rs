#[doc = "Register `RX_RECOMB_DMA_CH2` reader"]
pub type R = crate::R<RX_RECOMB_DMA_CH2_SPEC>;
#[doc = "Register `RX_RECOMB_DMA_CH2` writer"]
pub type W = crate::W<RX_RECOMB_DMA_CH2_SPEC>;
#[doc = "Field `VALID` reader - Set this bit to enable the adc-dma-channel."]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - Set this bit to enable the adc-dma-channel."]
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STYLE` reader - Set this field to set the recombined-dma-channel style. If choose to use i2s extracted ch 1&3 in 4 channels, the style should be: 6'b1010."]
pub type STYLE_R = crate::FieldReader;
#[doc = "Field `STYLE` writer - Set this field to set the recombined-dma-channel style. If choose to use i2s extracted ch 1&3 in 4 channels, the style should be: 6'b1010."]
pub type STYLE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ORDER` reader - Set this field to set the recombined-dma-channel order. If choose to use the order ch3 -> ch1, the order should be: 8'd7 = {2'd0,2'd0,2'd1,2'd3}."]
pub type ORDER_R = crate::FieldReader;
#[doc = "Field `ORDER` writer - Set this field to set the recombined-dma-channel order. If choose to use the order ch3 -> ch1, the order should be: 8'd7 = {2'd0,2'd0,2'd1,2'd3}."]
pub type ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EOF_NUM` reader - Set this field to set the receive eof byte length of the recombined-dma-channel."]
pub type EOF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `EOF_NUM` writer - Set this field to set the receive eof byte length of the recombined-dma-channel."]
pub type EOF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable the adc-dma-channel."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Set this field to set the recombined-dma-channel style. If choose to use i2s extracted ch 1&3 in 4 channels, the style should be: 6'b1010."]
    #[inline(always)]
    pub fn style(&self) -> STYLE_R {
        STYLE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:12 - Set this field to set the recombined-dma-channel order. If choose to use the order ch3 -> ch1, the order should be: 8'd7 = {2'd0,2'd0,2'd1,2'd3}."]
    #[inline(always)]
    pub fn order(&self) -> ORDER_R {
        ORDER_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 13:28 - Set this field to set the receive eof byte length of the recombined-dma-channel."]
    #[inline(always)]
    pub fn eof_num(&self) -> EOF_NUM_R {
        EOF_NUM_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_RECOMB_DMA_CH2")
            .field("valid", &self.valid())
            .field("style", &self.style())
            .field("order", &self.order())
            .field("eof_num", &self.eof_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable the adc-dma-channel."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W<RX_RECOMB_DMA_CH2_SPEC> {
        VALID_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Set this field to set the recombined-dma-channel style. If choose to use i2s extracted ch 1&3 in 4 channels, the style should be: 6'b1010."]
    #[inline(always)]
    pub fn style(&mut self) -> STYLE_W<RX_RECOMB_DMA_CH2_SPEC> {
        STYLE_W::new(self, 1)
    }
    #[doc = "Bits 5:12 - Set this field to set the recombined-dma-channel order. If choose to use the order ch3 -> ch1, the order should be: 8'd7 = {2'd0,2'd0,2'd1,2'd3}."]
    #[inline(always)]
    pub fn order(&mut self) -> ORDER_W<RX_RECOMB_DMA_CH2_SPEC> {
        ORDER_W::new(self, 5)
    }
    #[doc = "Bits 13:28 - Set this field to set the receive eof byte length of the recombined-dma-channel."]
    #[inline(always)]
    pub fn eof_num(&mut self) -> EOF_NUM_W<RX_RECOMB_DMA_CH2_SPEC> {
        EOF_NUM_W::new(self, 13)
    }
}
#[doc = "I2S RX recombined-dma-channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_recomb_dma_ch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_recomb_dma_ch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_RECOMB_DMA_CH2_SPEC;
impl crate::RegisterSpec for RX_RECOMB_DMA_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_recomb_dma_ch2::R`](R) reader structure"]
impl crate::Readable for RX_RECOMB_DMA_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_recomb_dma_ch2::W`](W) writer structure"]
impl crate::Writable for RX_RECOMB_DMA_CH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_RECOMB_DMA_CH2 to value 0"]
impl crate::Resettable for RX_RECOMB_DMA_CH2_SPEC {
    const RESET_VALUE: u32 = 0;
}
