///Register `EXTER_AXI_ERR` reader
pub type R = crate::R<EXTER_AXI_ERR_SPEC>;
///Field `EXTER_RID_ERR_CNT` reader - AXI read id err cnt
pub type EXTER_RID_ERR_CNT_R = crate::FieldReader;
///Field `EXTER_RRESP_ERR_CNT` reader - AXI read resp err cnt
pub type EXTER_RRESP_ERR_CNT_R = crate::FieldReader;
///Field `EXTER_WRESP_ERR_CNT` reader - AXI write resp err cnt
pub type EXTER_WRESP_ERR_CNT_R = crate::FieldReader;
///Field `EXTER_RD_FIFO_CNT` reader - AXI read cmd fifo remain cmd count
pub type EXTER_RD_FIFO_CNT_R = crate::FieldReader;
///Field `EXTER_RD_BAK_FIFO_CNT` reader - AXI read backup cmd fifo remain cmd count
pub type EXTER_RD_BAK_FIFO_CNT_R = crate::FieldReader;
///Field `EXTER_WR_FIFO_CNT` reader - AXI write cmd fifo remain cmd count
pub type EXTER_WR_FIFO_CNT_R = crate::FieldReader;
///Field `EXTER_WR_BAK_FIFO_CNT` reader - AXI write backup cmd fifo remain cmd count
pub type EXTER_WR_BAK_FIFO_CNT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - AXI read id err cnt
    #[inline(always)]
    pub fn exter_rid_err_cnt(&self) -> EXTER_RID_ERR_CNT_R {
        EXTER_RID_ERR_CNT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - AXI read resp err cnt
    #[inline(always)]
    pub fn exter_rresp_err_cnt(&self) -> EXTER_RRESP_ERR_CNT_R {
        EXTER_RRESP_ERR_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - AXI write resp err cnt
    #[inline(always)]
    pub fn exter_wresp_err_cnt(&self) -> EXTER_WRESP_ERR_CNT_R {
        EXTER_WRESP_ERR_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - AXI read cmd fifo remain cmd count
    #[inline(always)]
    pub fn exter_rd_fifo_cnt(&self) -> EXTER_RD_FIFO_CNT_R {
        EXTER_RD_FIFO_CNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:18 - AXI read backup cmd fifo remain cmd count
    #[inline(always)]
    pub fn exter_rd_bak_fifo_cnt(&self) -> EXTER_RD_BAK_FIFO_CNT_R {
        EXTER_RD_BAK_FIFO_CNT_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    ///Bits 19:21 - AXI write cmd fifo remain cmd count
    #[inline(always)]
    pub fn exter_wr_fifo_cnt(&self) -> EXTER_WR_FIFO_CNT_R {
        EXTER_WR_FIFO_CNT_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:25 - AXI write backup cmd fifo remain cmd count
    #[inline(always)]
    pub fn exter_wr_bak_fifo_cnt(&self) -> EXTER_WR_BAK_FIFO_CNT_R {
        EXTER_WR_BAK_FIFO_CNT_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTER_AXI_ERR")
            .field("exter_rid_err_cnt", &self.exter_rid_err_cnt())
            .field("exter_rresp_err_cnt", &self.exter_rresp_err_cnt())
            .field("exter_wresp_err_cnt", &self.exter_wresp_err_cnt())
            .field("exter_rd_fifo_cnt", &self.exter_rd_fifo_cnt())
            .field("exter_rd_bak_fifo_cnt", &self.exter_rd_bak_fifo_cnt())
            .field("exter_wr_fifo_cnt", &self.exter_wr_fifo_cnt())
            .field("exter_wr_bak_fifo_cnt", &self.exter_wr_bak_fifo_cnt())
            .finish()
    }
}
/**exter memory axi err register

You can [`read`](crate::generic::Reg::read) this register and get [`exter_axi_err::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXTER_AXI_ERR_SPEC;
impl crate::RegisterSpec for EXTER_AXI_ERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`exter_axi_err::R`](R) reader structure
impl crate::Readable for EXTER_AXI_ERR_SPEC {}
///`reset()` method sets EXTER_AXI_ERR to value 0
impl crate::Resettable for EXTER_AXI_ERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
