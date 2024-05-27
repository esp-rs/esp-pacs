#[doc = "Register `BROWN_OUT` reader"]
pub type R = crate::R<BROWN_OUT_SPEC>;
#[doc = "Register `BROWN_OUT` writer"]
pub type W = crate::W<BROWN_OUT_SPEC>;
#[doc = "Field `RTC_MEM_PID_CONF` reader - "]
pub type RTC_MEM_PID_CONF_R = crate::FieldReader;
#[doc = "Field `RTC_MEM_PID_CONF` writer - "]
pub type RTC_MEM_PID_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTC_MEM_CRC_START` reader - "]
pub type RTC_MEM_CRC_START_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_START` writer - "]
pub type RTC_MEM_CRC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_MEM_CRC_ADDR` reader - "]
pub type RTC_MEM_CRC_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_MEM_CRC_ADDR` writer - "]
pub type RTC_MEM_CRC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CLOSE_FLASH_ENA` reader - enable close flash when brown out happens"]
pub type CLOSE_FLASH_ENA_R = crate::BitReader;
#[doc = "Field `CLOSE_FLASH_ENA` writer - enable close flash when brown out happens"]
pub type CLOSE_FLASH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_RF_ENA` reader - enable power down RF when brown out happens"]
pub type PD_RF_ENA_R = crate::BitReader;
#[doc = "Field `PD_RF_ENA` writer - enable power down RF when brown out happens"]
pub type PD_RF_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_WAIT` reader - brown out reset wait cycles"]
pub type RST_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `RST_WAIT` writer - brown out reset wait cycles"]
pub type RST_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` reader - "]
pub type RTC_MEM_CRC_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` writer - "]
pub type RTC_MEM_CRC_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RST_ENA` reader - enable brown out reset"]
pub type RST_ENA_R = crate::BitReader;
#[doc = "Field `RST_ENA` writer - enable brown out reset"]
pub type RST_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBROWN_OUT_THRES` reader - brown out threshold"]
pub type DBROWN_OUT_THRES_R = crate::FieldReader;
#[doc = "Field `DBROWN_OUT_THRES` writer - brown out threshold"]
pub type DBROWN_OUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENA` reader - enable brown out"]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - enable brown out"]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DET` reader - brown out detect"]
pub type DET_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_FINISH` reader - "]
pub type RTC_MEM_CRC_FINISH_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_FINISH` writer - "]
pub type RTC_MEM_CRC_FINISH_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("rtc_mem_pid_conf", &self.rtc_mem_pid_conf())
            .field("rtc_mem_crc_start", &self.rtc_mem_crc_start())
            .field("rtc_mem_crc_addr", &self.rtc_mem_crc_addr())
            .field("close_flash_ena", &self.close_flash_ena())
            .field("pd_rf_ena", &self.pd_rf_ena())
            .field("rst_wait", &self.rst_wait())
            .field("rtc_mem_crc_len", &self.rtc_mem_crc_len())
            .field("rst_ena", &self.rst_ena())
            .field("dbrown_out_thres", &self.dbrown_out_thres())
            .field("ena", &self.ena())
            .field("det", &self.det())
            .field("rtc_mem_crc_finish", &self.rtc_mem_crc_finish())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_pid_conf(&mut self) -> RTC_MEM_PID_CONF_W<BROWN_OUT_SPEC> {
        RTC_MEM_PID_CONF_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_start(&mut self) -> RTC_MEM_CRC_START_W<BROWN_OUT_SPEC> {
        RTC_MEM_CRC_START_W::new(self, 8)
    }
    #[doc = "Bits 9:19"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_addr(&mut self) -> RTC_MEM_CRC_ADDR_W<BROWN_OUT_SPEC> {
        RTC_MEM_CRC_ADDR_W::new(self, 9)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    #[must_use]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W<BROWN_OUT_SPEC> {
        CLOSE_FLASH_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    #[must_use]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W<BROWN_OUT_SPEC> {
        PD_RF_ENA_W::new(self, 15)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    #[must_use]
    pub fn rst_wait(&mut self) -> RST_WAIT_W<BROWN_OUT_SPEC> {
        RST_WAIT_W::new(self, 16)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_len(&mut self) -> RTC_MEM_CRC_LEN_W<BROWN_OUT_SPEC> {
        RTC_MEM_CRC_LEN_W::new(self, 20)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst_ena(&mut self) -> RST_ENA_W<BROWN_OUT_SPEC> {
        RST_ENA_W::new(self, 26)
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    #[must_use]
    pub fn dbrown_out_thres(&mut self) -> DBROWN_OUT_THRES_W<BROWN_OUT_SPEC> {
        DBROWN_OUT_THRES_W::new(self, 27)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<BROWN_OUT_SPEC> {
        ENA_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_finish(&mut self) -> RTC_MEM_CRC_FINISH_W<BROWN_OUT_SPEC> {
        RTC_MEM_CRC_FINISH_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brown_out::R`](R) reader structure"]
impl crate::Readable for BROWN_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brown_out::W`](W) writer structure"]
impl crate::Writable for BROWN_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BROWN_OUT to value 0x13ff_0000"]
impl crate::Resettable for BROWN_OUT_SPEC {
    const RESET_VALUE: u32 = 0x13ff_0000;
}
