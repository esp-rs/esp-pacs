#[doc = "Register `HIST_WEIGHT0` reader"]
pub type R = crate::R<HIST_WEIGHT0_SPEC>;
#[doc = "Register `HIST_WEIGHT0` writer"]
pub type W = crate::W<HIST_WEIGHT0_SPEC>;
#[doc = "Field `HIST_WEIGHT_03` reader - this field configures weight of subwindow 03"]
pub type HIST_WEIGHT_03_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_03` writer - this field configures weight of subwindow 03"]
pub type HIST_WEIGHT_03_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_02` reader - this field configures weight of subwindow 02"]
pub type HIST_WEIGHT_02_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_02` writer - this field configures weight of subwindow 02"]
pub type HIST_WEIGHT_02_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_01` reader - this field configures weight of subwindow 01"]
pub type HIST_WEIGHT_01_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_01` writer - this field configures weight of subwindow 01"]
pub type HIST_WEIGHT_01_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_00` reader - this field configures weight of subwindow 00 and sum of all weight should be 256"]
pub type HIST_WEIGHT_00_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_00` writer - this field configures weight of subwindow 00 and sum of all weight should be 256"]
pub type HIST_WEIGHT_00_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 03"]
    #[inline(always)]
    pub fn hist_weight_03(&self) -> HIST_WEIGHT_03_R {
        HIST_WEIGHT_03_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 02"]
    #[inline(always)]
    pub fn hist_weight_02(&self) -> HIST_WEIGHT_02_R {
        HIST_WEIGHT_02_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 01"]
    #[inline(always)]
    pub fn hist_weight_01(&self) -> HIST_WEIGHT_01_R {
        HIST_WEIGHT_01_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 00 and sum of all weight should be 256"]
    #[inline(always)]
    pub fn hist_weight_00(&self) -> HIST_WEIGHT_00_R {
        HIST_WEIGHT_00_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT0")
            .field(
                "hist_weight_03",
                &format_args!("{}", self.hist_weight_03().bits()),
            )
            .field(
                "hist_weight_02",
                &format_args!("{}", self.hist_weight_02().bits()),
            )
            .field(
                "hist_weight_01",
                &format_args!("{}", self.hist_weight_01().bits()),
            )
            .field(
                "hist_weight_00",
                &format_args!("{}", self.hist_weight_00().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_WEIGHT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 03"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_03(&mut self) -> HIST_WEIGHT_03_W<HIST_WEIGHT0_SPEC> {
        HIST_WEIGHT_03_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 02"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_02(&mut self) -> HIST_WEIGHT_02_W<HIST_WEIGHT0_SPEC> {
        HIST_WEIGHT_02_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 01"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_01(&mut self) -> HIST_WEIGHT_01_W<HIST_WEIGHT0_SPEC> {
        HIST_WEIGHT_01_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 00 and sum of all weight should be 256"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_00(&mut self) -> HIST_WEIGHT_00_W<HIST_WEIGHT0_SPEC> {
        HIST_WEIGHT_00_W::new(self, 24)
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
#[doc = "histogram sub-window weight register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT0_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight0::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight0::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT0 to value 0x0101_0101"]
impl crate::Resettable for HIST_WEIGHT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0101;
}
