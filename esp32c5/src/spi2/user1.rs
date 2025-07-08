#[doc = "Register `USER1` reader"]
pub type R = crate::R<USER1_SPEC>;
#[doc = "Register `USER1` writer"]
pub type W = crate::W<USER1_SPEC>;
#[doc = "Field `USR_DUMMY_CYCLELEN` reader - Configures the length of DUMMY state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is (the expected cycle number - 1). Can be configured in CONF state."]
pub type USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `USR_DUMMY_CYCLELEN` writer - Configures the length of DUMMY state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is (the expected cycle number - 1). Can be configured in CONF state."]
pub type USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MST_WFULL_ERR_END_EN` reader - Configures whether or not to end the SPI transfer when SPI RX AFIFO wfull error occurs in master full-/half-duplex transfers. \\\\ 0: Not end \\\\ 1: End \\\\"]
pub type MST_WFULL_ERR_END_EN_R = crate::BitReader;
#[doc = "Field `MST_WFULL_ERR_END_EN` writer - Configures whether or not to end the SPI transfer when SPI RX AFIFO wfull error occurs in master full-/half-duplex transfers. \\\\ 0: Not end \\\\ 1: End \\\\"]
pub type MST_WFULL_ERR_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_SETUP_TIME` reader - Configures the length of prepare (PREP) state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is equal to the expected cycles - 1. This field is used together with SPI_CS_SETUP. Can be configured in CONF state."]
pub type CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `CS_SETUP_TIME` writer - Configures the length of prepare (PREP) state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is equal to the expected cycles - 1. This field is used together with SPI_CS_SETUP. Can be configured in CONF state."]
pub type CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CS_HOLD_TIME` reader - Configures the delay cycles of CS pin. Measurement unit: SPI_CLK clock cycles. \\\\ This field is used together with SPI_CS_HOLD. Can be configured in CONF state."]
pub type CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `CS_HOLD_TIME` writer - Configures the delay cycles of CS pin. Measurement unit: SPI_CLK clock cycles. \\\\ This field is used together with SPI_CS_HOLD. Can be configured in CONF state."]
pub type CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `USR_ADDR_BITLEN` reader - Configures the bit length in address state. This value is (expected bit number - 1). Can be configured in CONF state."]
pub type USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `USR_ADDR_BITLEN` writer - Configures the bit length in address state. This value is (expected bit number - 1). Can be configured in CONF state."]
pub type USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Configures the length of DUMMY state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is (the expected cycle number - 1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&self) -> USR_DUMMY_CYCLELEN_R {
        USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Configures whether or not to end the SPI transfer when SPI RX AFIFO wfull error occurs in master full-/half-duplex transfers. \\\\ 0: Not end \\\\ 1: End \\\\"]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&self) -> MST_WFULL_ERR_END_EN_R {
        MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - Configures the length of prepare (PREP) state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is equal to the expected cycles - 1. This field is used together with SPI_CS_SETUP. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&self) -> CS_SETUP_TIME_R {
        CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Configures the delay cycles of CS pin. Measurement unit: SPI_CLK clock cycles. \\\\ This field is used together with SPI_CS_HOLD. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&self) -> CS_HOLD_TIME_R {
        CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Configures the bit length in address state. This value is (expected bit number - 1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&self) -> USR_ADDR_BITLEN_R {
        USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER1")
            .field("usr_dummy_cyclelen", &self.usr_dummy_cyclelen())
            .field("mst_wfull_err_end_en", &self.mst_wfull_err_end_en())
            .field("cs_setup_time", &self.cs_setup_time())
            .field("cs_hold_time", &self.cs_hold_time())
            .field("usr_addr_bitlen", &self.usr_addr_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the length of DUMMY state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is (the expected cycle number - 1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_dummy_cyclelen(&mut self) -> USR_DUMMY_CYCLELEN_W<USER1_SPEC> {
        USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - Configures whether or not to end the SPI transfer when SPI RX AFIFO wfull error occurs in master full-/half-duplex transfers. \\\\ 0: Not end \\\\ 1: End \\\\"]
    #[inline(always)]
    pub fn mst_wfull_err_end_en(&mut self) -> MST_WFULL_ERR_END_EN_W<USER1_SPEC> {
        MST_WFULL_ERR_END_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - Configures the length of prepare (PREP) state. Measurement unit: SPI_CLK clock cycles.\\\\ This value is equal to the expected cycles - 1. This field is used together with SPI_CS_SETUP. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_setup_time(&mut self) -> CS_SETUP_TIME_W<USER1_SPEC> {
        CS_SETUP_TIME_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - Configures the delay cycles of CS pin. Measurement unit: SPI_CLK clock cycles. \\\\ This field is used together with SPI_CS_HOLD. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_hold_time(&mut self) -> CS_HOLD_TIME_W<USER1_SPEC> {
        CS_HOLD_TIME_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - Configures the bit length in address state. This value is (expected bit number - 1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn usr_addr_bitlen(&mut self) -> USR_ADDR_BITLEN_W<USER1_SPEC> {
        USR_ADDR_BITLEN_W::new(self, 27)
    }
}
#[doc = "SPI USER control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`user1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`user1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user1::R`](R) reader structure"]
impl crate::Readable for USER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`user1::W`](W) writer structure"]
impl crate::Writable for USER1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USER1 to value 0xb841_0007"]
impl crate::Resettable for USER1_SPEC {
    const RESET_VALUE: u32 = 0xb841_0007;
}
