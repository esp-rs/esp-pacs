#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub struct R(crate::R<RD_REPEAT_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDD_SPI_DREFM` reader - SPI regulator medium voltage reference."]
pub type VDD_SPI_DREFM_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_DREFL` reader - SPI regulator low voltage reference."]
pub type VDD_SPI_DREFL_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_XPD` reader - If VDD_SPI_FORCE is 1, this value determines if the VDD_SPI regulator is powered on."]
pub type VDD_SPI_XPD_R = crate::BitReader;
#[doc = "Field `VDD_SPI_TIEH` reader - If VDD_SPI_FORCE is 1, determines VDD_SPI voltage. 0: VDD_SPI connects to 1.8 V LDO. 1: VDD_SPI connects to VDD_RTC_IO."]
pub type VDD_SPI_TIEH_R = crate::BitReader;
#[doc = "Field `VDD_SPI_FORCE` reader - Set this bit to use XPD_VDD_PSI_REG and VDD_SPI_TIEH to configure VDD_SPI LDO."]
pub type VDD_SPI_FORCE_R = crate::BitReader;
#[doc = "Field `VDD_SPI_EN_INIT` reader - Set SPI regulator to 0 to configure init\\[1:0\\]=0."]
pub type VDD_SPI_EN_INIT_R = crate::BitReader;
#[doc = "Field `VDD_SPI_ENCURLIM` reader - Set SPI regulator to 1 to enable output current limit."]
pub type VDD_SPI_ENCURLIM_R = crate::BitReader;
#[doc = "Field `VDD_SPI_DCURLIM` reader - Tunes the current limit threshold of SPI regulator when tieh=0, about 800 mA/(8+d)."]
pub type VDD_SPI_DCURLIM_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_INIT` reader - Adds resistor from LDO output to ground. 0: no resistance. 1: 6 K. 2: 4 K. 3: 2 K."]
pub type VDD_SPI_INIT_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_DCAP` reader - Prevents SPI regulator from overshoot."]
pub type VDD_SPI_DCAP_R = crate::FieldReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Selects RTC watchdog timeout threshold at startup. 0: 40,000 slow clock cycles. 1: 80,000 slow clock cycles. 2: 160,000 slow clock cycles. 3: 320,000 slow clock cycles."]
pub type WDT_DELAY_SEL_R = crate::FieldReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Enables encryption and decryption, when an SPI boot mode is set. Feature is enabled 1 or 3 bits are set in the eFuse, disabled otherwise."]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - If set, revokes use of secure boot key digest 0."]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - If set, revokes use of secure boot key digest 1."]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - If set, revokes use of secure boot key digest 2."]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0` reader - Purpose of KEY0. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_0_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1` reader - Purpose of KEY1. Refer to Table Key Purpose Values."]
pub type KEY_PURPOSE_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - SPI regulator medium voltage reference."]
    #[inline(always)]
    pub fn vdd_spi_drefm(&self) -> VDD_SPI_DREFM_R {
        VDD_SPI_DREFM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI regulator low voltage reference."]
    #[inline(always)]
    pub fn vdd_spi_drefl(&self) -> VDD_SPI_DREFL_R {
        VDD_SPI_DREFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - If VDD_SPI_FORCE is 1, this value determines if the VDD_SPI regulator is powered on."]
    #[inline(always)]
    pub fn vdd_spi_xpd(&self) -> VDD_SPI_XPD_R {
        VDD_SPI_XPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If VDD_SPI_FORCE is 1, determines VDD_SPI voltage. 0: VDD_SPI connects to 1.8 V LDO. 1: VDD_SPI connects to VDD_RTC_IO."]
    #[inline(always)]
    pub fn vdd_spi_tieh(&self) -> VDD_SPI_TIEH_R {
        VDD_SPI_TIEH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to use XPD_VDD_PSI_REG and VDD_SPI_TIEH to configure VDD_SPI LDO."]
    #[inline(always)]
    pub fn vdd_spi_force(&self) -> VDD_SPI_FORCE_R {
        VDD_SPI_FORCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set SPI regulator to 0 to configure init\\[1:0\\]=0."]
    #[inline(always)]
    pub fn vdd_spi_en_init(&self) -> VDD_SPI_EN_INIT_R {
        VDD_SPI_EN_INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set SPI regulator to 1 to enable output current limit."]
    #[inline(always)]
    pub fn vdd_spi_encurlim(&self) -> VDD_SPI_ENCURLIM_R {
        VDD_SPI_ENCURLIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Tunes the current limit threshold of SPI regulator when tieh=0, about 800 mA/(8+d)."]
    #[inline(always)]
    pub fn vdd_spi_dcurlim(&self) -> VDD_SPI_DCURLIM_R {
        VDD_SPI_DCURLIM_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Adds resistor from LDO output to ground. 0: no resistance. 1: 6 K. 2: 4 K. 3: 2 K."]
    #[inline(always)]
    pub fn vdd_spi_init(&self) -> VDD_SPI_INIT_R {
        VDD_SPI_INIT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Prevents SPI regulator from overshoot."]
    #[inline(always)]
    pub fn vdd_spi_dcap(&self) -> VDD_SPI_DCAP_R {
        VDD_SPI_DCAP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Selects RTC watchdog timeout threshold at startup. 0: 40,000 slow clock cycles. 1: 80,000 slow clock cycles. 2: 160,000 slow clock cycles. 3: 320,000 slow clock cycles."]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Enables encryption and decryption, when an SPI boot mode is set. Feature is enabled 1 or 3 bits are set in the eFuse, disabled otherwise."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - If set, revokes use of secure boot key digest 0."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - If set, revokes use of secure boot key digest 1."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If set, revokes use of secure boot key digest 2."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Purpose of KEY0. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Purpose of KEY1. Refer to Table Key Purpose Values."]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA1")
            .field(
                "vdd_spi_drefm",
                &format_args!("{}", self.vdd_spi_drefm().bits()),
            )
            .field(
                "vdd_spi_drefl",
                &format_args!("{}", self.vdd_spi_drefl().bits()),
            )
            .field("vdd_spi_xpd", &format_args!("{}", self.vdd_spi_xpd().bit()))
            .field(
                "vdd_spi_tieh",
                &format_args!("{}", self.vdd_spi_tieh().bit()),
            )
            .field(
                "vdd_spi_force",
                &format_args!("{}", self.vdd_spi_force().bit()),
            )
            .field(
                "vdd_spi_en_init",
                &format_args!("{}", self.vdd_spi_en_init().bit()),
            )
            .field(
                "vdd_spi_encurlim",
                &format_args!("{}", self.vdd_spi_encurlim().bit()),
            )
            .field(
                "vdd_spi_dcurlim",
                &format_args!("{}", self.vdd_spi_dcurlim().bits()),
            )
            .field(
                "vdd_spi_init",
                &format_args!("{}", self.vdd_spi_init().bits()),
            )
            .field(
                "vdd_spi_dcap",
                &format_args!("{}", self.vdd_spi_dcap().bits()),
            )
            .field(
                "wdt_delay_sel",
                &format_args!("{}", self.wdt_delay_sel().bits()),
            )
            .field(
                "spi_boot_crypt_cnt",
                &format_args!("{}", self.spi_boot_crypt_cnt().bits()),
            )
            .field(
                "secure_boot_key_revoke0",
                &format_args!("{}", self.secure_boot_key_revoke0().bit()),
            )
            .field(
                "secure_boot_key_revoke1",
                &format_args!("{}", self.secure_boot_key_revoke1().bit()),
            )
            .field(
                "secure_boot_key_revoke2",
                &format_args!("{}", self.secure_boot_key_revoke2().bit()),
            )
            .field(
                "key_purpose_0",
                &format_args!("{}", self.key_purpose_0().bits()),
            )
            .field(
                "key_purpose_1",
                &format_args!("{}", self.key_purpose_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 2 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data1](index.html) module"]
pub struct RD_REPEAT_DATA1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data1::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
