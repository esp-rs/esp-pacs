#[doc = "Register `HIST_SEG0` reader"]
pub type R = crate::R<HIST_SEG0_SPEC>;
#[doc = "Register `HIST_SEG0` writer"]
pub type W = crate::W<HIST_SEG0_SPEC>;
#[doc = "Field `HIST_SEG_3_4` reader - this field configures threshold of histogram bin 3 and bin 4"]
pub type HIST_SEG_3_4_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_3_4` writer - this field configures threshold of histogram bin 3 and bin 4"]
pub type HIST_SEG_3_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_2_3` reader - this field configures threshold of histogram bin 2 and bin 3"]
pub type HIST_SEG_2_3_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_2_3` writer - this field configures threshold of histogram bin 2 and bin 3"]
pub type HIST_SEG_2_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_1_2` reader - this field configures threshold of histogram bin 1 and bin 2"]
pub type HIST_SEG_1_2_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_1_2` writer - this field configures threshold of histogram bin 1 and bin 2"]
pub type HIST_SEG_1_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_0_1` reader - this field configures threshold of histogram bin 0 and bin 1"]
pub type HIST_SEG_0_1_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_0_1` writer - this field configures threshold of histogram bin 0 and bin 1"]
pub type HIST_SEG_0_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 3 and bin 4"]
    #[inline(always)]
    pub fn hist_seg_3_4(&self) -> HIST_SEG_3_4_R {
        HIST_SEG_3_4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 2 and bin 3"]
    #[inline(always)]
    pub fn hist_seg_2_3(&self) -> HIST_SEG_2_3_R {
        HIST_SEG_2_3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 1 and bin 2"]
    #[inline(always)]
    pub fn hist_seg_1_2(&self) -> HIST_SEG_1_2_R {
        HIST_SEG_1_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 0 and bin 1"]
    #[inline(always)]
    pub fn hist_seg_0_1(&self) -> HIST_SEG_0_1_R {
        HIST_SEG_0_1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_SEG0")
            .field("hist_seg_3_4", &self.hist_seg_3_4())
            .field("hist_seg_2_3", &self.hist_seg_2_3())
            .field("hist_seg_1_2", &self.hist_seg_1_2())
            .field("hist_seg_0_1", &self.hist_seg_0_1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 3 and bin 4"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_3_4(&mut self) -> HIST_SEG_3_4_W<HIST_SEG0_SPEC> {
        HIST_SEG_3_4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 2 and bin 3"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_2_3(&mut self) -> HIST_SEG_2_3_W<HIST_SEG0_SPEC> {
        HIST_SEG_2_3_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 1 and bin 2"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_1_2(&mut self) -> HIST_SEG_1_2_W<HIST_SEG0_SPEC> {
        HIST_SEG_1_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 0 and bin 1"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_0_1(&mut self) -> HIST_SEG_0_1_W<HIST_SEG0_SPEC> {
        HIST_SEG_0_1_W::new(self, 24)
    }
}
#[doc = "histogram bin control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_seg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_SEG0_SPEC;
impl crate::RegisterSpec for HIST_SEG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg0::R`](R) reader structure"]
impl crate::Readable for HIST_SEG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_seg0::W`](W) writer structure"]
impl crate::Writable for HIST_SEG0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_SEG0 to value 0x1020_3040"]
impl crate::Resettable for HIST_SEG0_SPEC {
    const RESET_VALUE: u32 = 0x1020_3040;
}
