#[doc = "Register `HIST_WEIGHT1` reader"]
pub type R = crate::R<HIST_WEIGHT1_SPEC>;
#[doc = "Register `HIST_WEIGHT1` writer"]
pub type W = crate::W<HIST_WEIGHT1_SPEC>;
#[doc = "Field `HIST_WEIGHT_12` reader - this field configures weight of subwindow 12"]
pub type HIST_WEIGHT_12_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_12` writer - this field configures weight of subwindow 12"]
pub type HIST_WEIGHT_12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_11` reader - this field configures weight of subwindow 11"]
pub type HIST_WEIGHT_11_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_11` writer - this field configures weight of subwindow 11"]
pub type HIST_WEIGHT_11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_10` reader - this field configures weight of subwindow 10"]
pub type HIST_WEIGHT_10_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_10` writer - this field configures weight of subwindow 10"]
pub type HIST_WEIGHT_10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_04` reader - this field configures weight of subwindow 04"]
pub type HIST_WEIGHT_04_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_04` writer - this field configures weight of subwindow 04"]
pub type HIST_WEIGHT_04_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 12"]
    #[inline(always)]
    pub fn hist_weight_12(&self) -> HIST_WEIGHT_12_R {
        HIST_WEIGHT_12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 11"]
    #[inline(always)]
    pub fn hist_weight_11(&self) -> HIST_WEIGHT_11_R {
        HIST_WEIGHT_11_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 10"]
    #[inline(always)]
    pub fn hist_weight_10(&self) -> HIST_WEIGHT_10_R {
        HIST_WEIGHT_10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 04"]
    #[inline(always)]
    pub fn hist_weight_04(&self) -> HIST_WEIGHT_04_R {
        HIST_WEIGHT_04_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT1")
            .field(
                "hist_weight_12",
                &format_args!("{}", self.hist_weight_12().bits()),
            )
            .field(
                "hist_weight_11",
                &format_args!("{}", self.hist_weight_11().bits()),
            )
            .field(
                "hist_weight_10",
                &format_args!("{}", self.hist_weight_10().bits()),
            )
            .field(
                "hist_weight_04",
                &format_args!("{}", self.hist_weight_04().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIST_WEIGHT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 12"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_12(&mut self) -> HIST_WEIGHT_12_W<HIST_WEIGHT1_SPEC> {
        HIST_WEIGHT_12_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 11"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_11(&mut self) -> HIST_WEIGHT_11_W<HIST_WEIGHT1_SPEC> {
        HIST_WEIGHT_11_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 10"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_10(&mut self) -> HIST_WEIGHT_10_W<HIST_WEIGHT1_SPEC> {
        HIST_WEIGHT_10_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 04"]
    #[inline(always)]
    #[must_use]
    pub fn hist_weight_04(&mut self) -> HIST_WEIGHT_04_W<HIST_WEIGHT1_SPEC> {
        HIST_WEIGHT_04_W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hist_weight1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_weight1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT1_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight1::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight1::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT1 to value 0x0101_0101"]
impl crate::Resettable for HIST_WEIGHT1_SPEC {
    const RESET_VALUE: u32 = 0x0101_0101;
}
