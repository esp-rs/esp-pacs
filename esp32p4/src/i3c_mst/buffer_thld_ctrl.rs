///Register `BUFFER_THLD_CTRL` reader
pub type R = crate::R<BUFFER_THLD_CTRL_SPEC>;
///Register `BUFFER_THLD_CTRL` writer
pub type W = crate::W<BUFFER_THLD_CTRL_SPEC>;
///Field `REG_CMD_BUF_EMPTY_THLD` reader - Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
pub type REG_CMD_BUF_EMPTY_THLD_R = crate::FieldReader;
///Field `REG_CMD_BUF_EMPTY_THLD` writer - Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
pub type REG_CMD_BUF_EMPTY_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `REG_RESP_BUF_THLD` reader - Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
pub type REG_RESP_BUF_THLD_R = crate::FieldReader;
///Field `REG_RESP_BUF_THLD` writer - Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
pub type REG_RESP_BUF_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REG_IBI_DATA_BUF_THLD` reader - In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
pub type REG_IBI_DATA_BUF_THLD_R = crate::FieldReader;
///Field `REG_IBI_DATA_BUF_THLD` writer - In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
pub type REG_IBI_DATA_BUF_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `REG_IBI_STATUS_BUF_THLD` reader - NA
pub type REG_IBI_STATUS_BUF_THLD_R = crate::FieldReader;
///Field `REG_IBI_STATUS_BUF_THLD` writer - NA
pub type REG_IBI_STATUS_BUF_THLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:3 - Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    #[inline(always)]
    pub fn reg_cmd_buf_empty_thld(&self) -> REG_CMD_BUF_EMPTY_THLD_R {
        REG_CMD_BUF_EMPTY_THLD_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:8 - Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    #[inline(always)]
    pub fn reg_resp_buf_thld(&self) -> REG_RESP_BUF_THLD_R {
        REG_RESP_BUF_THLD_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 12:14 - In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    #[inline(always)]
    pub fn reg_ibi_data_buf_thld(&self) -> REG_IBI_DATA_BUF_THLD_R {
        REG_IBI_DATA_BUF_THLD_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 18:20 - NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_thld(&self) -> REG_IBI_STATUS_BUF_THLD_R {
        REG_IBI_STATUS_BUF_THLD_R::new(((self.bits >> 18) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFFER_THLD_CTRL")
            .field("reg_cmd_buf_empty_thld", &self.reg_cmd_buf_empty_thld())
            .field("reg_resp_buf_thld", &self.reg_resp_buf_thld())
            .field("reg_ibi_data_buf_thld", &self.reg_ibi_data_buf_thld())
            .field("reg_ibi_status_buf_thld", &self.reg_ibi_status_buf_thld())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Command Buffer Empty Threshold Value is used to control the number of empty locations(or greater) in the Command Buffer that trigger CMD_BUFFER_READY_STAT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn reg_cmd_buf_empty_thld(&mut self) -> REG_CMD_BUF_EMPTY_THLD_W<BUFFER_THLD_CTRL_SPEC> {
        REG_CMD_BUF_EMPTY_THLD_W::new(self, 0)
    }
    ///Bits 6:8 - Response Buffer Threshold Value is used to control the number of entries in the Response Buffer that trigger the RESP_READY_STAT_INTR.
    #[inline(always)]
    #[must_use]
    pub fn reg_resp_buf_thld(&mut self) -> REG_RESP_BUF_THLD_W<BUFFER_THLD_CTRL_SPEC> {
        REG_RESP_BUF_THLD_W::new(self, 6)
    }
    ///Bits 12:14 - In-Band Interrupt Data Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI data entries in the IBI buffer that trigger the IBI_DATA_THLD_STAT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_data_buf_thld(&mut self) -> REG_IBI_DATA_BUF_THLD_W<BUFFER_THLD_CTRL_SPEC> {
        REG_IBI_DATA_BUF_THLD_W::new(self, 12)
    }
    ///Bits 18:20 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_status_buf_thld(&mut self) -> REG_IBI_STATUS_BUF_THLD_W<BUFFER_THLD_CTRL_SPEC> {
        REG_IBI_STATUS_BUF_THLD_W::new(self, 18)
    }
}
/**In-Band Interrupt Status Threshold Value . Every In Band Interrupt received by I3C controller generates an IBI status. This field controls the number of IBI status entries in the IBI buffer that trigger the IBI_STATUS_THLD_STAT interrupt.

You can [`read`](crate::generic::Reg::read) this register and get [`buffer_thld_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buffer_thld_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BUFFER_THLD_CTRL_SPEC;
impl crate::RegisterSpec for BUFFER_THLD_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`buffer_thld_ctrl::R`](R) reader structure
impl crate::Readable for BUFFER_THLD_CTRL_SPEC {}
///`write(|w| ..)` method takes [`buffer_thld_ctrl::W`](W) writer structure
impl crate::Writable for BUFFER_THLD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BUFFER_THLD_CTRL to value 0x0004_1041
impl crate::Resettable for BUFFER_THLD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0004_1041;
}
