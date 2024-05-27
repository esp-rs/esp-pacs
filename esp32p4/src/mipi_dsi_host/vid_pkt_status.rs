///Register `VID_PKT_STATUS` reader
pub type R = crate::R<VID_PKT_STATUS_SPEC>;
///Field `DPI_CMD_W_EMPTY` reader - NA
pub type DPI_CMD_W_EMPTY_R = crate::BitReader;
///Field `DPI_CMD_W_FULL` reader - NA
pub type DPI_CMD_W_FULL_R = crate::BitReader;
///Field `DPI_PLD_W_EMPTY` reader - NA
pub type DPI_PLD_W_EMPTY_R = crate::BitReader;
///Field `DPI_PLD_W_FULL` reader - NA
pub type DPI_PLD_W_FULL_R = crate::BitReader;
///Field `DPI_BUFF_PLD_EMPTY` reader - NA
pub type DPI_BUFF_PLD_EMPTY_R = crate::BitReader;
///Field `DPI_BUFF_PLD_FULL` reader - NA
pub type DPI_BUFF_PLD_FULL_R = crate::BitReader;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn dpi_cmd_w_empty(&self) -> DPI_CMD_W_EMPTY_R {
        DPI_CMD_W_EMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn dpi_cmd_w_full(&self) -> DPI_CMD_W_FULL_R {
        DPI_CMD_W_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn dpi_pld_w_empty(&self) -> DPI_PLD_W_EMPTY_R {
        DPI_PLD_W_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NA
    #[inline(always)]
    pub fn dpi_pld_w_full(&self) -> DPI_PLD_W_FULL_R {
        DPI_PLD_W_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - NA
    #[inline(always)]
    pub fn dpi_buff_pld_empty(&self) -> DPI_BUFF_PLD_EMPTY_R {
        DPI_BUFF_PLD_EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NA
    #[inline(always)]
    pub fn dpi_buff_pld_full(&self) -> DPI_BUFF_PLD_FULL_R {
        DPI_BUFF_PLD_FULL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VID_PKT_STATUS")
            .field("dpi_cmd_w_empty", &self.dpi_cmd_w_empty())
            .field("dpi_cmd_w_full", &self.dpi_cmd_w_full())
            .field("dpi_pld_w_empty", &self.dpi_pld_w_empty())
            .field("dpi_pld_w_full", &self.dpi_pld_w_full())
            .field("dpi_buff_pld_empty", &self.dpi_buff_pld_empty())
            .field("dpi_buff_pld_full", &self.dpi_buff_pld_full())
            .finish()
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`vid_pkt_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VID_PKT_STATUS_SPEC;
impl crate::RegisterSpec for VID_PKT_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vid_pkt_status::R`](R) reader structure
impl crate::Readable for VID_PKT_STATUS_SPEC {}
///`reset()` method sets VID_PKT_STATUS to value 0x0001_0005
impl crate::Resettable for VID_PKT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0001_0005;
}
