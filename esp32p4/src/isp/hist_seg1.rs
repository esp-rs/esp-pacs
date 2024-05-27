#[doc = "Register `HIST_SEG1` reader"]
pub type R = crate::R<HIST_SEG1_SPEC>;
#[doc = "Register `HIST_SEG1` writer"]
pub type W = crate::W<HIST_SEG1_SPEC>;
#[doc = "Field `HIST_SEG_7_8` reader - this field configures threshold of histogram bin 7 and bin 8"]
pub type HIST_SEG_7_8_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_7_8` writer - this field configures threshold of histogram bin 7 and bin 8"]
pub type HIST_SEG_7_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_6_7` reader - this field configures threshold of histogram bin 6 and bin 7"]
pub type HIST_SEG_6_7_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_6_7` writer - this field configures threshold of histogram bin 6 and bin 7"]
pub type HIST_SEG_6_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_5_6` reader - this field configures threshold of histogram bin 5 and bin 6"]
pub type HIST_SEG_5_6_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_5_6` writer - this field configures threshold of histogram bin 5 and bin 6"]
pub type HIST_SEG_5_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_4_5` reader - this field configures threshold of histogram bin 4 and bin 5"]
pub type HIST_SEG_4_5_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_4_5` writer - this field configures threshold of histogram bin 4 and bin 5"]
pub type HIST_SEG_4_5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 7 and bin 8"]
    #[inline(always)]
    pub fn hist_seg_7_8(&self) -> HIST_SEG_7_8_R {
        HIST_SEG_7_8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 6 and bin 7"]
    #[inline(always)]
    pub fn hist_seg_6_7(&self) -> HIST_SEG_6_7_R {
        HIST_SEG_6_7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 5 and bin 6"]
    #[inline(always)]
    pub fn hist_seg_5_6(&self) -> HIST_SEG_5_6_R {
        HIST_SEG_5_6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 4 and bin 5"]
    #[inline(always)]
    pub fn hist_seg_4_5(&self) -> HIST_SEG_4_5_R {
        HIST_SEG_4_5_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_SEG1")
            .field("hist_seg_7_8", &self.hist_seg_7_8())
            .field("hist_seg_6_7", &self.hist_seg_6_7())
            .field("hist_seg_5_6", &self.hist_seg_5_6())
            .field("hist_seg_4_5", &self.hist_seg_4_5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 7 and bin 8"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_7_8(&mut self) -> HIST_SEG_7_8_W<HIST_SEG1_SPEC> {
        HIST_SEG_7_8_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 6 and bin 7"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_6_7(&mut self) -> HIST_SEG_6_7_W<HIST_SEG1_SPEC> {
        HIST_SEG_6_7_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 5 and bin 6"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_5_6(&mut self) -> HIST_SEG_5_6_W<HIST_SEG1_SPEC> {
        HIST_SEG_5_6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 4 and bin 5"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_4_5(&mut self) -> HIST_SEG_4_5_W<HIST_SEG1_SPEC> {
        HIST_SEG_4_5_W::new(self, 24)
    }
}
#[doc = "histogram bin control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_seg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_SEG1_SPEC;
impl crate::RegisterSpec for HIST_SEG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg1::R`](R) reader structure"]
impl crate::Readable for HIST_SEG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_seg1::W`](W) writer structure"]
impl crate::Writable for HIST_SEG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_SEG1 to value 0x5060_7080"]
impl crate::Resettable for HIST_SEG1_SPEC {
    const RESET_VALUE: u32 = 0x5060_7080;
}
