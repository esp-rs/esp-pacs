#[doc = "Register `RAW` reader"]
pub type R = crate::R<RAW_SPEC>;
#[doc = "Register `RAW` writer"]
pub type W = crate::W<RAW_SPEC>;
#[doc = "Field `OUT_DONE` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_DONE` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` reader - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` writer - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` reader - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` writer - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L1_OVF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L1_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L1_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L1_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L1_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L2_OVF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L2_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L2_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L2_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L2_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L2_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_OVF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L3_OVF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_OVF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
pub type OUTFIFO_L3_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_L3_UDF` reader - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L3_UDF_R = crate::BitReader;
#[doc = "Field `OUTFIFO_L3_UDF` writer - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
pub type OUTFIFO_L3_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    pub fn outfifo_l1_ovf(&self) -> OUTFIFO_L1_OVF_R {
        OUTFIFO_L1_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    pub fn outfifo_l1_udf(&self) -> OUTFIFO_L1_UDF_R {
        OUTFIFO_L1_UDF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    pub fn outfifo_l2_ovf(&self) -> OUTFIFO_L2_OVF_R {
        OUTFIFO_L2_OVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    pub fn outfifo_l2_udf(&self) -> OUTFIFO_L2_UDF_R {
        OUTFIFO_L2_UDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    pub fn outfifo_l3_ovf(&self) -> OUTFIFO_L3_OVF_R {
        OUTFIFO_L3_OVF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    pub fn outfifo_l3_udf(&self) -> OUTFIFO_L3_UDF_R {
        OUTFIFO_L3_UDF_R::new(((self.bits >> 9) & 1) != 0)
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
            .field("outfifo_l1_ovf", &self.outfifo_l1_ovf())
            .field("outfifo_l1_udf", &self.outfifo_l1_udf())
            .field("outfifo_l2_ovf", &self.outfifo_l2_ovf())
            .field("outfifo_l2_udf", &self.outfifo_l2_udf())
            .field("outfifo_l3_ovf", &self.outfifo_l3_ovf())
            .field("outfifo_l3_udf", &self.outfifo_l3_udf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been transmitted to peripherals for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<RAW_SPEC> {
        OUT_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt bit turns to high level when the last data pointed by one outlink descriptor has been read from memory for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<RAW_SPEC> {
        OUT_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt bit turns to high level when detecting outlink descriptor error including owner error and the second and third word error of outlink descriptor for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<RAW_SPEC> {
        OUT_DSCR_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt bit turns to high level when data corresponding a outlink (includes one link descriptor or few link descriptors) is transmitted out for Tx channel0."]
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<RAW_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l1_ovf(&mut self) -> OUTFIFO_L1_OVF_W<RAW_SPEC> {
        OUTFIFO_L1_OVF_W::new(self, 4)
    }
    #[doc = "Bit 5 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l1_udf(&mut self) -> OUTFIFO_L1_UDF_W<RAW_SPEC> {
        OUTFIFO_L1_UDF_W::new(self, 5)
    }
    #[doc = "Bit 6 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l2_ovf(&mut self) -> OUTFIFO_L2_OVF_W<RAW_SPEC> {
        OUTFIFO_L2_OVF_W::new(self, 6)
    }
    #[doc = "Bit 7 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l2_udf(&mut self) -> OUTFIFO_L2_UDF_W<RAW_SPEC> {
        OUTFIFO_L2_UDF_W::new(self, 7)
    }
    #[doc = "Bit 8 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l3_ovf(&mut self) -> OUTFIFO_L3_OVF_W<RAW_SPEC> {
        OUTFIFO_L3_OVF_W::new(self, 8)
    }
    #[doc = "Bit 9 - This raw interrupt bit turns to high level when level 1 fifo of Tx channel0 is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_l3_udf(&mut self) -> OUTFIFO_L3_UDF_W<RAW_SPEC> {
        OUTFIFO_L3_UDF_W::new(self, 9)
    }
}
#[doc = "Raw status interrupt of channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_SPEC;
impl crate::RegisterSpec for RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw::R`](R) reader structure"]
impl crate::Readable for RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw::W`](W) writer structure"]
impl crate::Writable for RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW to value 0"]
impl crate::Resettable for RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
