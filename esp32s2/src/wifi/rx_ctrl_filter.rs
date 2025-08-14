#[doc = "Register `RX_CTRL_FILTER%s` reader"]
pub type R = crate::R<RX_CTRL_FILTER_SPEC>;
#[doc = "Register `RX_CTRL_FILTER%s` writer"]
pub type W = crate::W<RX_CTRL_FILTER_SPEC>;
#[doc = "Field `CONTROL_WRAPPER` reader - Filter Control Wrapper frames"]
pub type CONTROL_WRAPPER_R = crate::BitReader;
#[doc = "Field `CONTROL_WRAPPER` writer - Filter Control Wrapper frames"]
pub type CONTROL_WRAPPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_ACK_REQUEST` reader - Filter Block ACK Request frames"]
pub type BLOCK_ACK_REQUEST_R = crate::BitReader;
#[doc = "Field `BLOCK_ACK_REQUEST` writer - Filter Block ACK Request frames"]
pub type BLOCK_ACK_REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK_ACK` reader - Filter Block ACK frames"]
pub type BLOCK_ACK_R = crate::BitReader;
#[doc = "Field `BLOCK_ACK` writer - Filter Block ACK frames"]
pub type BLOCK_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS_POLL` reader - Filter PS-Poll frames"]
pub type PS_POLL_R = crate::BitReader;
#[doc = "Field `PS_POLL` writer - Filter PS-Poll frames"]
pub type PS_POLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - Filter RTS frames"]
pub type RTS_R = crate::BitReader;
#[doc = "Field `RTS` writer - Filter RTS frames"]
pub type RTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - Filter CTS frames"]
pub type CTS_R = crate::BitReader;
#[doc = "Field `CTS` writer - Filter CTS frames"]
pub type CTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Filter ACK frames"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - Filter ACK frames"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CF_END` reader - Filter CF-End frames"]
pub type CF_END_R = crate::BitReader;
#[doc = "Field `CF_END` writer - Filter CF-End frames"]
pub type CF_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CF_END_CF_ACK` reader - Filter CF-End+CF-Ack frames"]
pub type CF_END_CF_ACK_R = crate::BitReader;
#[doc = "Field `CF_END_CF_ACK` writer - Filter CF-End+CF-Ack frames"]
pub type CF_END_CF_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23 - Filter Control Wrapper frames"]
    #[inline(always)]
    pub fn control_wrapper(&self) -> CONTROL_WRAPPER_R {
        CONTROL_WRAPPER_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filter Block ACK Request frames"]
    #[inline(always)]
    pub fn block_ack_request(&self) -> BLOCK_ACK_REQUEST_R {
        BLOCK_ACK_REQUEST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Filter Block ACK frames"]
    #[inline(always)]
    pub fn block_ack(&self) -> BLOCK_ACK_R {
        BLOCK_ACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Filter PS-Poll frames"]
    #[inline(always)]
    pub fn ps_poll(&self) -> PS_POLL_R {
        PS_POLL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Filter RTS frames"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Filter CTS frames"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Filter ACK frames"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Filter CF-End frames"]
    #[inline(always)]
    pub fn cf_end(&self) -> CF_END_R {
        CF_END_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Filter CF-End+CF-Ack frames"]
    #[inline(always)]
    pub fn cf_end_cf_ack(&self) -> CF_END_CF_ACK_R {
        CF_END_CF_ACK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CTRL_FILTER")
            .field("control_wrapper", &self.control_wrapper())
            .field("block_ack_request", &self.block_ack_request())
            .field("block_ack", &self.block_ack())
            .field("ps_poll", &self.ps_poll())
            .field("rts", &self.rts())
            .field("cts", &self.cts())
            .field("ack", &self.ack())
            .field("cf_end", &self.cf_end())
            .field("cf_end_cf_ack", &self.cf_end_cf_ack())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - Filter Control Wrapper frames"]
    #[inline(always)]
    pub fn control_wrapper(&mut self) -> CONTROL_WRAPPER_W<RX_CTRL_FILTER_SPEC> {
        CONTROL_WRAPPER_W::new(self, 23)
    }
    #[doc = "Bit 24 - Filter Block ACK Request frames"]
    #[inline(always)]
    pub fn block_ack_request(&mut self) -> BLOCK_ACK_REQUEST_W<RX_CTRL_FILTER_SPEC> {
        BLOCK_ACK_REQUEST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Filter Block ACK frames"]
    #[inline(always)]
    pub fn block_ack(&mut self) -> BLOCK_ACK_W<RX_CTRL_FILTER_SPEC> {
        BLOCK_ACK_W::new(self, 25)
    }
    #[doc = "Bit 26 - Filter PS-Poll frames"]
    #[inline(always)]
    pub fn ps_poll(&mut self) -> PS_POLL_W<RX_CTRL_FILTER_SPEC> {
        PS_POLL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Filter RTS frames"]
    #[inline(always)]
    pub fn rts(&mut self) -> RTS_W<RX_CTRL_FILTER_SPEC> {
        RTS_W::new(self, 27)
    }
    #[doc = "Bit 28 - Filter CTS frames"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W<RX_CTRL_FILTER_SPEC> {
        CTS_W::new(self, 28)
    }
    #[doc = "Bit 29 - Filter ACK frames"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<RX_CTRL_FILTER_SPEC> {
        ACK_W::new(self, 29)
    }
    #[doc = "Bit 30 - Filter CF-End frames"]
    #[inline(always)]
    pub fn cf_end(&mut self) -> CF_END_W<RX_CTRL_FILTER_SPEC> {
        CF_END_W::new(self, 30)
    }
    #[doc = "Bit 31 - Filter CF-End+CF-Ack frames"]
    #[inline(always)]
    pub fn cf_end_cf_ack(&mut self) -> CF_END_CF_ACK_W<RX_CTRL_FILTER_SPEC> {
        CF_END_CF_ACK_W::new(self, 31)
    }
}
#[doc = "Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl_filter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl_filter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CTRL_FILTER_SPEC;
impl crate::RegisterSpec for RX_CTRL_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctrl_filter::R`](R) reader structure"]
impl crate::Readable for RX_CTRL_FILTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl_filter::W`](W) writer structure"]
impl crate::Writable for RX_CTRL_FILTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CTRL_FILTER%s to value 0"]
impl crate::Resettable for RX_CTRL_FILTER_SPEC {}
