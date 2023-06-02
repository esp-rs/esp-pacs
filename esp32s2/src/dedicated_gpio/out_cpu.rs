#[doc = "Register `OUT_CPU` reader"]
pub struct R(crate::R<OUT_CPU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_CPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_CPU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_CPU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_CPU` writer"]
pub struct W(crate::W<OUT_CPU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_CPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OUT_CPU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_CPU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL0` reader - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL0_R = crate::BitReader;
#[doc = "Field `SEL0` writer - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL0_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL1` reader - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL1_R = crate::BitReader;
#[doc = "Field `SEL1` writer - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL1_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL2` reader - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL2_R = crate::BitReader;
#[doc = "Field `SEL2` writer - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL2_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL3` reader - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL3_R = crate::BitReader;
#[doc = "Field `SEL3` writer - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL3_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL4` reader - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL4_R = crate::BitReader;
#[doc = "Field `SEL4` writer - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL4_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL5` reader - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL5_R = crate::BitReader;
#[doc = "Field `SEL5` writer - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL5_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL6` reader - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL6_R = crate::BitReader;
#[doc = "Field `SEL6` writer - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL6_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
#[doc = "Field `SEL7` reader - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL7_R = crate::BitReader;
#[doc = "Field `SEL7` writer - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
pub type SEL7_W<'a, const O: u8> = crate::BitWriter<'a, OUT_CPU_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel3(&self) -> SEL3_R {
        SEL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel5(&self) -> SEL5_R {
        SEL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel6(&self) -> SEL6_R {
        SEL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    pub fn sel7(&self) -> SEL7_R {
        SEL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CPU")
            .field("sel0", &format_args!("{}", self.sel0().bit()))
            .field("sel1", &format_args!("{}", self.sel1().bit()))
            .field("sel2", &format_args!("{}", self.sel2().bit()))
            .field("sel3", &format_args!("{}", self.sel3().bit()))
            .field("sel4", &format_args!("{}", self.sel4().bit()))
            .field("sel5", &format_args!("{}", self.sel5().bit()))
            .field("sel6", &format_args!("{}", self.sel6().bit()))
            .field("sel7", &format_args!("{}", self.sel7().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CPU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Select GPIO out value configured by registers or CPU instructions for channel 0. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<0> {
        SEL0_W::new(self)
    }
    #[doc = "Bit 1 - Select GPIO out value configured by registers or CPU instructions for channel 1. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<1> {
        SEL1_W::new(self)
    }
    #[doc = "Bit 2 - Select GPIO out value configured by registers or CPU instructions for channel 2. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL2_W<2> {
        SEL2_W::new(self)
    }
    #[doc = "Bit 3 - Select GPIO out value configured by registers or CPU instructions for channel 3. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel3(&mut self) -> SEL3_W<3> {
        SEL3_W::new(self)
    }
    #[doc = "Bit 4 - Select GPIO out value configured by registers or CPU instructions for channel 4. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel4(&mut self) -> SEL4_W<4> {
        SEL4_W::new(self)
    }
    #[doc = "Bit 5 - Select GPIO out value configured by registers or CPU instructions for channel 5. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel5(&mut self) -> SEL5_W<5> {
        SEL5_W::new(self)
    }
    #[doc = "Bit 6 - Select GPIO out value configured by registers or CPU instructions for channel 6. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel6(&mut self) -> SEL6_W<6> {
        SEL6_W::new(self)
    }
    #[doc = "Bit 7 - Select GPIO out value configured by registers or CPU instructions for channel 7. 0: Configured by registers. 1: configured by CPU instructions."]
    #[inline(always)]
    #[must_use]
    pub fn sel7(&mut self) -> SEL7_W<7> {
        SEL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated GPIO output mode selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_cpu](index.html) module"]
pub struct OUT_CPU_SPEC;
impl crate::RegisterSpec for OUT_CPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_cpu::R](R) reader structure"]
impl crate::Readable for OUT_CPU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_cpu::W](W) writer structure"]
impl crate::Writable for OUT_CPU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CPU to value 0"]
impl crate::Resettable for OUT_CPU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
