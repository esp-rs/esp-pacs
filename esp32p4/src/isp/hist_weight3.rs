#[doc = "Register `HIST_WEIGHT3` reader"]
pub type R = crate::R<HIST_WEIGHT3_SPEC>;
#[doc = "Register `HIST_WEIGHT3` writer"]
pub type W = crate::W<HIST_WEIGHT3_SPEC>;
#[doc = "Field `HIST_WEIGHT_30` reader - this field configures weight of subwindow 30"]
pub type HIST_WEIGHT_30_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_30` writer - this field configures weight of subwindow 30"]
pub type HIST_WEIGHT_30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_24` reader - this field configures weight of subwindow 24"]
pub type HIST_WEIGHT_24_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_24` writer - this field configures weight of subwindow 24"]
pub type HIST_WEIGHT_24_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_23` reader - this field configures weight of subwindow 23"]
pub type HIST_WEIGHT_23_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_23` writer - this field configures weight of subwindow 23"]
pub type HIST_WEIGHT_23_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_22` reader - this field configures weight of subwindow 22"]
pub type HIST_WEIGHT_22_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_22` writer - this field configures weight of subwindow 22"]
pub type HIST_WEIGHT_22_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 30"]
    #[inline(always)]
    pub fn hist_weight_30(&self) -> HIST_WEIGHT_30_R {
        HIST_WEIGHT_30_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 24"]
    #[inline(always)]
    pub fn hist_weight_24(&self) -> HIST_WEIGHT_24_R {
        HIST_WEIGHT_24_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 23"]
    #[inline(always)]
    pub fn hist_weight_23(&self) -> HIST_WEIGHT_23_R {
        HIST_WEIGHT_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 22"]
    #[inline(always)]
    pub fn hist_weight_22(&self) -> HIST_WEIGHT_22_R {
        HIST_WEIGHT_22_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT3")
            .field(
                "hist_weight_30",
                &format_args!("{}", self.hist_weight_30().bits()),
            )
            .field(
                "hist_weight_24",
                &format_args!("{}", self.hist_weight_24().bits()),
            )
            .field(
                "hist_weight_23",
                &format_args!("{}", self.hist_weight_23().bits()),
            )
            .field(
                "hist_weight_22",
                &format_args!("{}", self.hist_weight_22().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_WEIGHT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 30"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_30(&mut self) -> HIST_WEIGHT_30_W<HIST_WEIGHT3_SPEC> {
        HIST_WEIGHT_30_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 24"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_24(&mut self) -> HIST_WEIGHT_24_W<HIST_WEIGHT3_SPEC> {
        HIST_WEIGHT_24_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 23"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_23(&mut self) -> HIST_WEIGHT_23_W<HIST_WEIGHT3_SPEC> {
        HIST_WEIGHT_23_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 22"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_22(&mut self) -> HIST_WEIGHT_22_W<HIST_WEIGHT3_SPEC> {
        HIST_WEIGHT_22_W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT3_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight3::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight3::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT3 to value 0xe801_0101"]
impl crate::Resettable for HIST_WEIGHT3_SPEC {
    const RESET_VALUE: u32 = 0xe801_0101;
}
