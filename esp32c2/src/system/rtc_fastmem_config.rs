#[doc = "Register `RTC_FASTMEM_CONFIG` reader"]
pub struct R(crate::R<RTC_FASTMEM_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_FASTMEM_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_FASTMEM_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_FASTMEM_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_FASTMEM_CONFIG` writer"]
pub struct W(crate::W<RTC_FASTMEM_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_FASTMEM_CONFIG_SPEC>;
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
impl From<crate::W<RTC_FASTMEM_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_FASTMEM_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_MEM_CRC_START` reader - Set 1 to start the CRC of RTC memory"]
pub type RTC_MEM_CRC_START_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_START` writer - Set 1 to start the CRC of RTC memory"]
pub type RTC_MEM_CRC_START_W<'a, const O: u8> = crate::BitWriter<'a, RTC_FASTMEM_CONFIG_SPEC, O>;
#[doc = "Field `RTC_MEM_CRC_ADDR` reader - This field is used to set address of RTC memory for CRC."]
pub type RTC_MEM_CRC_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_MEM_CRC_ADDR` writer - This field is used to set address of RTC memory for CRC."]
pub type RTC_MEM_CRC_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTC_FASTMEM_CONFIG_SPEC, 11, O, u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` reader - This field is used to set length of RTC memory for CRC based on start address."]
pub type RTC_MEM_CRC_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` writer - This field is used to set length of RTC memory for CRC based on start address."]
pub type RTC_MEM_CRC_LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTC_FASTMEM_CONFIG_SPEC, 11, O, u16>;
#[doc = "Field `RTC_MEM_CRC_FINISH` reader - This bit stores the status of RTC memory CRC.1 means finished."]
pub type RTC_MEM_CRC_FINISH_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Set 1 to start the CRC of RTC memory"]
    #[inline(always)]
    pub fn rtc_mem_crc_start(&self) -> RTC_MEM_CRC_START_R {
        RTC_MEM_CRC_START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:19 - This field is used to set address of RTC memory for CRC."]
    #[inline(always)]
    pub fn rtc_mem_crc_addr(&self) -> RTC_MEM_CRC_ADDR_R {
        RTC_MEM_CRC_ADDR_R::new(((self.bits >> 9) & 0x07ff) as u16)
    }
    #[doc = "Bits 20:30 - This field is used to set length of RTC memory for CRC based on start address."]
    #[inline(always)]
    pub fn rtc_mem_crc_len(&self) -> RTC_MEM_CRC_LEN_R {
        RTC_MEM_CRC_LEN_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - This bit stores the status of RTC memory CRC.1 means finished."]
    #[inline(always)]
    pub fn rtc_mem_crc_finish(&self) -> RTC_MEM_CRC_FINISH_R {
        RTC_MEM_CRC_FINISH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_FASTMEM_CONFIG")
            .field(
                "rtc_mem_crc_start",
                &format_args!("{}", self.rtc_mem_crc_start().bit()),
            )
            .field(
                "rtc_mem_crc_addr",
                &format_args!("{}", self.rtc_mem_crc_addr().bits()),
            )
            .field(
                "rtc_mem_crc_len",
                &format_args!("{}", self.rtc_mem_crc_len().bits()),
            )
            .field(
                "rtc_mem_crc_finish",
                &format_args!("{}", self.rtc_mem_crc_finish().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_FASTMEM_CONFIG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8 - Set 1 to start the CRC of RTC memory"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_start(&mut self) -> RTC_MEM_CRC_START_W<8> {
        RTC_MEM_CRC_START_W::new(self)
    }
    #[doc = "Bits 9:19 - This field is used to set address of RTC memory for CRC."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_addr(&mut self) -> RTC_MEM_CRC_ADDR_W<9> {
        RTC_MEM_CRC_ADDR_W::new(self)
    }
    #[doc = "Bits 20:30 - This field is used to set length of RTC memory for CRC based on start address."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_len(&mut self) -> RTC_MEM_CRC_LEN_W<20> {
        RTC_MEM_CRC_LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fast memory config register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_fastmem_config](index.html) module"]
pub struct RTC_FASTMEM_CONFIG_SPEC;
impl crate::RegisterSpec for RTC_FASTMEM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_fastmem_config::R](R) reader structure"]
impl crate::Readable for RTC_FASTMEM_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_fastmem_config::W](W) writer structure"]
impl crate::Writable for RTC_FASTMEM_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_FASTMEM_CONFIG to value 0x7ff0_0000"]
impl crate::Resettable for RTC_FASTMEM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x7ff0_0000;
}
