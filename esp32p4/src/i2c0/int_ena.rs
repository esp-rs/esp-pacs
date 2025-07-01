#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RXFIFO_WM` reader - Write 1 to enable I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_R = crate::BitReader;
#[doc = "Field `RXFIFO_WM` writer - Write 1 to enable I2C_RXFIFO_WM_INT interrupt."]
pub type RXFIFO_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_WM` reader - Write 1 to enable I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_R = crate::BitReader;
#[doc = "Field `TXFIFO_WM` writer - Write 1 to enable I2C_TXFIFO_WM_INT interrupt."]
pub type TXFIFO_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF` reader - Write 1 to enable I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF` writer - Write 1 to enable I2C_RXFIFO_OVF_INT interrupt."]
pub type RXFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_DETECT` reader - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_R = crate::BitReader;
#[doc = "Field `END_DETECT` writer - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
pub type END_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_TRANS_DONE` reader - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS_DONE` writer - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
pub type BYTE_TRANS_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBITRATION_LOST` reader - Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_R = crate::BitReader;
#[doc = "Field `ARBITRATION_LOST` writer - Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt."]
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TXFIFO_UDF` reader - Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `MST_TXFIFO_UDF` writer - Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt."]
pub type MST_TXFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_COMPLETE` reader - Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_R = crate::BitReader;
#[doc = "Field `TRANS_COMPLETE` writer - Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt."]
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME_OUT` reader - Write 1 to enable the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_R = crate::BitReader;
#[doc = "Field `TIME_OUT` writer - Write 1 to enable the I2C_TIME_OUT_INT interrupt."]
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_START` reader - Write 1 to enable the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_R = crate::BitReader;
#[doc = "Field `TRANS_START` writer - Write 1 to enable the I2C_TRANS_START_INT interrupt."]
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_OVF` reader - Write 1 to enable I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_R = crate::BitReader;
#[doc = "Field `TXFIFO_OVF` writer - Write 1 to enable I2C_TXFIFO_OVF_INT interrupt."]
pub type TXFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_UDF` reader - Write 1 to enable I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_R = crate::BitReader;
#[doc = "Field `RXFIFO_UDF` writer - Write 1 to enable I2C_RXFIFO_UDF_INT interrupt."]
pub type RXFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_ST_TO` reader - Write 1 to enable I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_R = crate::BitReader;
#[doc = "Field `SCL_ST_TO` writer - Write 1 to enable I2C_SCL_ST_TO_INT interrupt."]
pub type SCL_ST_TO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_MAIN_ST_TO` reader - Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_R = crate::BitReader;
#[doc = "Field `SCL_MAIN_ST_TO` writer - Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt."]
pub type SCL_MAIN_ST_TO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DET_START` reader - Write 1 to enable I2C_DET_START_INT interrupt."]
pub type DET_START_R = crate::BitReader;
#[doc = "Field `DET_START` writer - Write 1 to enable I2C_DET_START_INT interrupt."]
pub type DET_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_STRETCH` reader - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
pub type SLAVE_STRETCH_R = crate::BitReader;
#[doc = "Field `SLAVE_STRETCH` writer - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
pub type SLAVE_STRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENERAL_CALL` reader - Write 1 to enable I2C_GENARAL_CALL_INT interrupt."]
pub type GENERAL_CALL_R = crate::BitReader;
#[doc = "Field `GENERAL_CALL` writer - Write 1 to enable I2C_GENARAL_CALL_INT interrupt."]
pub type GENERAL_CALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_ADDR_UNMATCH` reader - Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt."]
pub type SLAVE_ADDR_UNMATCH_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_UNMATCH` writer - Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt."]
pub type SLAVE_ADDR_UNMATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&self) -> RXFIFO_WM_R {
        RXFIFO_WM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&self) -> TXFIFO_WM_R {
        TXFIFO_WM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&self) -> BYTE_TRANS_DONE_R {
        BYTE_TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&self) -> MST_TXFIFO_UDF_R {
        MST_TXFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to enable the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to enable the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write 1 to enable I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&self) -> TXFIFO_OVF_R {
        TXFIFO_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write 1 to enable I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&self) -> RXFIFO_UDF_R {
        RXFIFO_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write 1 to enable I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&self) -> SCL_ST_TO_R {
        SCL_ST_TO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SCL_MAIN_ST_TO_R {
        SCL_MAIN_ST_TO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write 1 to enable I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&self) -> DET_START_R {
        DET_START_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch(&self) -> SLAVE_STRETCH_R {
        SLAVE_STRETCH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write 1 to enable I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call(&self) -> GENERAL_CALL_R {
        GENERAL_CALL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_addr_unmatch(&self) -> SLAVE_ADDR_UNMATCH_R {
        SLAVE_ADDR_UNMATCH_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rxfifo_wm", &self.rxfifo_wm())
            .field("txfifo_wm", &self.txfifo_wm())
            .field("rxfifo_ovf", &self.rxfifo_ovf())
            .field("end_detect", &self.end_detect())
            .field("byte_trans_done", &self.byte_trans_done())
            .field("arbitration_lost", &self.arbitration_lost())
            .field("mst_txfifo_udf", &self.mst_txfifo_udf())
            .field("trans_complete", &self.trans_complete())
            .field("time_out", &self.time_out())
            .field("trans_start", &self.trans_start())
            .field("nack", &self.nack())
            .field("txfifo_ovf", &self.txfifo_ovf())
            .field("rxfifo_udf", &self.rxfifo_udf())
            .field("scl_st_to", &self.scl_st_to())
            .field("scl_main_st_to", &self.scl_main_st_to())
            .field("det_start", &self.det_start())
            .field("slave_stretch", &self.slave_stretch())
            .field("general_call", &self.general_call())
            .field("slave_addr_unmatch", &self.slave_addr_unmatch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable I2C_RXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_wm(&mut self) -> RXFIFO_WM_W<INT_ENA_SPEC> {
        RXFIFO_WM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable I2C_TXFIFO_WM_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_wm(&mut self) -> TXFIFO_WM_W<INT_ENA_SPEC> {
        TXFIFO_WM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable I2C_RXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<INT_ENA_SPEC> {
        RXFIFO_OVF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn end_detect(&mut self) -> END_DETECT_W<INT_ENA_SPEC> {
        END_DETECT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable the I2C_END_DETECT_INT interrupt."]
    #[inline(always)]
    pub fn byte_trans_done(&mut self) -> BYTE_TRANS_DONE_W<INT_ENA_SPEC> {
        BYTE_TRANS_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable the I2C_ARBITRATION_LOST_INT interrupt."]
    #[inline(always)]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_ENA_SPEC> {
        ARBITRATION_LOST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn mst_txfifo_udf(&mut self) -> MST_TXFIFO_UDF_W<INT_ENA_SPEC> {
        MST_TXFIFO_UDF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to enable the I2C_TRANS_COMPLETE_INT interrupt."]
    #[inline(always)]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_ENA_SPEC> {
        TRANS_COMPLETE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to enable the I2C_TIME_OUT_INT interrupt."]
    #[inline(always)]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_ENA_SPEC> {
        TIME_OUT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to enable the I2C_TRANS_START_INT interrupt."]
    #[inline(always)]
    pub fn trans_start(&mut self) -> TRANS_START_W<INT_ENA_SPEC> {
        TRANS_START_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<INT_ENA_SPEC> {
        NACK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to enable I2C_TXFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn txfifo_ovf(&mut self) -> TXFIFO_OVF_W<INT_ENA_SPEC> {
        TXFIFO_OVF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to enable I2C_RXFIFO_UDF_INT interrupt."]
    #[inline(always)]
    pub fn rxfifo_udf(&mut self) -> RXFIFO_UDF_W<INT_ENA_SPEC> {
        RXFIFO_UDF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to enable I2C_SCL_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_st_to(&mut self) -> SCL_ST_TO_W<INT_ENA_SPEC> {
        SCL_ST_TO_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to enable I2C_SCL_MAIN_ST_TO_INT interrupt."]
    #[inline(always)]
    pub fn scl_main_st_to(&mut self) -> SCL_MAIN_ST_TO_W<INT_ENA_SPEC> {
        SCL_MAIN_ST_TO_W::new(self, 14)
    }
    #[doc = "Bit 15 - Write 1 to enable I2C_DET_START_INT interrupt."]
    #[inline(always)]
    pub fn det_start(&mut self) -> DET_START_W<INT_ENA_SPEC> {
        DET_START_W::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to enable I2C_SLAVE_STRETCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_stretch(&mut self) -> SLAVE_STRETCH_W<INT_ENA_SPEC> {
        SLAVE_STRETCH_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to enable I2C_GENARAL_CALL_INT interrupt."]
    #[inline(always)]
    pub fn general_call(&mut self) -> GENERAL_CALL_W<INT_ENA_SPEC> {
        GENERAL_CALL_W::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to enable I2C_SLAVE_ADDR_UNMATCH_INT interrupt."]
    #[inline(always)]
    pub fn slave_addr_unmatch(&mut self) -> SLAVE_ADDR_UNMATCH_W<INT_ENA_SPEC> {
        SLAVE_ADDR_UNMATCH_W::new(self, 18)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
