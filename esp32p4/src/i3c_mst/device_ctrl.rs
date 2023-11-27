#[doc = "Register `DEVICE_CTRL` reader"]
pub type R = crate::R<DEVICE_CTRL_SPEC>;
#[doc = "Register `DEVICE_CTRL` writer"]
pub type W = crate::W<DEVICE_CTRL_SPEC>;
#[doc = "Field `REG_BA_INCLUDE` reader - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
pub type REG_BA_INCLUDE_R = crate::BitReader;
#[doc = "Field `REG_BA_INCLUDE` writer - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
pub type REG_BA_INCLUDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TRANS_START` reader - Transfer Start"]
pub type REG_TRANS_START_R = crate::BitReader;
#[doc = "Field `REG_TRANS_START` writer - Transfer Start"]
pub type REG_TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_CLK_EN` reader - NA"]
pub type REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_CLK_EN` writer - NA"]
pub type REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_IBI_RSTART_TRANS_EN` reader - NA"]
pub type REG_IBI_RSTART_TRANS_EN_R = crate::BitReader;
#[doc = "Field `REG_IBI_RSTART_TRANS_EN` writer - NA"]
pub type REG_IBI_RSTART_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_AUTO_DIS_IBI_EN` reader - NA"]
pub type REG_AUTO_DIS_IBI_EN_R = crate::BitReader;
#[doc = "Field `REG_AUTO_DIS_IBI_EN` writer - NA"]
pub type REG_AUTO_DIS_IBI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DMA_RX_EN` reader - NA"]
pub type REG_DMA_RX_EN_R = crate::BitReader;
#[doc = "Field `REG_DMA_RX_EN` writer - NA"]
pub type REG_DMA_RX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DMA_TX_EN` reader - NA"]
pub type REG_DMA_TX_EN_R = crate::BitReader;
#[doc = "Field `REG_DMA_TX_EN` writer - NA"]
pub type REG_DMA_TX_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MULTI_SLV_SINGLE_CCC_EN` reader - 0: rx high bit first, 1: rx low bit first"]
pub type REG_MULTI_SLV_SINGLE_CCC_EN_R = crate::BitReader;
#[doc = "Field `REG_MULTI_SLV_SINGLE_CCC_EN` writer - 0: rx high bit first, 1: rx low bit first"]
pub type REG_MULTI_SLV_SINGLE_CCC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RX_BIT_ORDER` reader - 0: rx low byte fist, 1: rx high byte first"]
pub type REG_RX_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `REG_RX_BIT_ORDER` writer - 0: rx low byte fist, 1: rx high byte first"]
pub type REG_RX_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_RX_BYTE_ORDER` reader - NA"]
pub type REG_RX_BYTE_ORDER_R = crate::BitReader;
#[doc = "Field `REG_RX_BYTE_ORDER` writer - NA"]
pub type REG_RX_BYTE_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SCL_PULLUP_FORCE_EN` reader - This bit is used to force scl_pullup_en"]
pub type REG_SCL_PULLUP_FORCE_EN_R = crate::BitReader;
#[doc = "Field `REG_SCL_PULLUP_FORCE_EN` writer - This bit is used to force scl_pullup_en"]
pub type REG_SCL_PULLUP_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SCL_OE_FORCE_EN` reader - This bit is used to force scl_oe"]
pub type REG_SCL_OE_FORCE_EN_R = crate::BitReader;
#[doc = "Field `REG_SCL_OE_FORCE_EN` writer - This bit is used to force scl_oe"]
pub type REG_SCL_OE_FORCE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDA_PP_RD_PULLUP_EN` reader - NA"]
pub type REG_SDA_PP_RD_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `REG_SDA_PP_RD_PULLUP_EN` writer - NA"]
pub type REG_SDA_PP_RD_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDA_RD_TBIT_HLVL_PULLUP_EN` reader - NA"]
pub type REG_SDA_RD_TBIT_HLVL_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `REG_SDA_RD_TBIT_HLVL_PULLUP_EN` writer - NA"]
pub type REG_SDA_RD_TBIT_HLVL_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDA_PP_WR_PULLUP_EN` reader - NA"]
pub type REG_SDA_PP_WR_PULLUP_EN_R = crate::BitReader;
#[doc = "Field `REG_SDA_PP_WR_PULLUP_EN` writer - NA"]
pub type REG_SDA_PP_WR_PULLUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_DATA_BYTE_CNT_UNLATCH` reader - 1: read current real-time updated value 0: read latch data byte cnt value"]
pub type REG_DATA_BYTE_CNT_UNLATCH_R = crate::BitReader;
#[doc = "Field `REG_DATA_BYTE_CNT_UNLATCH` writer - 1: read current real-time updated value 0: read latch data byte cnt value"]
pub type REG_DATA_BYTE_CNT_UNLATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MEM_CLK_FORCE_ON` reader - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
pub type REG_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `REG_MEM_CLK_FORCE_ON` writer - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
pub type REG_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
    #[inline(always)]
    pub fn reg_ba_include(&self) -> REG_BA_INCLUDE_R {
        REG_BA_INCLUDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Start"]
    #[inline(always)]
    pub fn reg_trans_start(&self) -> REG_TRANS_START_R {
        REG_TRANS_START_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_clk_en(&self) -> REG_CLK_EN_R {
        REG_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn reg_ibi_rstart_trans_en(&self) -> REG_IBI_RSTART_TRANS_EN_R {
        REG_IBI_RSTART_TRANS_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn reg_auto_dis_ibi_en(&self) -> REG_AUTO_DIS_IBI_EN_R {
        REG_AUTO_DIS_IBI_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn reg_dma_rx_en(&self) -> REG_DMA_RX_EN_R {
        REG_DMA_RX_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn reg_dma_tx_en(&self) -> REG_DMA_TX_EN_R {
        REG_DMA_TX_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: rx high bit first, 1: rx low bit first"]
    #[inline(always)]
    pub fn reg_multi_slv_single_ccc_en(&self) -> REG_MULTI_SLV_SINGLE_CCC_EN_R {
        REG_MULTI_SLV_SINGLE_CCC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: rx low byte fist, 1: rx high byte first"]
    #[inline(always)]
    pub fn reg_rx_bit_order(&self) -> REG_RX_BIT_ORDER_R {
        REG_RX_BIT_ORDER_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn reg_rx_byte_order(&self) -> REG_RX_BYTE_ORDER_R {
        REG_RX_BYTE_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit is used to force scl_pullup_en"]
    #[inline(always)]
    pub fn reg_scl_pullup_force_en(&self) -> REG_SCL_PULLUP_FORCE_EN_R {
        REG_SCL_PULLUP_FORCE_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit is used to force scl_oe"]
    #[inline(always)]
    pub fn reg_scl_oe_force_en(&self) -> REG_SCL_OE_FORCE_EN_R {
        REG_SCL_OE_FORCE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    pub fn reg_sda_pp_rd_pullup_en(&self) -> REG_SDA_PP_RD_PULLUP_EN_R {
        REG_SDA_PP_RD_PULLUP_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    pub fn reg_sda_rd_tbit_hlvl_pullup_en(&self) -> REG_SDA_RD_TBIT_HLVL_PULLUP_EN_R {
        REG_SDA_RD_TBIT_HLVL_PULLUP_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn reg_sda_pp_wr_pullup_en(&self) -> REG_SDA_PP_WR_PULLUP_EN_R {
        REG_SDA_PP_WR_PULLUP_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: read current real-time updated value 0: read latch data byte cnt value"]
    #[inline(always)]
    pub fn reg_data_byte_cnt_unlatch(&self) -> REG_DATA_BYTE_CNT_UNLATCH_R {
        REG_DATA_BYTE_CNT_UNLATCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
    #[inline(always)]
    pub fn reg_mem_clk_force_on(&self) -> REG_MEM_CLK_FORCE_ON_R {
        REG_MEM_CLK_FORCE_ON_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_CTRL")
            .field(
                "reg_ba_include",
                &format_args!("{}", self.reg_ba_include().bit()),
            )
            .field(
                "reg_trans_start",
                &format_args!("{}", self.reg_trans_start().bit()),
            )
            .field("reg_clk_en", &format_args!("{}", self.reg_clk_en().bit()))
            .field(
                "reg_ibi_rstart_trans_en",
                &format_args!("{}", self.reg_ibi_rstart_trans_en().bit()),
            )
            .field(
                "reg_auto_dis_ibi_en",
                &format_args!("{}", self.reg_auto_dis_ibi_en().bit()),
            )
            .field(
                "reg_dma_rx_en",
                &format_args!("{}", self.reg_dma_rx_en().bit()),
            )
            .field(
                "reg_dma_tx_en",
                &format_args!("{}", self.reg_dma_tx_en().bit()),
            )
            .field(
                "reg_multi_slv_single_ccc_en",
                &format_args!("{}", self.reg_multi_slv_single_ccc_en().bit()),
            )
            .field(
                "reg_rx_bit_order",
                &format_args!("{}", self.reg_rx_bit_order().bit()),
            )
            .field(
                "reg_rx_byte_order",
                &format_args!("{}", self.reg_rx_byte_order().bit()),
            )
            .field(
                "reg_scl_pullup_force_en",
                &format_args!("{}", self.reg_scl_pullup_force_en().bit()),
            )
            .field(
                "reg_scl_oe_force_en",
                &format_args!("{}", self.reg_scl_oe_force_en().bit()),
            )
            .field(
                "reg_sda_pp_rd_pullup_en",
                &format_args!("{}", self.reg_sda_pp_rd_pullup_en().bit()),
            )
            .field(
                "reg_sda_rd_tbit_hlvl_pullup_en",
                &format_args!("{}", self.reg_sda_rd_tbit_hlvl_pullup_en().bit()),
            )
            .field(
                "reg_sda_pp_wr_pullup_en",
                &format_args!("{}", self.reg_sda_pp_wr_pullup_en().bit()),
            )
            .field(
                "reg_data_byte_cnt_unlatch",
                &format_args!("{}", self.reg_data_byte_cnt_unlatch().bit()),
            )
            .field(
                "reg_mem_clk_force_on",
                &format_args!("{}", self.reg_mem_clk_force_on().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVICE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - This bit is used to include I3C broadcast address(0x7E) for private transfer.(If I3C broadcast address is not include for the private transfer, In-Band Interrupts driven from Slaves may not win address arbitration. Hence IBIs will get delayed)"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ba_include(&mut self) -> REG_BA_INCLUDE_W<DEVICE_CTRL_SPEC> {
        REG_BA_INCLUDE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Start"]
    #[inline(always)]
    #[must_use]
    pub fn reg_trans_start(&mut self) -> REG_TRANS_START_W<DEVICE_CTRL_SPEC> {
        REG_TRANS_START_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_clk_en(&mut self) -> REG_CLK_EN_W<DEVICE_CTRL_SPEC> {
        REG_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ibi_rstart_trans_en(&mut self) -> REG_IBI_RSTART_TRANS_EN_W<DEVICE_CTRL_SPEC> {
        REG_IBI_RSTART_TRANS_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_auto_dis_ibi_en(&mut self) -> REG_AUTO_DIS_IBI_EN_W<DEVICE_CTRL_SPEC> {
        REG_AUTO_DIS_IBI_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dma_rx_en(&mut self) -> REG_DMA_RX_EN_W<DEVICE_CTRL_SPEC> {
        REG_DMA_RX_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_dma_tx_en(&mut self) -> REG_DMA_TX_EN_W<DEVICE_CTRL_SPEC> {
        REG_DMA_TX_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 0: rx high bit first, 1: rx low bit first"]
    #[inline(always)]
    #[must_use]
    pub fn reg_multi_slv_single_ccc_en(
        &mut self,
    ) -> REG_MULTI_SLV_SINGLE_CCC_EN_W<DEVICE_CTRL_SPEC> {
        REG_MULTI_SLV_SINGLE_CCC_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 0: rx low byte fist, 1: rx high byte first"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_bit_order(&mut self) -> REG_RX_BIT_ORDER_W<DEVICE_CTRL_SPEC> {
        REG_RX_BIT_ORDER_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_rx_byte_order(&mut self) -> REG_RX_BYTE_ORDER_W<DEVICE_CTRL_SPEC> {
        REG_RX_BYTE_ORDER_W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is used to force scl_pullup_en"]
    #[inline(always)]
    #[must_use]
    pub fn reg_scl_pullup_force_en(&mut self) -> REG_SCL_PULLUP_FORCE_EN_W<DEVICE_CTRL_SPEC> {
        REG_SCL_PULLUP_FORCE_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is used to force scl_oe"]
    #[inline(always)]
    #[must_use]
    pub fn reg_scl_oe_force_en(&mut self) -> REG_SCL_OE_FORCE_EN_W<DEVICE_CTRL_SPEC> {
        REG_SCL_OE_FORCE_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sda_pp_rd_pullup_en(&mut self) -> REG_SDA_PP_RD_PULLUP_EN_W<DEVICE_CTRL_SPEC> {
        REG_SDA_PP_RD_PULLUP_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sda_rd_tbit_hlvl_pullup_en(
        &mut self,
    ) -> REG_SDA_RD_TBIT_HLVL_PULLUP_EN_W<DEVICE_CTRL_SPEC> {
        REG_SDA_RD_TBIT_HLVL_PULLUP_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sda_pp_wr_pullup_en(&mut self) -> REG_SDA_PP_WR_PULLUP_EN_W<DEVICE_CTRL_SPEC> {
        REG_SDA_PP_WR_PULLUP_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - 1: read current real-time updated value 0: read latch data byte cnt value"]
    #[inline(always)]
    #[must_use]
    pub fn reg_data_byte_cnt_unlatch(&mut self) -> REG_DATA_BYTE_CNT_UNLATCH_W<DEVICE_CTRL_SPEC> {
        REG_DATA_BYTE_CNT_UNLATCH_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1: dev characteristic and address table memory clk date force on . 0 : clock gating by rd/wr."]
    #[inline(always)]
    #[must_use]
    pub fn reg_mem_clk_force_on(&mut self) -> REG_MEM_CLK_FORCE_ON_W<DEVICE_CTRL_SPEC> {
        REG_MEM_CLK_FORCE_ON_W::new(self, 17)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DEVICE_CTRL register controls the transfer properties and disposition of controllers capabilities.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICE_CTRL_SPEC;
impl crate::RegisterSpec for DEVICE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_ctrl::R`](R) reader structure"]
impl crate::Readable for DEVICE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`device_ctrl::W`](W) writer structure"]
impl crate::Writable for DEVICE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVICE_CTRL to value 0x1020"]
impl crate::Resettable for DEVICE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020;
}
