#[doc = "Register `HIST_SEG2` reader"]
pub type R = crate::R<HIST_SEG2_SPEC>;
#[doc = "Register `HIST_SEG2` writer"]
pub type W = crate::W<HIST_SEG2_SPEC>;
#[doc = "Field `HIST_SEG_11_12` reader - this field configures threshold of histogram bin 11 and bin 12"]
pub type HIST_SEG_11_12_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_11_12` writer - this field configures threshold of histogram bin 11 and bin 12"]
pub type HIST_SEG_11_12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_10_11` reader - this field configures threshold of histogram bin 10 and bin 11"]
pub type HIST_SEG_10_11_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_10_11` writer - this field configures threshold of histogram bin 10 and bin 11"]
pub type HIST_SEG_10_11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_9_10` reader - this field configures threshold of histogram bin 9 and bin 10"]
pub type HIST_SEG_9_10_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_9_10` writer - this field configures threshold of histogram bin 9 and bin 10"]
pub type HIST_SEG_9_10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_8_9` reader - this field configures threshold of histogram bin 8 and bin 9"]
pub type HIST_SEG_8_9_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_8_9` writer - this field configures threshold of histogram bin 8 and bin 9"]
pub type HIST_SEG_8_9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 11 and bin 12"]
    #[inline(always)]
    pub fn hist_seg_11_12(&self) -> HIST_SEG_11_12_R {
        HIST_SEG_11_12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 10 and bin 11"]
    #[inline(always)]
    pub fn hist_seg_10_11(&self) -> HIST_SEG_10_11_R {
        HIST_SEG_10_11_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 9 and bin 10"]
    #[inline(always)]
    pub fn hist_seg_9_10(&self) -> HIST_SEG_9_10_R {
        HIST_SEG_9_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 8 and bin 9"]
    #[inline(always)]
    pub fn hist_seg_8_9(&self) -> HIST_SEG_8_9_R {
        HIST_SEG_8_9_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_SEG2")
            .field("hist_seg_11_12", &self.hist_seg_11_12().bits())
            .field("hist_seg_10_11", &self.hist_seg_10_11().bits())
            .field("hist_seg_9_10", &self.hist_seg_9_10().bits())
            .field("hist_seg_8_9", &self.hist_seg_8_9().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_SEG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 11 and bin 12"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_11_12(&mut self) -> HIST_SEG_11_12_W<HIST_SEG2_SPEC> {
        HIST_SEG_11_12_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 10 and bin 11"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_10_11(&mut self) -> HIST_SEG_10_11_W<HIST_SEG2_SPEC> {
        HIST_SEG_10_11_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 9 and bin 10"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_9_10(&mut self) -> HIST_SEG_9_10_W<HIST_SEG2_SPEC> {
        HIST_SEG_9_10_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures threshold of histogram bin 8 and bin 9"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_8_9(&mut self) -> HIST_SEG_8_9_W<HIST_SEG2_SPEC> {
        HIST_SEG_8_9_W::new(self, 24)
    }
}
#[doc = "histogram bin control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_seg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_SEG2_SPEC;
impl crate::RegisterSpec for HIST_SEG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg2::R`](R) reader structure"]
impl crate::Readable for HIST_SEG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_seg2::W`](W) writer structure"]
impl crate::Writable for HIST_SEG2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_SEG2 to value 0x90a0_b0c0"]
impl crate::Resettable for HIST_SEG2_SPEC {
    const RESET_VALUE: u32 = 0x90a0_b0c0;
}
