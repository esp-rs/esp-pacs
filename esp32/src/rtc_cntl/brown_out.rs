#[doc = "Register `BROWN_OUT` reader"]
pub struct R(crate::R<BROWN_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BROWN_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BROWN_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BROWN_OUT` writer"]
pub struct W(crate::W<BROWN_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BROWN_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BROWN_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BROWN_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_MEM_PID_CONF` reader - "]
pub type RTC_MEM_PID_CONF_R = crate::FieldReader;
#[doc = "Field `RTC_MEM_PID_CONF` writer - "]
pub type RTC_MEM_PID_CONF_W<'a, const O: u8> = crate::FieldWriter<'a, BROWN_OUT_SPEC, 8, O>;
#[doc = "Field `RTC_MEM_CRC_START` reader - "]
pub type RTC_MEM_CRC_START_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_START` writer - "]
pub type RTC_MEM_CRC_START_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `RTC_MEM_CRC_ADDR` reader - "]
pub type RTC_MEM_CRC_ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_MEM_CRC_ADDR` writer - "]
pub type RTC_MEM_CRC_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, BROWN_OUT_SPEC, 11, O, u16, u16>;
#[doc = "Field `CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub type CLOSE_FLASH_ENA_R = crate::BitReader;
#[doc = "Field `CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub type CLOSE_FLASH_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub type PD_RF_ENA_R = crate::BitReader;
#[doc = "Field `PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub type PD_RF_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `RST_WAIT` reader - brown out reset wait cycles"]
pub type RST_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RST_WAIT` writer - brown out reset wait cycles"]
pub type RST_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, BROWN_OUT_SPEC, 10, O, u16, u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` reader - "]
pub type RTC_MEM_CRC_LEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` writer - "]
pub type RTC_MEM_CRC_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, BROWN_OUT_SPEC, 11, O, u16, u16>;
#[doc = "Field `RST_ENA` reader - enable brown out reset"]
pub type RST_ENA_R = crate::BitReader;
#[doc = "Field `RST_ENA` writer - enable brown out reset"]
pub type RST_ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `DBROWN_OUT_THRES` reader - brown out threshold"]
pub type DBROWN_OUT_THRES_R = crate::FieldReader;
#[doc = "Field `DBROWN_OUT_THRES` writer - brown out threshold"]
pub type DBROWN_OUT_THRES_W<'a, const O: u8> = crate::FieldWriter<'a, BROWN_OUT_SPEC, 3, O>;
#[doc = "Field `ENA` reader - enable brown out"]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - enable brown out"]
pub type ENA_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
#[doc = "Field `DET` reader - brown out detect"]
pub type DET_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_FINISH` reader - "]
pub type RTC_MEM_CRC_FINISH_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_FINISH` writer - "]
pub type RTC_MEM_CRC_FINISH_W<'a, const O: u8> = crate::BitWriter<'a, BROWN_OUT_SPEC, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_mem_pid_conf(&self) -> RTC_MEM_PID_CONF_R {
        RTC_MEM_PID_CONF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtc_mem_crc_start(&self) -> RTC_MEM_CRC_START_R {
        RTC_MEM_CRC_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    pub fn rtc_mem_crc_addr(&self) -> RTC_MEM_CRC_ADDR_R {
        RTC_MEM_CRC_ADDR_R::new(((self.bits >> 9) & 0x07ff) as u16)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn close_flash_ena(&self) -> CLOSE_FLASH_ENA_R {
        CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn pd_rf_ena(&self) -> PD_RF_ENA_R {
        PD_RF_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn rst_wait(&self) -> RST_WAIT_R {
        RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn rtc_mem_crc_len(&self) -> RTC_MEM_CRC_LEN_R {
        RTC_MEM_CRC_LEN_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn rst_ena(&self) -> RST_ENA_R {
        RST_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    pub fn dbrown_out_thres(&self) -> DBROWN_OUT_THRES_R {
        DBROWN_OUT_THRES_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - brown out detect"]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_mem_crc_finish(&self) -> RTC_MEM_CRC_FINISH_R {
        RTC_MEM_CRC_FINISH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BROWN_OUT")
            .field(
                "rtc_mem_pid_conf",
                &format_args!("{}", self.rtc_mem_pid_conf().bits()),
            )
            .field(
                "rtc_mem_crc_start",
                &format_args!("{}", self.rtc_mem_crc_start().bit()),
            )
            .field(
                "rtc_mem_crc_addr",
                &format_args!("{}", self.rtc_mem_crc_addr().bits()),
            )
            .field(
                "close_flash_ena",
                &format_args!("{}", self.close_flash_ena().bit()),
            )
            .field("pd_rf_ena", &format_args!("{}", self.pd_rf_ena().bit()))
            .field("rst_wait", &format_args!("{}", self.rst_wait().bits()))
            .field(
                "rtc_mem_crc_len",
                &format_args!("{}", self.rtc_mem_crc_len().bits()),
            )
            .field("rst_ena", &format_args!("{}", self.rst_ena().bit()))
            .field(
                "dbrown_out_thres",
                &format_args!("{}", self.dbrown_out_thres().bits()),
            )
            .field("ena", &format_args!("{}", self.ena().bit()))
            .field("det", &format_args!("{}", self.det().bit()))
            .field(
                "rtc_mem_crc_finish",
                &format_args!("{}", self.rtc_mem_crc_finish().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BROWN_OUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_pid_conf(&mut self) -> RTC_MEM_PID_CONF_W<0> {
        RTC_MEM_PID_CONF_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_start(&mut self) -> RTC_MEM_CRC_START_W<8> {
        RTC_MEM_CRC_START_W::new(self)
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_addr(&mut self) -> RTC_MEM_CRC_ADDR_W<9> {
        RTC_MEM_CRC_ADDR_W::new(self)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    #[must_use]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W<14> {
        CLOSE_FLASH_ENA_W::new(self)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    #[must_use]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W<15> {
        PD_RF_ENA_W::new(self)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    #[must_use]
    pub fn rst_wait(&mut self) -> RST_WAIT_W<16> {
        RST_WAIT_W::new(self)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_len(&mut self) -> RTC_MEM_CRC_LEN_W<20> {
        RTC_MEM_CRC_LEN_W::new(self)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ena(&mut self) -> RST_ENA_W<26> {
        RST_ENA_W::new(self)
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    #[must_use]
    pub fn dbrown_out_thres(&mut self) -> DBROWN_OUT_THRES_W<27> {
        DBROWN_OUT_THRES_W::new(self)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<30> {
        ENA_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_finish(&mut self) -> RTC_MEM_CRC_FINISH_W<31> {
        RTC_MEM_CRC_FINISH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brown_out](index.html) module"]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brown_out::R](R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brown_out::W](W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x13ff_0000"]
impl crate::Resettable for BROWN_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x13ff_0000;
}
