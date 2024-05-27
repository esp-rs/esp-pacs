///Register `SDIO_ST` reader
pub type R = crate::R<SDIO_ST_SPEC>;
///Field `CMD_ST` reader -
pub type CMD_ST_R = crate::FieldReader;
///Field `FUNC_ST` reader -
pub type FUNC_ST_R = crate::FieldReader;
///Field `SDIO_WAKEUP` reader -
pub type SDIO_WAKEUP_R = crate::BitReader;
///Field `BUS_ST` reader -
pub type BUS_ST_R = crate::FieldReader;
///Field `FUNC1_ACC_STATE` reader -
pub type FUNC1_ACC_STATE_R = crate::FieldReader;
///Field `FUNC2_ACC_STATE` reader -
pub type FUNC2_ACC_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn cmd_st(&self) -> CMD_ST_R {
        CMD_ST_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:7
    #[inline(always)]
    pub fn func_st(&self) -> FUNC_ST_R {
        FUNC_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn sdio_wakeup(&self) -> SDIO_WAKEUP_R {
        SDIO_WAKEUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 12:14
    #[inline(always)]
    pub fn bus_st(&self) -> BUS_ST_R {
        BUS_ST_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:20
    #[inline(always)]
    pub fn func1_acc_state(&self) -> FUNC1_ACC_STATE_R {
        FUNC1_ACC_STATE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:28
    #[inline(always)]
    pub fn func2_acc_state(&self) -> FUNC2_ACC_STATE_R {
        FUNC2_ACC_STATE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_ST")
            .field("cmd_st", &self.cmd_st())
            .field("func_st", &self.func_st())
            .field("sdio_wakeup", &self.sdio_wakeup())
            .field("bus_st", &self.bus_st())
            .field("func1_acc_state", &self.func1_acc_state())
            .field("func2_acc_state", &self.func2_acc_state())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDIO_ST_SPEC;
impl crate::RegisterSpec for SDIO_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sdio_st::R`](R) reader structure
impl crate::Readable for SDIO_ST_SPEC {}
///`reset()` method sets SDIO_ST to value 0
impl crate::Resettable for SDIO_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
