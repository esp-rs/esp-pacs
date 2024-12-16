#[doc = "Register `RX_MEM_CONF` reader"]
pub type R = crate::R<RX_MEM_CONF_SPEC>;
#[doc = "Register `RX_MEM_CONF` writer"]
pub type W = crate::W<RX_MEM_CONF_SPEC>;
#[doc = "Field `RX_MEM_FIFO_CNT` reader - The number of data in the rx mem"]
pub type RX_MEM_FIFO_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `RX_MEM_THRESHOLD` reader - I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold"]
pub type RX_MEM_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `RX_MEM_THRESHOLD` writer - I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold"]
pub type RX_MEM_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:8 - The number of data in the rx mem"]
    #[inline(always)]
    pub fn rx_mem_fifo_cnt(&self) -> RX_MEM_FIFO_CNT_R {
        RX_MEM_FIFO_CNT_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:16 - I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold"]
    #[inline(always)]
    pub fn rx_mem_threshold(&self) -> RX_MEM_THRESHOLD_R {
        RX_MEM_THRESHOLD_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MEM_CONF")
            .field("rx_mem_fifo_cnt", &self.rx_mem_fifo_cnt())
            .field("rx_mem_threshold", &self.rx_mem_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 9:16 - I2S rx mem will trigger an interrupt when the data in the mem is over(not including equal) reg_rx_mem_threshold"]
    #[inline(always)]
    pub fn rx_mem_threshold(&mut self) -> RX_MEM_THRESHOLD_W<RX_MEM_CONF_SPEC> {
        RX_MEM_THRESHOLD_W::new(self, 9)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_mem_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_mem_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_MEM_CONF_SPEC;
impl crate::RegisterSpec for RX_MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_mem_conf::R`](R) reader structure"]
impl crate::Readable for RX_MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_mem_conf::W`](W) writer structure"]
impl crate::Writable for RX_MEM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_MEM_CONF to value 0x7e00"]
impl crate::Resettable for RX_MEM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x7e00;
}
