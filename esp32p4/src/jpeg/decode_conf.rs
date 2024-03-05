#[doc = "Register `DECODE_CONF` reader"]
pub type R = crate::R<DECODE_CONF_SPEC>;
#[doc = "Register `DECODE_CONF` writer"]
pub type W = crate::W<DECODE_CONF_SPEC>;
#[doc = "Field `RESTART_INTERVAL` reader - configure restart interval in DRI marker when decode"]
pub type RESTART_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `RESTART_INTERVAL` writer - configure restart interval in DRI marker when decode"]
pub type RESTART_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COMPONENT_NUM` reader - configure number of components in frame when decode"]
pub type COMPONENT_NUM_R = crate::FieldReader;
#[doc = "Field `COMPONENT_NUM` writer - configure number of components in frame when decode"]
pub type COMPONENT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SW_DHT_EN` reader - software decode dht table enable"]
pub type SW_DHT_EN_R = crate::BitReader;
#[doc = "Field `SOS_CHECK_BYTE_NUM` reader - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
pub type SOS_CHECK_BYTE_NUM_R = crate::FieldReader;
#[doc = "Field `SOS_CHECK_BYTE_NUM` writer - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
pub type SOS_CHECK_BYTE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RST_CHECK_BYTE_NUM` reader - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
pub type RST_CHECK_BYTE_NUM_R = crate::FieldReader;
#[doc = "Field `RST_CHECK_BYTE_NUM` writer - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
pub type RST_CHECK_BYTE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MULTI_SCAN_ERR_CHECK` reader - reserved for decoder"]
pub type MULTI_SCAN_ERR_CHECK_R = crate::BitReader;
#[doc = "Field `MULTI_SCAN_ERR_CHECK` writer - reserved for decoder"]
pub type MULTI_SCAN_ERR_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEZIGZAG_READY_CTL` reader - reserved for decoder"]
pub type DEZIGZAG_READY_CTL_R = crate::BitReader;
#[doc = "Field `DEZIGZAG_READY_CTL` writer - reserved for decoder"]
pub type DEZIGZAG_READY_CTL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - configure restart interval in DRI marker when decode"]
    #[inline(always)]
    pub fn restart_interval(&self) -> RESTART_INTERVAL_R {
        RESTART_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - configure number of components in frame when decode"]
    #[inline(always)]
    pub fn component_num(&self) -> COMPONENT_NUM_R {
        COMPONENT_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - software decode dht table enable"]
    #[inline(always)]
    pub fn sw_dht_en(&self) -> SW_DHT_EN_R {
        SW_DHT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
    #[inline(always)]
    pub fn sos_check_byte_num(&self) -> SOS_CHECK_BYTE_NUM_R {
        SOS_CHECK_BYTE_NUM_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
    #[inline(always)]
    pub fn rst_check_byte_num(&self) -> RST_CHECK_BYTE_NUM_R {
        RST_CHECK_BYTE_NUM_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - reserved for decoder"]
    #[inline(always)]
    pub fn multi_scan_err_check(&self) -> MULTI_SCAN_ERR_CHECK_R {
        MULTI_SCAN_ERR_CHECK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - reserved for decoder"]
    #[inline(always)]
    pub fn dezigzag_ready_ctl(&self) -> DEZIGZAG_READY_CTL_R {
        DEZIGZAG_READY_CTL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODE_CONF")
            .field(
                "restart_interval",
                &format_args!("{}", self.restart_interval().bits()),
            )
            .field(
                "component_num",
                &format_args!("{}", self.component_num().bits()),
            )
            .field("sw_dht_en", &format_args!("{}", self.sw_dht_en().bit()))
            .field(
                "sos_check_byte_num",
                &format_args!("{}", self.sos_check_byte_num().bits()),
            )
            .field(
                "rst_check_byte_num",
                &format_args!("{}", self.rst_check_byte_num().bits()),
            )
            .field(
                "multi_scan_err_check",
                &format_args!("{}", self.multi_scan_err_check().bit()),
            )
            .field(
                "dezigzag_ready_ctl",
                &format_args!("{}", self.dezigzag_ready_ctl().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DECODE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - configure restart interval in DRI marker when decode"]
    #[inline(always)]
    #[must_use]
    pub fn restart_interval(&mut self) -> RESTART_INTERVAL_W<DECODE_CONF_SPEC> {
        RESTART_INTERVAL_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - configure number of components in frame when decode"]
    #[inline(always)]
    #[must_use]
    pub fn component_num(&mut self) -> COMPONENT_NUM_W<DECODE_CONF_SPEC> {
        COMPONENT_NUM_W::new(self, 16)
    }
    #[doc = "Bits 25:26 - Configure the byte number to check next sos marker in the multi-scan picture after one scan is decoded down. The real check number is reg_sos_check_byte_num+1"]
    #[inline(always)]
    #[must_use]
    pub fn sos_check_byte_num(&mut self) -> SOS_CHECK_BYTE_NUM_W<DECODE_CONF_SPEC> {
        SOS_CHECK_BYTE_NUM_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - Configure the byte number to check next rst marker after one rst interval is decoded down. The real check number is reg_rst_check_byte_num+1"]
    #[inline(always)]
    #[must_use]
    pub fn rst_check_byte_num(&mut self) -> RST_CHECK_BYTE_NUM_W<DECODE_CONF_SPEC> {
        RST_CHECK_BYTE_NUM_W::new(self, 27)
    }
    #[doc = "Bit 29 - reserved for decoder"]
    #[inline(always)]
    #[must_use]
    pub fn multi_scan_err_check(&mut self) -> MULTI_SCAN_ERR_CHECK_W<DECODE_CONF_SPEC> {
        MULTI_SCAN_ERR_CHECK_W::new(self, 29)
    }
    #[doc = "Bit 30 - reserved for decoder"]
    #[inline(always)]
    #[must_use]
    pub fn dezigzag_ready_ctl(&mut self) -> DEZIGZAG_READY_CTL_W<DECODE_CONF_SPEC> {
        DEZIGZAG_READY_CTL_W::new(self, 30)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`decode_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`decode_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DECODE_CONF_SPEC;
impl crate::RegisterSpec for DECODE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decode_conf::R`](R) reader structure"]
impl crate::Readable for DECODE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`decode_conf::W`](W) writer structure"]
impl crate::Writable for DECODE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DECODE_CONF to value 0x5f03_0000"]
impl crate::Resettable for DECODE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x5f03_0000;
}
