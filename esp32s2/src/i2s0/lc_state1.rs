#[doc = "Register `LC_STATE1` reader"]
pub type R = crate::R<LC_STATE1_SPEC>;
#[doc = "Field `INLINK_DSCR_ADDR` reader - I2S DMA in descriptor address."]
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE` reader - I2S DMA in descriptor state."]
pub type IN_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `IN_STATE` reader - I2S DMA in data state."]
pub type IN_STATE_R = crate::FieldReader;
#[doc = "Field `INFIFO_CNT_DEBUG` reader - The remains of I2S DMA infifo data."]
pub type INFIFO_CNT_DEBUG_R = crate::FieldReader;
#[doc = "Field `IN_FULL` reader - I2S DMA infifo is full."]
pub type IN_FULL_R = crate::BitReader;
#[doc = "Field `IN_EMPTY` reader - I2S DMA infifo is empty."]
pub type IN_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:17 - I2S DMA in descriptor address."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - I2S DMA in descriptor state."]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - I2S DMA in data state."]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:29 - The remains of I2S DMA infifo data."]
    #[inline(always)]
    pub fn infifo_cnt_debug(&self) -> INFIFO_CNT_DEBUG_R {
        INFIFO_CNT_DEBUG_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - I2S DMA infifo is full."]
    #[inline(always)]
    pub fn in_full(&self) -> IN_FULL_R {
        IN_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2S DMA infifo is empty."]
    #[inline(always)]
    pub fn in_empty(&self) -> IN_EMPTY_R {
        IN_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_STATE1")
            .field("inlink_dscr_addr", &self.inlink_dscr_addr())
            .field("in_dscr_state", &self.in_dscr_state())
            .field("in_state", &self.in_state())
            .field("infifo_cnt_debug", &self.infifo_cnt_debug())
            .field("in_full", &self.in_full())
            .field("in_empty", &self.in_empty())
            .finish()
    }
}
#[doc = "I2S DMA RX status\n\nYou can [`read`](crate::Reg::read) this register and get [`lc_state1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LC_STATE1_SPEC;
impl crate::RegisterSpec for LC_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lc_state1::R`](R) reader structure"]
impl crate::Readable for LC_STATE1_SPEC {}
#[doc = "`reset()` method sets LC_STATE1 to value 0"]
impl crate::Resettable for LC_STATE1_SPEC {}
