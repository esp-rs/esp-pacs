#[doc = "Register `RTC_FASTMEM_CONFIG` reader"]
pub type R = crate::R<RTC_FASTMEM_CONFIG_SPEC>;
#[doc = "Register `RTC_FASTMEM_CONFIG` writer"]
pub type W = crate::W<RTC_FASTMEM_CONFIG_SPEC>;
#[doc = "Field `RTC_MEM_CRC_START` reader - Set this bit to start the CRC of RTC memory."]
pub type RTC_MEM_CRC_START_R = crate::BitReader;
#[doc = "Field `RTC_MEM_CRC_START` writer - Set this bit to start the CRC of RTC memory."]
pub type RTC_MEM_CRC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_MEM_CRC_ADDR` reader - This field is used to set address of RTC memory for CRC."]
pub type RTC_MEM_CRC_ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_MEM_CRC_ADDR` writer - This field is used to set address of RTC memory for CRC."]
pub type RTC_MEM_CRC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` reader - This field is used to set length of RTC memory for CRC based on start address."]
pub type RTC_MEM_CRC_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `RTC_MEM_CRC_LEN` writer - This field is used to set length of RTC memory for CRC based on start address."]
pub type RTC_MEM_CRC_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `RTC_MEM_CRC_FINISH` reader - This bit stores the status of RTC memory CRC. High level means finished while low level means not finished."]
pub type RTC_MEM_CRC_FINISH_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Set this bit to start the CRC of RTC memory."]
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
    #[doc = "Bit 31 - This bit stores the status of RTC memory CRC. High level means finished while low level means not finished."]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 8 - Set this bit to start the CRC of RTC memory."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_start(&mut self) -> RTC_MEM_CRC_START_W<RTC_FASTMEM_CONFIG_SPEC> {
        RTC_MEM_CRC_START_W::new(self, 8)
    }
    #[doc = "Bits 9:19 - This field is used to set address of RTC memory for CRC."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_addr(&mut self) -> RTC_MEM_CRC_ADDR_W<RTC_FASTMEM_CONFIG_SPEC> {
        RTC_MEM_CRC_ADDR_W::new(self, 9)
    }
    #[doc = "Bits 20:30 - This field is used to set length of RTC memory for CRC based on start address."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mem_crc_len(&mut self) -> RTC_MEM_CRC_LEN_W<RTC_FASTMEM_CONFIG_SPEC> {
        RTC_MEM_CRC_LEN_W::new(self, 20)
    }
}
#[doc = "RTC fast memory configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc_fastmem_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_fastmem_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTC_FASTMEM_CONFIG_SPEC;
impl crate::RegisterSpec for RTC_FASTMEM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_fastmem_config::R`](R) reader structure"]
impl crate::Readable for RTC_FASTMEM_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtc_fastmem_config::W`](W) writer structure"]
impl crate::Writable for RTC_FASTMEM_CONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_FASTMEM_CONFIG to value 0x7ff0_0000"]
impl crate::Resettable for RTC_FASTMEM_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x7ff0_0000;
}
