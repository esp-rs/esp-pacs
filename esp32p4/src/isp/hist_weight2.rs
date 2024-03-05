#[doc = "Register `HIST_WEIGHT2` reader"]
pub type R = crate::R<HIST_WEIGHT2_SPEC>;
#[doc = "Register `HIST_WEIGHT2` writer"]
pub type W = crate::W<HIST_WEIGHT2_SPEC>;
#[doc = "Field `HIST_WEIGHT_21` reader - this field configures weight of subwindow 21"]
pub type HIST_WEIGHT_21_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_21` writer - this field configures weight of subwindow 21"]
pub type HIST_WEIGHT_21_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_20` reader - this field configures weight of subwindow 20"]
pub type HIST_WEIGHT_20_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_20` writer - this field configures weight of subwindow 20"]
pub type HIST_WEIGHT_20_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_14` reader - this field configures weight of subwindow 04"]
pub type HIST_WEIGHT_14_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_14` writer - this field configures weight of subwindow 04"]
pub type HIST_WEIGHT_14_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_13` reader - this field configures weight of subwindow 13"]
pub type HIST_WEIGHT_13_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_13` writer - this field configures weight of subwindow 13"]
pub type HIST_WEIGHT_13_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 21"]
    #[inline(always)]
    pub fn hist_weight_21(&self) -> HIST_WEIGHT_21_R {
        HIST_WEIGHT_21_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 20"]
    #[inline(always)]
    pub fn hist_weight_20(&self) -> HIST_WEIGHT_20_R {
        HIST_WEIGHT_20_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 04"]
    #[inline(always)]
    pub fn hist_weight_14(&self) -> HIST_WEIGHT_14_R {
        HIST_WEIGHT_14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 13"]
    #[inline(always)]
    pub fn hist_weight_13(&self) -> HIST_WEIGHT_13_R {
        HIST_WEIGHT_13_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT2")
            .field(
                "hist_weight_21",
                &format_args!("{}", self.hist_weight_21().bits()),
            )
            .field(
                "hist_weight_20",
                &format_args!("{}", self.hist_weight_20().bits()),
            )
            .field(
                "hist_weight_14",
                &format_args!("{}", self.hist_weight_14().bits()),
            )
            .field(
                "hist_weight_13",
                &format_args!("{}", self.hist_weight_13().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_WEIGHT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 21"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_21(&mut self) -> HIST_WEIGHT_21_W<HIST_WEIGHT2_SPEC> {
        HIST_WEIGHT_21_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 20"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_20(&mut self) -> HIST_WEIGHT_20_W<HIST_WEIGHT2_SPEC> {
        HIST_WEIGHT_20_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 04"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_14(&mut self) -> HIST_WEIGHT_14_W<HIST_WEIGHT2_SPEC> {
        HIST_WEIGHT_14_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 13"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_13(&mut self) -> HIST_WEIGHT_13_W<HIST_WEIGHT2_SPEC> {
        HIST_WEIGHT_13_W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT2_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight2::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight2::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT2 to value 0x0101_0101"]
impl crate::Resettable for HIST_WEIGHT2_SPEC {
    const RESET_VALUE: u32 = 0x0101_0101;
}
