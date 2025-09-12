#[doc = "Register `IN_CONF1` reader"]
pub type R = crate::R<IN_CONF1_SPEC>;
#[doc = "Register `IN_CONF1` writer"]
pub type W = crate::W<IN_CONF1_SPEC>;
#[doc = "Field `DMA_INFIFO_FULL_THRS` reader - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
pub type DMA_INFIFO_FULL_THRS_R = crate::FieldReader<u16>;
#[doc = "Field `DMA_INFIFO_FULL_THRS` writer - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
pub type DMA_INFIFO_FULL_THRS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `IN_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `IN_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type IN_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_EXT_MEM_BK_SIZE` reader - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type IN_EXT_MEM_BK_SIZE_R = crate::FieldReader;
#[doc = "Field `IN_EXT_MEM_BK_SIZE` writer - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type IN_EXT_MEM_BK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:11 - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
    #[inline(always)]
    pub fn dma_infifo_full_thrs(&self) -> DMA_INFIFO_FULL_THRS_R {
        DMA_INFIFO_FULL_THRS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner(&self) -> IN_CHECK_OWNER_R {
        IN_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn in_ext_mem_bk_size(&self) -> IN_EXT_MEM_BK_SIZE_R {
        IN_EXT_MEM_BK_SIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_CONF1")
            .field("dma_infifo_full_thrs", &self.dma_infifo_full_thrs())
            .field("in_check_owner", &self.in_check_owner())
            .field("in_ext_mem_bk_size", &self.in_ext_mem_bk_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This register is used to generate the INFIFO_FULL_WM_INT interrupt when Rx channel 0 received byte number in Rx FIFO is up to the value of the register."]
    #[inline(always)]
    pub fn dma_infifo_full_thrs(&mut self) -> DMA_INFIFO_FULL_THRS_W<'_, IN_CONF1_SPEC> {
        DMA_INFIFO_FULL_THRS_W::new(self, 0)
    }
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn in_check_owner(&mut self) -> IN_CHECK_OWNER_W<'_, IN_CONF1_SPEC> {
        IN_CHECK_OWNER_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Block size of Rx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn in_ext_mem_bk_size(&mut self) -> IN_EXT_MEM_BK_SIZE_W<'_, IN_CONF1_SPEC> {
        IN_EXT_MEM_BK_SIZE_W::new(self, 13)
    }
}
#[doc = "Configure 1 register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_CONF1_SPEC;
impl crate::RegisterSpec for IN_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_conf1::R`](R) reader structure"]
impl crate::Readable for IN_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_conf1::W`](W) writer structure"]
impl crate::Writable for IN_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_CONF1 to value 0x0c"]
impl crate::Resettable for IN_CONF1_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
