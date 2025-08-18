#[doc = "Register `RAW` reader"]
pub type R = crate::R<RAW_SPEC>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RAW_SPEC>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` writer - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L1` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L1` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L1` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L1_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L1` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_OVF_L3` reader - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_OVF_L3` writer - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
pub type OUTFIFO_OVF_L3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_UDF_L3` reader - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L3_R = crate::BitReader;
#[doc = "Field `OUTFIFO_UDF_L3` writer - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
pub type OUTFIFO_UDF_L3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&self) -> OUTFIFO_OVF_L1_R {
        OUTFIFO_OVF_L1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&self) -> OUTFIFO_UDF_L1_R {
        OUTFIFO_UDF_L1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l3(&self) -> OUTFIFO_OVF_L3_R {
        OUTFIFO_OVF_L3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l3(&self) -> OUTFIFO_UDF_L3_R {
        OUTFIFO_UDF_L3_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW")
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("out_total_eof", &self.out_total_eof())
            .field("outfifo_ovf_l1", &self.outfifo_ovf_l1())
            .field("outfifo_udf_l1", &self.outfifo_udf_l1())
            .field("outfifo_ovf_l3", &self.outfifo_ovf_l3())
            .field("outfifo_udf_l3", &self.outfifo_udf_l3())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel 0."]
    #[inline(always)]
    pub fn out_done(&mut self) -> OUT_DONE_W<'_, RAW_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel 0."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<'_, RAW_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error, including owner error, the second and third word error of outlink descriptor for Tx channel 0."]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<'_, RAW_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel 0."]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<'_, RAW_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l1(&mut self) -> OUTFIFO_OVF_L1_W<'_, RAW_SPEC> {
        OUTFIFO_OVF_L1_W::new(self, 4)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l1(&mut self) -> OUTFIFO_UDF_L1_W<'_, RAW_SPEC> {
        OUTFIFO_UDF_L1_W::new(self, 5)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is overflow."]
    #[inline(always)]
    pub fn outfifo_ovf_l3(&mut self) -> OUTFIFO_OVF_L3_W<'_, RAW_SPEC> {
        OUTFIFO_OVF_L3_W::new(self, 6)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 3 fifo of Tx channel 0 is underflow."]
    #[inline(always)]
    pub fn outfifo_udf_l3(&mut self) -> OUTFIFO_UDF_L3_W<'_, RAW_SPEC> {
        OUTFIFO_UDF_L3_W::new(self, 7)
    }
}
#[doc = "Raw status interrupt of Tx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_SPEC;
impl crate::RegisterSpec for RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw::R`](R) reader structure"]
impl crate::Readable for RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw::W`](W) writer structure"]
impl crate::Writable for RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAW to value 0"]
impl crate::Resettable for RAW_SPEC {}
