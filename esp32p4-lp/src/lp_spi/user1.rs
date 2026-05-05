#[doc = "Register `USER1` reader"]
pub type R = crate::R<USER1_SPEC>;
#[doc = "Register `USER1` writer"]
pub type W = crate::W<USER1_SPEC>;
#[doc = "Field `LP_REG_USR_DUMMY_CYCLELEN` reader - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type LP_REG_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `LP_REG_USR_DUMMY_CYCLELEN` writer - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
pub type LP_REG_USR_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_REG_MST_WFULL_ERR_END_EN` reader - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type LP_REG_MST_WFULL_ERR_END_EN_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_WFULL_ERR_END_EN` writer - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
pub type LP_REG_MST_WFULL_ERR_END_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_CS_SETUP_TIME` reader - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type LP_REG_CS_SETUP_TIME_R = crate::FieldReader;
#[doc = "Field `LP_REG_CS_SETUP_TIME` writer - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
pub type LP_REG_CS_SETUP_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LP_REG_CS_HOLD_TIME` reader - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type LP_REG_CS_HOLD_TIME_R = crate::FieldReader;
#[doc = "Field `LP_REG_CS_HOLD_TIME` writer - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
pub type LP_REG_CS_HOLD_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LP_REG_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type LP_REG_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `LP_REG_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
pub type LP_REG_USR_ADDR_BITLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_dummy_cyclelen(&self) -> LP_REG_USR_DUMMY_CYCLELEN_R {
        LP_REG_USR_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn lp_reg_mst_wfull_err_end_en(&self) -> LP_REG_MST_WFULL_ERR_END_EN_R {
        LP_REG_MST_WFULL_ERR_END_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_setup_time(&self) -> LP_REG_CS_SETUP_TIME_R {
        LP_REG_CS_SETUP_TIME_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_hold_time(&self) -> LP_REG_CS_HOLD_TIME_R {
        LP_REG_CS_HOLD_TIME_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_addr_bitlen(&self) -> LP_REG_USR_ADDR_BITLEN_R {
        LP_REG_USR_ADDR_BITLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER1")
            .field(
                "lp_reg_usr_dummy_cyclelen",
                &self.lp_reg_usr_dummy_cyclelen(),
            )
            .field(
                "lp_reg_mst_wfull_err_end_en",
                &self.lp_reg_mst_wfull_err_end_en(),
            )
            .field("lp_reg_cs_setup_time", &self.lp_reg_cs_setup_time())
            .field("lp_reg_cs_hold_time", &self.lp_reg_cs_hold_time())
            .field("lp_reg_usr_addr_bitlen", &self.lp_reg_usr_addr_bitlen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The length in spi_clk cycles of dummy phase. The register value shall be (cycle_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_dummy_cyclelen(&mut self) -> LP_REG_USR_DUMMY_CYCLELEN_W<'_, USER1_SPEC> {
        LP_REG_USR_DUMMY_CYCLELEN_W::new(self, 0)
    }
    #[doc = "Bit 16 - 1: SPI transfer is ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode. 0: SPI transfer is not ended when SPI RX AFIFO wfull error is valid in GP-SPI master FD/HD-mode."]
    #[inline(always)]
    pub fn lp_reg_mst_wfull_err_end_en(&mut self) -> LP_REG_MST_WFULL_ERR_END_EN_W<'_, USER1_SPEC> {
        LP_REG_MST_WFULL_ERR_END_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:21 - (cycles+1) of prepare phase by spi clock this bits are combined with spi_cs_setup bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_setup_time(&mut self) -> LP_REG_CS_SETUP_TIME_W<'_, USER1_SPEC> {
        LP_REG_CS_SETUP_TIME_W::new(self, 17)
    }
    #[doc = "Bits 22:26 - delay cycles of cs pin by spi clock this bits are combined with spi_cs_hold bit. Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_cs_hold_time(&mut self) -> LP_REG_CS_HOLD_TIME_W<'_, USER1_SPEC> {
        LP_REG_CS_HOLD_TIME_W::new(self, 22)
    }
    #[doc = "Bits 27:31 - The length in bits of address phase. The register value shall be (bit_num-1). Can be configured in CONF state."]
    #[inline(always)]
    pub fn lp_reg_usr_addr_bitlen(&mut self) -> LP_REG_USR_ADDR_BITLEN_W<'_, USER1_SPEC> {
        LP_REG_USR_ADDR_BITLEN_W::new(self, 27)
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
#[doc = "`reset()` method sets USER1 to value 0"]
impl crate::Resettable for USER1_SPEC {}
