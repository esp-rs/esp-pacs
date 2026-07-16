#[doc = "Register `AXI_ERR` reader"]
pub type R = crate::R<AXI_ERR_SPEC>;
#[doc = "Field `RID_ERR_CNT` reader - AXI read id err cnt"]
pub type RID_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `RRESP_ERR_CNT` reader - AXI read resp err cnt"]
pub type RRESP_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `WRESP_ERR_CNT` reader - AXI write resp err cnt"]
pub type WRESP_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `RD_FIFO_CNT` reader - AXI read cmd fifo remain cmd count"]
pub type RD_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `RD_BAK_FIFO_CNT` reader - AXI read backup cmd fifo remain cmd count"]
pub type RD_BAK_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `WR_FIFO_CNT` reader - AXI write cmd fifo remain cmd count"]
pub type WR_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `WR_BAK_FIFO_CNT` reader - AXI write backup cmd fifo remain cmd count"]
pub type WR_BAK_FIFO_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AXI read id err cnt"]
    #[inline(always)]
    pub fn rid_err_cnt(&self) -> RID_ERR_CNT_R {
        RID_ERR_CNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AXI read resp err cnt"]
    #[inline(always)]
    pub fn rresp_err_cnt(&self) -> RRESP_ERR_CNT_R {
        RRESP_ERR_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AXI write resp err cnt"]
    #[inline(always)]
    pub fn wresp_err_cnt(&self) -> WRESP_ERR_CNT_R {
        WRESP_ERR_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - AXI read cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn rd_fifo_cnt(&self) -> RD_FIFO_CNT_R {
        RD_FIFO_CNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:18 - AXI read backup cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn rd_bak_fifo_cnt(&self) -> RD_BAK_FIFO_CNT_R {
        RD_BAK_FIFO_CNT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21 - AXI write cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn wr_fifo_cnt(&self) -> WR_FIFO_CNT_R {
        WR_FIFO_CNT_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - AXI write backup cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn wr_bak_fifo_cnt(&self) -> WR_BAK_FIFO_CNT_R {
        WR_BAK_FIFO_CNT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ERR")
            .field("rid_err_cnt", &self.rid_err_cnt())
            .field("rresp_err_cnt", &self.rresp_err_cnt())
            .field("wresp_err_cnt", &self.wresp_err_cnt())
            .field("rd_fifo_cnt", &self.rd_fifo_cnt())
            .field("rd_bak_fifo_cnt", &self.rd_bak_fifo_cnt())
            .field("wr_fifo_cnt", &self.wr_fifo_cnt())
            .field("wr_bak_fifo_cnt", &self.wr_bak_fifo_cnt())
            .finish()
    }
}
#[doc = "Represents the status of th axi bus\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_ERR_SPEC;
impl crate::RegisterSpec for AXI_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_err::R`](R) reader structure"]
impl crate::Readable for AXI_ERR_SPEC {}
#[doc = "`reset()` method sets AXI_ERR to value 0"]
impl crate::Resettable for AXI_ERR_SPEC {}
