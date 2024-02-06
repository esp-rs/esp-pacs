#[doc = "Register `HIST_WEIGHT5` reader"]
pub type R = crate::R<HIST_WEIGHT5_SPEC>;
#[doc = "Register `HIST_WEIGHT5` writer"]
pub type W = crate::W<HIST_WEIGHT5_SPEC>;
#[doc = "Field `HIST_WEIGHT_43` reader - this field configures weight of subwindow 43"]
pub type HIST_WEIGHT_43_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_43` writer - this field configures weight of subwindow 43"]
pub type HIST_WEIGHT_43_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_42` reader - this field configures weight of subwindow 42"]
pub type HIST_WEIGHT_42_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_42` writer - this field configures weight of subwindow 42"]
pub type HIST_WEIGHT_42_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_41` reader - this field configures weight of subwindow 41"]
pub type HIST_WEIGHT_41_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_41` writer - this field configures weight of subwindow 41"]
pub type HIST_WEIGHT_41_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_40` reader - this field configures weight of subwindow 40"]
pub type HIST_WEIGHT_40_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_40` writer - this field configures weight of subwindow 40"]
pub type HIST_WEIGHT_40_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 43"]
    #[inline(always)]
    pub fn hist_weight_43(&self) -> HIST_WEIGHT_43_R {
        HIST_WEIGHT_43_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 42"]
    #[inline(always)]
    pub fn hist_weight_42(&self) -> HIST_WEIGHT_42_R {
        HIST_WEIGHT_42_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 41"]
    #[inline(always)]
    pub fn hist_weight_41(&self) -> HIST_WEIGHT_41_R {
        HIST_WEIGHT_41_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 40"]
    #[inline(always)]
    pub fn hist_weight_40(&self) -> HIST_WEIGHT_40_R {
        HIST_WEIGHT_40_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT5")
            .field(
                "hist_weight_43",
                &format_args!("{}", self.hist_weight_43().bits()),
            )
            .field(
                "hist_weight_42",
                &format_args!("{}", self.hist_weight_42().bits()),
            )
            .field(
                "hist_weight_41",
                &format_args!("{}", self.hist_weight_41().bits()),
            )
            .field(
                "hist_weight_40",
                &format_args!("{}", self.hist_weight_40().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_WEIGHT5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 43"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_43(&mut self) -> HIST_WEIGHT_43_W<HIST_WEIGHT5_SPEC> {
        HIST_WEIGHT_43_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 42"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_42(&mut self) -> HIST_WEIGHT_42_W<HIST_WEIGHT5_SPEC> {
        HIST_WEIGHT_42_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 41"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_41(&mut self) -> HIST_WEIGHT_41_W<HIST_WEIGHT5_SPEC> {
        HIST_WEIGHT_41_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 40"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_40(&mut self) -> HIST_WEIGHT_40_W<HIST_WEIGHT5_SPEC> {
        HIST_WEIGHT_40_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "histogram sub-window weight register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT5_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight5::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight5::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT5 to value 0x0101_0101"]
impl crate::Resettable for HIST_WEIGHT5_SPEC {
    const RESET_VALUE: u32 = 0x0101_0101;
}
