///Register `CMD_PKT_STATUS` reader
pub type R = crate::R<CMD_PKT_STATUS_SPEC>;
///Field `GEN_CMD_EMPTY` reader - NA
pub type GEN_CMD_EMPTY_R = crate::BitReader;
///Field `GEN_CMD_FULL` reader - NA
pub type GEN_CMD_FULL_R = crate::BitReader;
///Field `GEN_PLD_W_EMPTY` reader - NA
pub type GEN_PLD_W_EMPTY_R = crate::BitReader;
///Field `GEN_PLD_W_FULL` reader - NA
pub type GEN_PLD_W_FULL_R = crate::BitReader;
///Field `GEN_PLD_R_EMPTY` reader - NA
pub type GEN_PLD_R_EMPTY_R = crate::BitReader;
///Field `GEN_PLD_R_FULL` reader - NA
pub type GEN_PLD_R_FULL_R = crate::BitReader;
///Field `GEN_RD_CMD_BUSY` reader - NA
pub type GEN_RD_CMD_BUSY_R = crate::BitReader;
///Field `GEN_BUFF_CMD_EMPTY` reader - NA
pub type GEN_BUFF_CMD_EMPTY_R = crate::BitReader;
///Field `GEN_BUFF_CMD_FULL` reader - NA
pub type GEN_BUFF_CMD_FULL_R = crate::BitReader;
///Field `GEN_BUFF_PLD_EMPTY` reader - NA
pub type GEN_BUFF_PLD_EMPTY_R = crate::BitReader;
///Field `GEN_BUFF_PLD_FULL` reader - NA
pub type GEN_BUFF_PLD_FULL_R = crate::BitReader;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn gen_cmd_empty(&self) -> GEN_CMD_EMPTY_R {
        GEN_CMD_EMPTY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn gen_cmd_full(&self) -> GEN_CMD_FULL_R {
        GEN_CMD_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn gen_pld_w_empty(&self) -> GEN_PLD_W_EMPTY_R {
        GEN_PLD_W_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NA
    #[inline(always)]
    pub fn gen_pld_w_full(&self) -> GEN_PLD_W_FULL_R {
        GEN_PLD_W_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NA
    #[inline(always)]
    pub fn gen_pld_r_empty(&self) -> GEN_PLD_R_EMPTY_R {
        GEN_PLD_R_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NA
    #[inline(always)]
    pub fn gen_pld_r_full(&self) -> GEN_PLD_R_FULL_R {
        GEN_PLD_R_FULL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - NA
    #[inline(always)]
    pub fn gen_rd_cmd_busy(&self) -> GEN_RD_CMD_BUSY_R {
        GEN_RD_CMD_BUSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - NA
    #[inline(always)]
    pub fn gen_buff_cmd_empty(&self) -> GEN_BUFF_CMD_EMPTY_R {
        GEN_BUFF_CMD_EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NA
    #[inline(always)]
    pub fn gen_buff_cmd_full(&self) -> GEN_BUFF_CMD_FULL_R {
        GEN_BUFF_CMD_FULL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - NA
    #[inline(always)]
    pub fn gen_buff_pld_empty(&self) -> GEN_BUFF_PLD_EMPTY_R {
        GEN_BUFF_PLD_EMPTY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - NA
    #[inline(always)]
    pub fn gen_buff_pld_full(&self) -> GEN_BUFF_PLD_FULL_R {
        GEN_BUFF_PLD_FULL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_PKT_STATUS")
            .field("gen_cmd_empty", &self.gen_cmd_empty())
            .field("gen_cmd_full", &self.gen_cmd_full())
            .field("gen_pld_w_empty", &self.gen_pld_w_empty())
            .field("gen_pld_w_full", &self.gen_pld_w_full())
            .field("gen_pld_r_empty", &self.gen_pld_r_empty())
            .field("gen_pld_r_full", &self.gen_pld_r_full())
            .field("gen_rd_cmd_busy", &self.gen_rd_cmd_busy())
            .field("gen_buff_cmd_empty", &self.gen_buff_cmd_empty())
            .field("gen_buff_cmd_full", &self.gen_buff_cmd_full())
            .field("gen_buff_pld_empty", &self.gen_buff_pld_empty())
            .field("gen_buff_pld_full", &self.gen_buff_pld_full())
            .finish()
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`cmd_pkt_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CMD_PKT_STATUS_SPEC;
impl crate::RegisterSpec for CMD_PKT_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cmd_pkt_status::R`](R) reader structure
impl crate::Readable for CMD_PKT_STATUS_SPEC {}
///`reset()` method sets CMD_PKT_STATUS to value 0x0005_0015
impl crate::Resettable for CMD_PKT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x0005_0015;
}
