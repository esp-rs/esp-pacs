#[doc = "Register `INTER_AXI_ERR` reader"]
pub type R = crate::R<INTER_AXI_ERR_SPEC>;
#[doc = "Field `INTER_RID_ERR_CNT` reader - AXI read id err cnt"]
pub type INTER_RID_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `INTER_RRESP_ERR_CNT` reader - AXI read resp err cnt"]
pub type INTER_RRESP_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `INTER_WRESP_ERR_CNT` reader - AXI write resp err cnt"]
pub type INTER_WRESP_ERR_CNT_R = crate::FieldReader;
#[doc = "Field `INTER_RD_FIFO_CNT` reader - AXI read cmd fifo remain cmd count"]
pub type INTER_RD_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `INTER_RD_BAK_FIFO_CNT` reader - AXI read backup cmd fifo remain cmd count"]
pub type INTER_RD_BAK_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `INTER_WR_FIFO_CNT` reader - AXI write cmd fifo remain cmd count"]
pub type INTER_WR_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `INTER_WR_BAK_FIFO_CNT` reader - AXI write backup cmd fifo remain cmd count"]
pub type INTER_WR_BAK_FIFO_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - AXI read id err cnt"]
    #[inline(always)]
    pub fn inter_rid_err_cnt(&self) -> INTER_RID_ERR_CNT_R {
        INTER_RID_ERR_CNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AXI read resp err cnt"]
    #[inline(always)]
    pub fn inter_rresp_err_cnt(&self) -> INTER_RRESP_ERR_CNT_R {
        INTER_RRESP_ERR_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AXI write resp err cnt"]
    #[inline(always)]
    pub fn inter_wresp_err_cnt(&self) -> INTER_WRESP_ERR_CNT_R {
        INTER_WRESP_ERR_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - AXI read cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn inter_rd_fifo_cnt(&self) -> INTER_RD_FIFO_CNT_R {
        INTER_RD_FIFO_CNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:18 - AXI read backup cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn inter_rd_bak_fifo_cnt(&self) -> INTER_RD_BAK_FIFO_CNT_R {
        INTER_RD_BAK_FIFO_CNT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:21 - AXI write cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn inter_wr_fifo_cnt(&self) -> INTER_WR_FIFO_CNT_R {
        INTER_WR_FIFO_CNT_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - AXI write backup cmd fifo remain cmd count"]
    #[inline(always)]
    pub fn inter_wr_bak_fifo_cnt(&self) -> INTER_WR_BAK_FIFO_CNT_R {
        INTER_WR_BAK_FIFO_CNT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTER_AXI_ERR")
            .field("inter_rid_err_cnt", &self.inter_rid_err_cnt())
            .field("inter_rresp_err_cnt", &self.inter_rresp_err_cnt())
            .field("inter_wresp_err_cnt", &self.inter_wresp_err_cnt())
            .field("inter_rd_fifo_cnt", &self.inter_rd_fifo_cnt())
            .field("inter_rd_bak_fifo_cnt", &self.inter_rd_bak_fifo_cnt())
            .field("inter_wr_fifo_cnt", &self.inter_wr_fifo_cnt())
            .field("inter_wr_bak_fifo_cnt", &self.inter_wr_bak_fifo_cnt())
            .finish()
    }
}
#[doc = "inter memory axi err register\n\nYou can [`read`](crate::Reg::read) this register and get [`inter_axi_err::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTER_AXI_ERR_SPEC;
impl crate::RegisterSpec for INTER_AXI_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inter_axi_err::R`](R) reader structure"]
impl crate::Readable for INTER_AXI_ERR_SPEC {}
#[doc = "`reset()` method sets INTER_AXI_ERR to value 0"]
impl crate::Resettable for INTER_AXI_ERR_SPEC {}
