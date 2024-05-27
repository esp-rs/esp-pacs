///Register `LC_STATE0` reader
pub type R = crate::R<LC_STATE0_SPEC>;
///Field `OUTLINK_DSCR_ADDR` reader - I2S DMA out descriptor address.
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
///Field `OUT_DSCR_STATE` reader - I2S DMA out descriptor state.
pub type OUT_DSCR_STATE_R = crate::FieldReader;
///Field `OUT_STATE` reader - I2S DMA out data state.
pub type OUT_STATE_R = crate::FieldReader;
///Field `OUTFIFO_CNT` reader - The remains of I2S DMA outfifo data.
pub type OUTFIFO_CNT_R = crate::FieldReader;
///Field `OUT_FULL` reader - I2S DMA outfifo is full.
pub type OUT_FULL_R = crate::BitReader;
///Field `OUT_EMPTY` reader - I2S DMA outfifo is empty.
pub type OUT_EMPTY_R = crate::BitReader;
impl R {
    ///Bits 0:17 - I2S DMA out descriptor address.
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:19 - I2S DMA out descriptor state.
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:22 - I2S DMA out data state.
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:29 - The remains of I2S DMA outfifo data.
    #[inline(always)]
    pub fn outfifo_cnt(&self) -> OUTFIFO_CNT_R {
        OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    ///Bit 30 - I2S DMA outfifo is full.
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - I2S DMA outfifo is empty.
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LC_STATE0")
            .field("outlink_dscr_addr", &self.outlink_dscr_addr())
            .field("out_dscr_state", &self.out_dscr_state())
            .field("out_state", &self.out_state())
            .field("outfifo_cnt", &self.outfifo_cnt())
            .field("out_full", &self.out_full())
            .field("out_empty", &self.out_empty())
            .finish()
    }
}
/**I2S DMA TX status

You can [`read`](crate::generic::Reg::read) this register and get [`lc_state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LC_STATE0_SPEC;
impl crate::RegisterSpec for LC_STATE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lc_state0::R`](R) reader structure
impl crate::Readable for LC_STATE0_SPEC {}
///`reset()` method sets LC_STATE0 to value 0
impl crate::Resettable for LC_STATE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
