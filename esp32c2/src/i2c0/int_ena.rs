///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `RXFIFO_WM` reader - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt.
pub type RXFIFO_WM_R = crate::BitReader;
///Field `RXFIFO_WM` writer - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt.
pub type RXFIFO_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFO_WM` reader - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt.
pub type TXFIFO_WM_R = crate::BitReader;
///Field `TXFIFO_WM` writer - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt.
pub type TXFIFO_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFO_OVF` reader - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt.
pub type RXFIFO_OVF_R = crate::BitReader;
///Field `RXFIFO_OVF` writer - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt.
pub type RXFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `END_DETECT` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
pub type END_DETECT_R = crate::BitReader;
///Field `END_DETECT` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
pub type END_DETECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BYTE_TRANS_DONE` reader - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
pub type BYTE_TRANS_DONE_R = crate::BitReader;
///Field `BYTE_TRANS_DONE` writer - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
pub type BYTE_TRANS_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBITRATION_LOST` reader - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt.
pub type ARBITRATION_LOST_R = crate::BitReader;
///Field `ARBITRATION_LOST` writer - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt.
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MST_TXFIFO_UDF` reader - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt.
pub type MST_TXFIFO_UDF_R = crate::BitReader;
///Field `MST_TXFIFO_UDF` writer - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt.
pub type MST_TXFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRANS_COMPLETE` reader - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt.
pub type TRANS_COMPLETE_R = crate::BitReader;
///Field `TRANS_COMPLETE` writer - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt.
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIME_OUT` reader - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt.
pub type TIME_OUT_R = crate::BitReader;
///Field `TIME_OUT` writer - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt.
pub type TIME_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRANS_START` reader - The interrupt enable bit for the I2C_TRANS_START_INT interrupt.
pub type TRANS_START_R = crate::BitReader;
///Field `TRANS_START` writer - The interrupt enable bit for the I2C_TRANS_START_INT interrupt.
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` reader - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt.
pub type NACK_R = crate::BitReader;
///Field `NACK` writer - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt.
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIFO_OVF` reader - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt.
pub type TXFIFO_OVF_R = crate::BitReader;
///Field `TXFIFO_OVF` writer - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt.
pub type TXFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFIFO_UDF` reader - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt.
pub type RXFIFO_UDF_R = crate::BitReader;
///Field `RXFIFO_UDF` writer - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt.
pub type RXFIFO_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCL_ST_TO` reader - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt.
pub type SCL_ST_TO_R = crate::BitReader;
///Field `SCL_ST_TO` writer - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt.
pub type SCL_ST_TO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCL_MAIN_ST_TO` reader - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt.
pub type SCL_MAIN_ST_TO_R = crate::BitReader;
///Field `SCL_MAIN_ST_TO` writer - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt.
pub type SCL_MAIN_ST_TO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DET_START` reader - The interrupt enable bit for I2C_DET_START_INT interrupt.
pub type DET_START_R = crate::BitReader;
///Field `DET_START` writer - The interrupt enable bit for I2C_DET_START_INT interrupt.
pub type DET_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_wm(&self) -> RXFIFO_WM_R {
        RXFIFO_WM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    pub fn txfifo_wm(&self) -> TXFIFO_WM_R {
        TXFIFO_WM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn end_detect(&self) -> END_DETECT_R {
        END_DETECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    pub fn byte_trans_done(&self) -> BYTE_TRANS_DONE_R {
        BYTE_TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    pub fn arbitration_lost(&self) -> ARBITRATION_LOST_R {
        ARBITRATION_LOST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn mst_txfifo_udf(&self) -> MST_TXFIFO_UDF_R {
        MST_TXFIFO_UDF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    pub fn trans_complete(&self) -> TRANS_COMPLETE_R {
        TRANS_COMPLETE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    pub fn time_out(&self) -> TIME_OUT_R {
        TIME_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    pub fn txfifo_ovf(&self) -> TXFIFO_OVF_R {
        TXFIFO_OVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    pub fn rxfifo_udf(&self) -> RXFIFO_UDF_R {
        RXFIFO_UDF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_st_to(&self) -> SCL_ST_TO_R {
        SCL_ST_TO_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    pub fn scl_main_st_to(&self) -> SCL_MAIN_ST_TO_R {
        SCL_MAIN_ST_TO_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt.
    #[inline(always)]
    pub fn det_start(&self) -> DET_START_R {
        DET_START_R::new(((self.bits >> 15) & 1) != 0)
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
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for I2C_RXFIFO_WM_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_wm(&mut self) -> RXFIFO_WM_W<INT_ENA_SPEC> {
        RXFIFO_WM_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for I2C_TXFIFO_WM_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn txfifo_wm(&mut self) -> TXFIFO_WM_W<INT_ENA_SPEC> {
        TXFIFO_WM_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for I2C_RXFIFO_OVF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<INT_ENA_SPEC> {
        RXFIFO_OVF_W::new(self, 2)
    }
    ///Bit 3 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn end_detect(&mut self) -> END_DETECT_W<INT_ENA_SPEC> {
        END_DETECT_W::new(self, 3)
    }
    ///Bit 4 - The interrupt enable bit for the I2C_END_DETECT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn byte_trans_done(&mut self) -> BYTE_TRANS_DONE_W<INT_ENA_SPEC> {
        BYTE_TRANS_DONE_W::new(self, 4)
    }
    ///Bit 5 - The interrupt enable bit for the I2C_ARBITRATION_LOST_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_ENA_SPEC> {
        ARBITRATION_LOST_W::new(self, 5)
    }
    ///Bit 6 - The interrupt enable bit for I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn mst_txfifo_udf(&mut self) -> MST_TXFIFO_UDF_W<INT_ENA_SPEC> {
        MST_TXFIFO_UDF_W::new(self, 6)
    }
    ///Bit 7 - The interrupt enable bit for the I2C_TRANS_COMPLETE_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_ENA_SPEC> {
        TRANS_COMPLETE_W::new(self, 7)
    }
    ///Bit 8 - The interrupt enable bit for the I2C_TIME_OUT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_ENA_SPEC> {
        TIME_OUT_W::new(self, 8)
    }
    ///Bit 9 - The interrupt enable bit for the I2C_TRANS_START_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<INT_ENA_SPEC> {
        TRANS_START_W::new(self, 9)
    }
    ///Bit 10 - The interrupt enable bit for I2C_SLAVE_STRETCH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<INT_ENA_SPEC> {
        NACK_W::new(self, 10)
    }
    ///Bit 11 - The interrupt enable bit for I2C_TXFIFO_OVF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn txfifo_ovf(&mut self) -> TXFIFO_OVF_W<INT_ENA_SPEC> {
        TXFIFO_OVF_W::new(self, 11)
    }
    ///Bit 12 - The interrupt enable bit for I2C_RXFIFO_UDF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_udf(&mut self) -> RXFIFO_UDF_W<INT_ENA_SPEC> {
        RXFIFO_UDF_W::new(self, 12)
    }
    ///Bit 13 - The interrupt enable bit for I2C_SCL_ST_TO_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn scl_st_to(&mut self) -> SCL_ST_TO_W<INT_ENA_SPEC> {
        SCL_ST_TO_W::new(self, 13)
    }
    ///Bit 14 - The interrupt enable bit for I2C_SCL_MAIN_ST_TO_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn scl_main_st_to(&mut self) -> SCL_MAIN_ST_TO_W<INT_ENA_SPEC> {
        SCL_MAIN_ST_TO_W::new(self, 14)
    }
    ///Bit 15 - The interrupt enable bit for I2C_DET_START_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn det_start(&mut self) -> DET_START_W<INT_ENA_SPEC> {
        DET_START_W::new(self, 15)
    }
}
/**Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
