#[doc = "Register `RAW_BUF_CREDIT_CTL` reader"]
pub type R = crate::R<RAW_BUF_CREDIT_CTL_SPEC>;
#[doc = "Register `RAW_BUF_CREDIT_CTL` writer"]
pub type W = crate::W<RAW_BUF_CREDIT_CTL_SPEC>;
#[doc = "Field `CREDIT_THRD` reader - this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller"]
pub type CREDIT_THRD_R = crate::FieldReader<u16>;
#[doc = "Field `CREDIT_THRD` writer - this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller"]
pub type CREDIT_THRD_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `CREDIT_BURST_THRD` reader - this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller"]
pub type CREDIT_BURST_THRD_R = crate::FieldReader<u16>;
#[doc = "Field `CREDIT_BURST_THRD` writer - this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller"]
pub type CREDIT_BURST_THRD_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `CREDIT_RESET` reader - this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller"]
pub type CREDIT_RESET_R = crate::BitReader;
#[doc = "Field `CREDIT_RESET` writer - this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller"]
pub type CREDIT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn credit_thrd(&self) -> CREDIT_THRD_R {
        CREDIT_THRD_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn credit_burst_thrd(&self) -> CREDIT_BURST_THRD_R {
        CREDIT_BURST_THRD_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn credit_reset(&self) -> CREDIT_RESET_R {
        CREDIT_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW_BUF_CREDIT_CTL")
            .field("credit_thrd", &self.credit_thrd())
            .field("credit_burst_thrd", &self.credit_burst_thrd())
            .field("credit_reset", &self.credit_reset())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - this field configures the threshold whether dsi_bridge fifo can receive one more 64-bit, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn credit_thrd(&mut self) -> CREDIT_THRD_W<RAW_BUF_CREDIT_CTL_SPEC> {
        CREDIT_THRD_W::new(self, 0)
    }
    #[doc = "Bits 16:30 - this field configures the threshold whether dsi_bridge fifo can receive one more dma burst, valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn credit_burst_thrd(&mut self) -> CREDIT_BURST_THRD_W<RAW_BUF_CREDIT_CTL_SPEC> {
        CREDIT_BURST_THRD_W::new(self, 16)
    }
    #[doc = "Bit 31 - this bit configures internal credit cnt clear, 0: non, 1: reset. valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    pub fn credit_reset(&mut self) -> CREDIT_RESET_W<RAW_BUF_CREDIT_CTL_SPEC> {
        CREDIT_RESET_W::new(self, 31)
    }
}
#[doc = "dsi bridge credit register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_buf_credit_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_buf_credit_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_BUF_CREDIT_CTL_SPEC;
impl crate::RegisterSpec for RAW_BUF_CREDIT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_buf_credit_ctl::R`](R) reader structure"]
impl crate::Readable for RAW_BUF_CREDIT_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw_buf_credit_ctl::W`](W) writer structure"]
impl crate::Writable for RAW_BUF_CREDIT_CTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAW_BUF_CREDIT_CTL to value 0x0320_0400"]
impl crate::Resettable for RAW_BUF_CREDIT_CTL_SPEC {
    const RESET_VALUE: u32 = 0x0320_0400;
}
