///Register `CH%s_TX_CONF0` reader
pub type R = crate::R<CH_TX_CONF0_SPEC>;
///Register `CH%s_TX_CONF0` writer
pub type W = crate::W<CH_TX_CONF0_SPEC>;
///Field `TX_START` writer - reg_tx_start_ch0.
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_RD_RST` writer - reg_mem_rd_rst_ch0.
pub type MEM_RD_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APB_MEM_RST` writer - reg_apb_mem_rst_ch0.
pub type APB_MEM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CONTI_MODE` reader - reg_tx_conti_mode_ch0.
pub type TX_CONTI_MODE_R = crate::BitReader;
///Field `TX_CONTI_MODE` writer - reg_tx_conti_mode_ch0.
pub type TX_CONTI_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_TX_WRAP_EN` reader - reg_mem_tx_wrap_en_ch0.
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
///Field `MEM_TX_WRAP_EN` writer - reg_mem_tx_wrap_en_ch0.
pub type MEM_TX_WRAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLE_OUT_LV` reader - reg_idle_out_lv_ch0.
pub type IDLE_OUT_LV_R = crate::BitReader;
///Field `IDLE_OUT_LV` writer - reg_idle_out_lv_ch0.
pub type IDLE_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDLE_OUT_EN` reader - reg_idle_out_en_ch0.
pub type IDLE_OUT_EN_R = crate::BitReader;
///Field `IDLE_OUT_EN` writer - reg_idle_out_en_ch0.
pub type IDLE_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_STOP` reader - reg_tx_stop_ch0.
pub type TX_STOP_R = crate::BitReader;
///Field `TX_STOP` writer - reg_tx_stop_ch0.
pub type TX_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIV_CNT` reader - reg_div_cnt_ch0.
pub type DIV_CNT_R = crate::FieldReader;
///Field `DIV_CNT` writer - reg_div_cnt_ch0.
pub type DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `MEM_SIZE` reader - reg_mem_size_ch0.
pub type MEM_SIZE_R = crate::FieldReader;
///Field `MEM_SIZE` writer - reg_mem_size_ch0.
pub type MEM_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CARRIER_EFF_EN` reader - reg_carrier_eff_en_ch0.
pub type CARRIER_EFF_EN_R = crate::BitReader;
///Field `CARRIER_EFF_EN` writer - reg_carrier_eff_en_ch0.
pub type CARRIER_EFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARRIER_EN` reader - reg_carrier_en_ch0.
pub type CARRIER_EN_R = crate::BitReader;
///Field `CARRIER_EN` writer - reg_carrier_en_ch0.
pub type CARRIER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CARRIER_OUT_LV` reader - reg_carrier_out_lv_ch0.
pub type CARRIER_OUT_LV_R = crate::BitReader;
///Field `CARRIER_OUT_LV` writer - reg_carrier_out_lv_ch0.
pub type CARRIER_OUT_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AFIFO_RST` writer - reg_afifo_rst_ch0.
pub type AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONF_UPDATE` writer - reg_reg_conf_update_ch0.
pub type CONF_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - reg_tx_conti_mode_ch0.
    #[inline(always)]
    pub fn tx_conti_mode(&self) -> TX_CONTI_MODE_R {
        TX_CONTI_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - reg_mem_tx_wrap_en_ch0.
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - reg_idle_out_lv_ch0.
    #[inline(always)]
    pub fn idle_out_lv(&self) -> IDLE_OUT_LV_R {
        IDLE_OUT_LV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - reg_idle_out_en_ch0.
    #[inline(always)]
    pub fn idle_out_en(&self) -> IDLE_OUT_EN_R {
        IDLE_OUT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - reg_tx_stop_ch0.
    #[inline(always)]
    pub fn tx_stop(&self) -> TX_STOP_R {
        TX_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - reg_div_cnt_ch0.
    #[inline(always)]
    pub fn div_cnt(&self) -> DIV_CNT_R {
        DIV_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:18 - reg_mem_size_ch0.
    #[inline(always)]
    pub fn mem_size(&self) -> MEM_SIZE_R {
        MEM_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - reg_carrier_eff_en_ch0.
    #[inline(always)]
    pub fn carrier_eff_en(&self) -> CARRIER_EFF_EN_R {
        CARRIER_EFF_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - reg_carrier_en_ch0.
    #[inline(always)]
    pub fn carrier_en(&self) -> CARRIER_EN_R {
        CARRIER_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - reg_carrier_out_lv_ch0.
    #[inline(always)]
    pub fn carrier_out_lv(&self) -> CARRIER_OUT_LV_R {
        CARRIER_OUT_LV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_CONF0")
            .field("tx_conti_mode", &self.tx_conti_mode())
            .field("mem_tx_wrap_en", &self.mem_tx_wrap_en())
            .field("idle_out_lv", &self.idle_out_lv())
            .field("idle_out_en", &self.idle_out_en())
            .field("tx_stop", &self.tx_stop())
            .field("div_cnt", &self.div_cnt())
            .field("mem_size", &self.mem_size())
            .field("carrier_eff_en", &self.carrier_eff_en())
            .field("carrier_en", &self.carrier_en())
            .field("carrier_out_lv", &self.carrier_out_lv())
            .finish()
    }
}
impl W {
    ///Bit 0 - reg_tx_start_ch0.
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<CH_TX_CONF0_SPEC> {
        TX_START_W::new(self, 0)
    }
    ///Bit 1 - reg_mem_rd_rst_ch0.
    #[inline(always)]
    #[must_use]
    pub fn mem_rd_rst(&mut self) -> MEM_RD_RST_W<CH_TX_CONF0_SPEC> {
        MEM_RD_RST_W::new(self, 1)
    }
    ///Bit 2 - reg_apb_mem_rst_ch0.
    #[inline(always)]
    #[must_use]
    pub fn apb_mem_rst(&mut self) -> APB_MEM_RST_W<CH_TX_CONF0_SPEC> {
        APB_MEM_RST_W::new(self, 2)
    }
    ///Bit 3 - reg_tx_conti_mode_ch0.
    #[inline(always)]
    #[must_use]
    pub fn tx_conti_mode(&mut self) -> TX_CONTI_MODE_W<CH_TX_CONF0_SPEC> {
        TX_CONTI_MODE_W::new(self, 3)
    }
    ///Bit 4 - reg_mem_tx_wrap_en_ch0.
    #[inline(always)]
    #[must_use]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<CH_TX_CONF0_SPEC> {
        MEM_TX_WRAP_EN_W::new(self, 4)
    }
    ///Bit 5 - reg_idle_out_lv_ch0.
    #[inline(always)]
    #[must_use]
    pub fn idle_out_lv(&mut self) -> IDLE_OUT_LV_W<CH_TX_CONF0_SPEC> {
        IDLE_OUT_LV_W::new(self, 5)
    }
    ///Bit 6 - reg_idle_out_en_ch0.
    #[inline(always)]
    #[must_use]
    pub fn idle_out_en(&mut self) -> IDLE_OUT_EN_W<CH_TX_CONF0_SPEC> {
        IDLE_OUT_EN_W::new(self, 6)
    }
    ///Bit 7 - reg_tx_stop_ch0.
    #[inline(always)]
    #[must_use]
    pub fn tx_stop(&mut self) -> TX_STOP_W<CH_TX_CONF0_SPEC> {
        TX_STOP_W::new(self, 7)
    }
    ///Bits 8:15 - reg_div_cnt_ch0.
    #[inline(always)]
    #[must_use]
    pub fn div_cnt(&mut self) -> DIV_CNT_W<CH_TX_CONF0_SPEC> {
        DIV_CNT_W::new(self, 8)
    }
    ///Bits 16:18 - reg_mem_size_ch0.
    #[inline(always)]
    #[must_use]
    pub fn mem_size(&mut self) -> MEM_SIZE_W<CH_TX_CONF0_SPEC> {
        MEM_SIZE_W::new(self, 16)
    }
    ///Bit 20 - reg_carrier_eff_en_ch0.
    #[inline(always)]
    #[must_use]
    pub fn carrier_eff_en(&mut self) -> CARRIER_EFF_EN_W<CH_TX_CONF0_SPEC> {
        CARRIER_EFF_EN_W::new(self, 20)
    }
    ///Bit 21 - reg_carrier_en_ch0.
    #[inline(always)]
    #[must_use]
    pub fn carrier_en(&mut self) -> CARRIER_EN_W<CH_TX_CONF0_SPEC> {
        CARRIER_EN_W::new(self, 21)
    }
    ///Bit 22 - reg_carrier_out_lv_ch0.
    #[inline(always)]
    #[must_use]
    pub fn carrier_out_lv(&mut self) -> CARRIER_OUT_LV_W<CH_TX_CONF0_SPEC> {
        CARRIER_OUT_LV_W::new(self, 22)
    }
    ///Bit 23 - reg_afifo_rst_ch0.
    #[inline(always)]
    #[must_use]
    pub fn afifo_rst(&mut self) -> AFIFO_RST_W<CH_TX_CONF0_SPEC> {
        AFIFO_RST_W::new(self, 23)
    }
    ///Bit 24 - reg_reg_conf_update_ch0.
    #[inline(always)]
    #[must_use]
    pub fn conf_update(&mut self) -> CONF_UPDATE_W<CH_TX_CONF0_SPEC> {
        CONF_UPDATE_W::new(self, 24)
    }
}
/**RMT_CH%sCONF%s_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_tx_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_tx_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_TX_CONF0_SPEC;
impl crate::RegisterSpec for CH_TX_CONF0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ch_tx_conf0::R`](R) reader structure
impl crate::Readable for CH_TX_CONF0_SPEC {}
///`write(|w| ..)` method takes [`ch_tx_conf0::W`](W) writer structure
impl crate::Writable for CH_TX_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH%s_TX_CONF0 to value 0x0071_0200
impl crate::Resettable for CH_TX_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x0071_0200;
}
