///Register `TIME_OUT_VALUE` reader
pub type R = crate::R<TIME_OUT_VALUE_SPEC>;
///Register `TIME_OUT_VALUE` writer
pub type W = crate::W<TIME_OUT_VALUE_SPEC>;
///Field `REG_RESP_BUF_TO_VALUE` reader - NA
pub type REG_RESP_BUF_TO_VALUE_R = crate::FieldReader;
///Field `REG_RESP_BUF_TO_VALUE` writer - NA
pub type REG_RESP_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `REG_RESP_BUF_TO_EN` reader - NA
pub type REG_RESP_BUF_TO_EN_R = crate::BitReader;
///Field `REG_RESP_BUF_TO_EN` writer - NA
pub type REG_RESP_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_IBI_DATA_BUF_TO_VALUE` reader - NA
pub type REG_IBI_DATA_BUF_TO_VALUE_R = crate::FieldReader;
///Field `REG_IBI_DATA_BUF_TO_VALUE` writer - NA
pub type REG_IBI_DATA_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `REG_IBI_DATA_BUF_TO_EN` reader - NA
pub type REG_IBI_DATA_BUF_TO_EN_R = crate::BitReader;
///Field `REG_IBI_DATA_BUF_TO_EN` writer - NA
pub type REG_IBI_DATA_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_IBI_STATUS_BUF_TO_VALUE` reader - NA
pub type REG_IBI_STATUS_BUF_TO_VALUE_R = crate::FieldReader;
///Field `REG_IBI_STATUS_BUF_TO_VALUE` writer - NA
pub type REG_IBI_STATUS_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `REG_IBI_STATUS_BUF_TO_EN` reader - NA
pub type REG_IBI_STATUS_BUF_TO_EN_R = crate::BitReader;
///Field `REG_IBI_STATUS_BUF_TO_EN` writer - NA
pub type REG_IBI_STATUS_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_RX_DATA_BUF_TO_VALUE` reader - NA
pub type REG_RX_DATA_BUF_TO_VALUE_R = crate::FieldReader;
///Field `REG_RX_DATA_BUF_TO_VALUE` writer - NA
pub type REG_RX_DATA_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `REG_RX_DATA_BUF_TO_EN` reader - NA
pub type REG_RX_DATA_BUF_TO_EN_R = crate::BitReader;
///Field `REG_RX_DATA_BUF_TO_EN` writer - NA
pub type REG_RX_DATA_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:4 - NA
    #[inline(always)]
    pub fn reg_resp_buf_to_value(&self) -> REG_RESP_BUF_TO_VALUE_R {
        REG_RESP_BUF_TO_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - NA
    #[inline(always)]
    pub fn reg_resp_buf_to_en(&self) -> REG_RESP_BUF_TO_EN_R {
        REG_RESP_BUF_TO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:10 - NA
    #[inline(always)]
    pub fn reg_ibi_data_buf_to_value(&self) -> REG_IBI_DATA_BUF_TO_VALUE_R {
        REG_IBI_DATA_BUF_TO_VALUE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bit 11 - NA
    #[inline(always)]
    pub fn reg_ibi_data_buf_to_en(&self) -> REG_IBI_DATA_BUF_TO_EN_R {
        REG_IBI_DATA_BUF_TO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:16 - NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_to_value(&self) -> REG_IBI_STATUS_BUF_TO_VALUE_R {
        REG_IBI_STATUS_BUF_TO_VALUE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bit 17 - NA
    #[inline(always)]
    pub fn reg_ibi_status_buf_to_en(&self) -> REG_IBI_STATUS_BUF_TO_EN_R {
        REG_IBI_STATUS_BUF_TO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:22 - NA
    #[inline(always)]
    pub fn reg_rx_data_buf_to_value(&self) -> REG_RX_DATA_BUF_TO_VALUE_R {
        REG_RX_DATA_BUF_TO_VALUE_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bit 23 - NA
    #[inline(always)]
    pub fn reg_rx_data_buf_to_en(&self) -> REG_RX_DATA_BUF_TO_EN_R {
        REG_RX_DATA_BUF_TO_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME_OUT_VALUE")
            .field("reg_resp_buf_to_value", &self.reg_resp_buf_to_value())
            .field("reg_resp_buf_to_en", &self.reg_resp_buf_to_en())
            .field(
                "reg_ibi_data_buf_to_value",
                &self.reg_ibi_data_buf_to_value(),
            )
            .field("reg_ibi_data_buf_to_en", &self.reg_ibi_data_buf_to_en())
            .field(
                "reg_ibi_status_buf_to_value",
                &self.reg_ibi_status_buf_to_value(),
            )
            .field("reg_ibi_status_buf_to_en", &self.reg_ibi_status_buf_to_en())
            .field("reg_rx_data_buf_to_value", &self.reg_rx_data_buf_to_value())
            .field("reg_rx_data_buf_to_en", &self.reg_rx_data_buf_to_en())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_resp_buf_to_value(&mut self) -> REG_RESP_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_RESP_BUF_TO_VALUE_W::new(self, 0)
    }
    ///Bit 5 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_resp_buf_to_en(&mut self) -> REG_RESP_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_RESP_BUF_TO_EN_W::new(self, 5)
    }
    ///Bits 6:10 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_data_buf_to_value(
        &mut self,
    ) -> REG_IBI_DATA_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_DATA_BUF_TO_VALUE_W::new(self, 6)
    }
    ///Bit 11 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_data_buf_to_en(&mut self) -> REG_IBI_DATA_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_DATA_BUF_TO_EN_W::new(self, 11)
    }
    ///Bits 12:16 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_status_buf_to_value(
        &mut self,
    ) -> REG_IBI_STATUS_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_STATUS_BUF_TO_VALUE_W::new(self, 12)
    }
    ///Bit 17 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_status_buf_to_en(&mut self) -> REG_IBI_STATUS_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_STATUS_BUF_TO_EN_W::new(self, 17)
    }
    ///Bits 18:22 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_data_buf_to_value(&mut self) -> REG_RX_DATA_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_RX_DATA_BUF_TO_VALUE_W::new(self, 18)
    }
    ///Bit 23 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_data_buf_to_en(&mut self) -> REG_RX_DATA_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_RX_DATA_BUF_TO_EN_W::new(self, 23)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`time_out_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_out_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIME_OUT_VALUE_SPEC;
impl crate::RegisterSpec for TIME_OUT_VALUE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`time_out_value::R`](R) reader structure
impl crate::Readable for TIME_OUT_VALUE_SPEC {}
///`write(|w| ..)` method takes [`time_out_value::W`](W) writer structure
impl crate::Writable for TIME_OUT_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIME_OUT_VALUE to value 0x0041_0410
impl crate::Resettable for TIME_OUT_VALUE_SPEC {
    const RESET_VALUE: u32 = 0x0041_0410;
}
