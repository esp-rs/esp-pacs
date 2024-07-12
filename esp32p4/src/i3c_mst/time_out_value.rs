#[doc = "Register `TIME_OUT_VALUE` reader"]
pub type R = crate::R<TIME_OUT_VALUE_SPEC>;
#[doc = "Register `TIME_OUT_VALUE` writer"]
pub type W = crate::W<TIME_OUT_VALUE_SPEC>;
#[doc = "Field `REG_RESP_BUF_TO_VALUE` reader - NA"]
pub type REG_RESP_BUF_TO_VALUE_R = crate::FieldReader;
#[doc = "Field `REG_RESP_BUF_TO_VALUE` writer - NA"]
pub type REG_RESP_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_RESP_BUF_TO_EN` reader - NA"]
pub type REG_RESP_BUF_TO_EN_R = crate::BitReader;
#[doc = "Field `REG_RESP_BUF_TO_EN` writer - NA"]
pub type REG_RESP_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IBI_DATA_BUF_TO_VALUE` reader - NA"]
pub type REG_IBI_DATA_BUF_TO_VALUE_R = crate::FieldReader;
#[doc = "Field `REG_IBI_DATA_BUF_TO_VALUE` writer - NA"]
pub type REG_IBI_DATA_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_IBI_DATA_BUF_TO_EN` reader - NA"]
pub type REG_IBI_DATA_BUF_TO_EN_R = crate::BitReader;
#[doc = "Field `REG_IBI_DATA_BUF_TO_EN` writer - NA"]
pub type REG_IBI_DATA_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IBI_STATUS_BUF_TO_VALUE` reader - NA"]
pub type REG_IBI_STATUS_BUF_TO_VALUE_R = crate::FieldReader;
#[doc = "Field `REG_IBI_STATUS_BUF_TO_VALUE` writer - NA"]
pub type REG_IBI_STATUS_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_IBI_STATUS_BUF_TO_EN` reader - NA"]
pub type REG_IBI_STATUS_BUF_TO_EN_R = crate::BitReader;
#[doc = "Field `REG_IBI_STATUS_BUF_TO_EN` writer - NA"]
pub type REG_IBI_STATUS_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RX_DATA_BUF_TO_VALUE` reader - NA"]
pub type REG_RX_DATA_BUF_TO_VALUE_R = crate::FieldReader;
#[doc = "Field `REG_RX_DATA_BUF_TO_VALUE` writer - NA"]
pub type REG_RX_DATA_BUF_TO_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_RX_DATA_BUF_TO_EN` reader - NA"]
pub type REG_RX_DATA_BUF_TO_EN_R = crate::BitReader;
#[doc = "Field `REG_RX_DATA_BUF_TO_EN` writer - NA"]
pub type REG_RX_DATA_BUF_TO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - NA"]
    #[inline(always)]
    pub fn reg_resp_buf_to_value(&self) -> REG_RESP_BUF_TO_VALUE_R {
        REG_RESP_BUF_TO_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_resp_buf_to_en(&self) -> REG_RESP_BUF_TO_EN_R {
        REG_RESP_BUF_TO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - NA"]
    #[inline(always)]
    pub fn reg_ibi_data_buf_to_value(&self) -> REG_IBI_DATA_BUF_TO_VALUE_R {
        REG_IBI_DATA_BUF_TO_VALUE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn reg_ibi_data_buf_to_en(&self) -> REG_IBI_DATA_BUF_TO_EN_R {
        REG_IBI_DATA_BUF_TO_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - NA"]
    #[inline(always)]
    pub fn reg_ibi_status_buf_to_value(&self) -> REG_IBI_STATUS_BUF_TO_VALUE_R {
        REG_IBI_STATUS_BUF_TO_VALUE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn reg_ibi_status_buf_to_en(&self) -> REG_IBI_STATUS_BUF_TO_EN_R {
        REG_IBI_STATUS_BUF_TO_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - NA"]
    #[inline(always)]
    pub fn reg_rx_data_buf_to_value(&self) -> REG_RX_DATA_BUF_TO_VALUE_R {
        REG_RX_DATA_BUF_TO_VALUE_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - NA"]
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
    #[doc = "Bits 0:4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_resp_buf_to_value(&mut self) -> REG_RESP_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_RESP_BUF_TO_VALUE_W::new(self, 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_resp_buf_to_en(&mut self) -> REG_RESP_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_RESP_BUF_TO_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_data_buf_to_value(
        &mut self,
    ) -> REG_IBI_DATA_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_DATA_BUF_TO_VALUE_W::new(self, 6)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_data_buf_to_en(&mut self) -> REG_IBI_DATA_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_DATA_BUF_TO_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_status_buf_to_value(
        &mut self,
    ) -> REG_IBI_STATUS_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_STATUS_BUF_TO_VALUE_W::new(self, 12)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_status_buf_to_en(&mut self) -> REG_IBI_STATUS_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_IBI_STATUS_BUF_TO_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:22 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_data_buf_to_value(&mut self) -> REG_RX_DATA_BUF_TO_VALUE_W<TIME_OUT_VALUE_SPEC> {
        REG_RX_DATA_BUF_TO_VALUE_W::new(self, 18)
    }
    #[doc = "Bit 23 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_data_buf_to_en(&mut self) -> REG_RX_DATA_BUF_TO_EN_W<TIME_OUT_VALUE_SPEC> {
        REG_RX_DATA_BUF_TO_EN_W::new(self, 23)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`time_out_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time_out_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_OUT_VALUE_SPEC;
impl crate::RegisterSpec for TIME_OUT_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time_out_value::R`](R) reader structure"]
impl crate::Readable for TIME_OUT_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time_out_value::W`](W) writer structure"]
impl crate::Writable for TIME_OUT_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME_OUT_VALUE to value 0x0041_0410"]
impl crate::Resettable for TIME_OUT_VALUE_SPEC {
    const RESET_VALUE: u32 = 0x0041_0410;
}
