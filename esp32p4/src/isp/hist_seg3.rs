#[doc = "Register `HIST_SEG3` reader"]
pub type R = crate::R<HIST_SEG3_SPEC>;
#[doc = "Register `HIST_SEG3` writer"]
pub type W = crate::W<HIST_SEG3_SPEC>;
#[doc = "Field `HIST_SEG_14_15` reader - this field configures threshold of histogram bin 14 and bin 15"]
pub type HIST_SEG_14_15_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_14_15` writer - this field configures threshold of histogram bin 14 and bin 15"]
pub type HIST_SEG_14_15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_13_14` reader - this field configures threshold of histogram bin 13 and bin 14"]
pub type HIST_SEG_13_14_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_13_14` writer - this field configures threshold of histogram bin 13 and bin 14"]
pub type HIST_SEG_13_14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_SEG_12_13` reader - this field configures threshold of histogram bin 12 and bin 13"]
pub type HIST_SEG_12_13_R = crate::FieldReader;
#[doc = "Field `HIST_SEG_12_13` writer - this field configures threshold of histogram bin 12 and bin 13"]
pub type HIST_SEG_12_13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 14 and bin 15"]
    #[inline(always)]
    pub fn hist_seg_14_15(&self) -> HIST_SEG_14_15_R {
        HIST_SEG_14_15_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 13 and bin 14"]
    #[inline(always)]
    pub fn hist_seg_13_14(&self) -> HIST_SEG_13_14_R {
        HIST_SEG_13_14_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 12 and bin 13"]
    #[inline(always)]
    pub fn hist_seg_12_13(&self) -> HIST_SEG_12_13_R {
        HIST_SEG_12_13_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_SEG3")
            .field(
                "hist_seg_14_15",
                &format_args!("{}", self.hist_seg_14_15().bits()),
            )
            .field(
                "hist_seg_13_14",
                &format_args!("{}", self.hist_seg_13_14().bits()),
            )
            .field(
                "hist_seg_12_13",
                &format_args!("{}", self.hist_seg_12_13().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_SEG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures threshold of histogram bin 14 and bin 15"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_14_15(&mut self) -> HIST_SEG_14_15_W<HIST_SEG3_SPEC> {
        HIST_SEG_14_15_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures threshold of histogram bin 13 and bin 14"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_13_14(&mut self) -> HIST_SEG_13_14_W<HIST_SEG3_SPEC> {
        HIST_SEG_13_14_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures threshold of histogram bin 12 and bin 13"]
    #[inline(always)]
    #[must_use]
    pub fn hist_seg_12_13(&mut self) -> HIST_SEG_12_13_W<HIST_SEG3_SPEC> {
        HIST_SEG_12_13_W::new(self, 16)
    }
}
#[doc = "histogram bin control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_seg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_seg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_SEG3_SPEC;
impl crate::RegisterSpec for HIST_SEG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_seg3::R`](R) reader structure"]
impl crate::Readable for HIST_SEG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_seg3::W`](W) writer structure"]
impl crate::Writable for HIST_SEG3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_SEG3 to value 0x00d0_e0f0"]
impl crate::Resettable for HIST_SEG3_SPEC {
    const RESET_VALUE: u32 = 0x00d0_e0f0;
}
