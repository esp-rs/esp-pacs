#[doc = "Register `RAW_BUF_ALMOST_EMPTY_THRD` reader"]
pub type R = crate::R<RAW_BUF_ALMOST_EMPTY_THRD_SPEC>;
#[doc = "Register `RAW_BUF_ALMOST_EMPTY_THRD` writer"]
pub type W = crate::W<RAW_BUF_ALMOST_EMPTY_THRD_SPEC>;
#[doc = "Field `DSI_RAW_BUF_ALMOST_EMPTY_THRD` reader - this field configures the fifo almost empty threshold, is valid only when dmac as flow controller"]
pub type DSI_RAW_BUF_ALMOST_EMPTY_THRD_R = crate::FieldReader<u16>;
#[doc = "Field `DSI_RAW_BUF_ALMOST_EMPTY_THRD` writer - this field configures the fifo almost empty threshold, is valid only when dmac as flow controller"]
pub type DSI_RAW_BUF_ALMOST_EMPTY_THRD_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - this field configures the fifo almost empty threshold, is valid only when dmac as flow controller"]
    #[inline(always)]
    pub fn dsi_raw_buf_almost_empty_thrd(&self) -> DSI_RAW_BUF_ALMOST_EMPTY_THRD_R {
        DSI_RAW_BUF_ALMOST_EMPTY_THRD_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW_BUF_ALMOST_EMPTY_THRD")
            .field(
                "dsi_raw_buf_almost_empty_thrd",
                &self.dsi_raw_buf_almost_empty_thrd(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - this field configures the fifo almost empty threshold, is valid only when dmac as flow controller"]
    #[inline(always)]
    pub fn dsi_raw_buf_almost_empty_thrd(
        &mut self,
    ) -> DSI_RAW_BUF_ALMOST_EMPTY_THRD_W<RAW_BUF_ALMOST_EMPTY_THRD_SPEC> {
        DSI_RAW_BUF_ALMOST_EMPTY_THRD_W::new(self, 0)
    }
}
#[doc = "dsi_bridge buffer empty threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_buf_almost_empty_thrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw_buf_almost_empty_thrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_BUF_ALMOST_EMPTY_THRD_SPEC;
impl crate::RegisterSpec for RAW_BUF_ALMOST_EMPTY_THRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_buf_almost_empty_thrd::R`](R) reader structure"]
impl crate::Readable for RAW_BUF_ALMOST_EMPTY_THRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw_buf_almost_empty_thrd::W`](W) writer structure"]
impl crate::Writable for RAW_BUF_ALMOST_EMPTY_THRD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAW_BUF_ALMOST_EMPTY_THRD to value 0x0200"]
impl crate::Resettable for RAW_BUF_ALMOST_EMPTY_THRD_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
