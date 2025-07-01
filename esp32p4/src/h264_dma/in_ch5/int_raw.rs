#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `IN_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_OVF_L1` reader - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_OVF_L1` writer - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
pub type INFIFO_OVF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_UDF_L1` reader - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `INFIFO_UDF_L1` writer - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type INFIFO_UDF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETCH_MB_COL_CNT_OVF` reader - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type FETCH_MB_COL_CNT_OVF_R = crate::BitReader;
#[doc = "Field `FETCH_MB_COL_CNT_OVF` writer - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
pub type FETCH_MB_COL_CNT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&self) -> FETCH_MB_COL_CNT_OVF_R {
        FETCH_MB_COL_CNT_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("infifo_ovf_l1", &self.infifo_ovf_l1())
            .field("infifo_udf_l1", &self.infifo_udf_l1())
            .field("fetch_mb_col_cnt_ovf", &self.fetch_mb_col_cnt_ovf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been transmitted to peripherals for Rx channel 1."]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_RAW_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one inlink descriptor has been received and no data error is detected for Rx channel 1."]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_RAW_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - This raw interrupt bit turns to high level when fifo of Rx channel is overflow."]
    #[inline(always)]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<INT_RAW_SPEC> {
        INFIFO_OVF_L1_W::new(self, 2)
    }
    #[doc = "Bit 3 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<INT_RAW_SPEC> {
        INFIFO_UDF_L1_W::new(self, 3)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when fifo of Rx channel is underflow."]
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&mut self) -> FETCH_MB_COL_CNT_OVF_W<INT_RAW_SPEC> {
        FETCH_MB_COL_CNT_OVF_W::new(self, 4)
    }
}
#[doc = "RX CH5 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
