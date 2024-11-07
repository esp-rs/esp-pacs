#[doc = "Register `HIST_WEIGHT4` reader"]
pub type R = crate::R<HIST_WEIGHT4_SPEC>;
#[doc = "Register `HIST_WEIGHT4` writer"]
pub type W = crate::W<HIST_WEIGHT4_SPEC>;
#[doc = "Field `HIST_WEIGHT_34` reader - this field configures weight of subwindow 34"]
pub type HIST_WEIGHT_34_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_34` writer - this field configures weight of subwindow 34"]
pub type HIST_WEIGHT_34_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_33` reader - this field configures weight of subwindow 33"]
pub type HIST_WEIGHT_33_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_33` writer - this field configures weight of subwindow 33"]
pub type HIST_WEIGHT_33_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_32` reader - this field configures weight of subwindow 32"]
pub type HIST_WEIGHT_32_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_32` writer - this field configures weight of subwindow 32"]
pub type HIST_WEIGHT_32_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HIST_WEIGHT_31` reader - this field configures weight of subwindow 31"]
pub type HIST_WEIGHT_31_R = crate::FieldReader;
#[doc = "Field `HIST_WEIGHT_31` writer - this field configures weight of subwindow 31"]
pub type HIST_WEIGHT_31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 34"]
    #[inline(always)]
    pub fn hist_weight_34(&self) -> HIST_WEIGHT_34_R {
        HIST_WEIGHT_34_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 33"]
    #[inline(always)]
    pub fn hist_weight_33(&self) -> HIST_WEIGHT_33_R {
        HIST_WEIGHT_33_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 32"]
    #[inline(always)]
    pub fn hist_weight_32(&self) -> HIST_WEIGHT_32_R {
        HIST_WEIGHT_32_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 31"]
    #[inline(always)]
    pub fn hist_weight_31(&self) -> HIST_WEIGHT_31_R {
        HIST_WEIGHT_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_WEIGHT4")
            .field("hist_weight_34", &self.hist_weight_34())
            .field("hist_weight_33", &self.hist_weight_33())
            .field("hist_weight_32", &self.hist_weight_32())
            .field("hist_weight_31", &self.hist_weight_31())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures weight of subwindow 34"]
    #[inline(always)]
    pub fn hist_weight_34(&mut self) -> HIST_WEIGHT_34_W<HIST_WEIGHT4_SPEC> {
        HIST_WEIGHT_34_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures weight of subwindow 33"]
    #[inline(always)]
    pub fn hist_weight_33(&mut self) -> HIST_WEIGHT_33_W<HIST_WEIGHT4_SPEC> {
        HIST_WEIGHT_33_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures weight of subwindow 32"]
    #[inline(always)]
    pub fn hist_weight_32(&mut self) -> HIST_WEIGHT_32_W<HIST_WEIGHT4_SPEC> {
        HIST_WEIGHT_32_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures weight of subwindow 31"]
    #[inline(always)]
    pub fn hist_weight_31(&mut self) -> HIST_WEIGHT_31_W<HIST_WEIGHT4_SPEC> {
        HIST_WEIGHT_31_W::new(self, 24)
    }
}
#[doc = "histogram sub-window weight register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_weight4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_weight4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_WEIGHT4_SPEC;
impl crate::RegisterSpec for HIST_WEIGHT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_weight4::R`](R) reader structure"]
impl crate::Readable for HIST_WEIGHT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_weight4::W`](W) writer structure"]
impl crate::Writable for HIST_WEIGHT4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIST_WEIGHT4 to value 0x0101_0101"]
impl crate::Resettable for HIST_WEIGHT4_SPEC {
    const RESET_VALUE: u32 = 0x0101_0101;
}
