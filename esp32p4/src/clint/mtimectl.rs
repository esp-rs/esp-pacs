#[doc = "Register `MTIMECTL` reader"]
pub type R = crate::R<MTIMECTL_SPEC>;
#[doc = "Register `MTIMECTL` writer"]
pub type W = crate::W<MTIMECTL_SPEC>;
#[doc = "Field `MTIME_EN` reader - Configures whether to enable the system counter. This bit is implemented in the CLINT of core 0 only."]
pub type MTIME_EN_R = crate::BitReader;
#[doc = "Field `MTIME_EN` writer - Configures whether to enable the system counter. This bit is implemented in the CLINT of core 0 only."]
pub type MTIME_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTIME_OVF` reader - Set by hardware when the system counter reaches its maximum value. Software can clear this bit by writing 0 to it."]
pub type MTIME_OVF_R = crate::BitReader;
#[doc = "Field `MTIME_OVF` writer - Set by hardware when the system counter reaches its maximum value. Software can clear this bit by writing 0 to it."]
pub type MTIME_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configures the sampling mode of MTIME, to allow reading the 64-bit counter value consistently.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTIME_SAM {
    #[doc = "0: No sampling"]
    None        = 0,
    #[doc = "1: Sample the upper half of the system counter when the lower half of MTIME is read"]
    SampleHigh  = 1,
    #[doc = "2: Sample the lower half of the system counter when the upper half of MTIME is read"]
    SampleLow   = 2,
    #[doc = "3: Sample the other half of the system counter when either half of MTIME is read"]
    SampleOther = 3,
}
impl From<MTIME_SAM> for u8 {
    #[inline(always)]
    fn from(variant: MTIME_SAM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MTIME_SAM {
    type Ux = u8;
}
impl crate::IsEnum for MTIME_SAM {}
#[doc = "Field `MTIME_SAM` reader - Configures the sampling mode of MTIME, to allow reading the 64-bit counter value consistently."]
pub type MTIME_SAM_R = crate::FieldReader<MTIME_SAM>;
impl MTIME_SAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MTIME_SAM {
        match self.bits {
            0 => MTIME_SAM::None,
            1 => MTIME_SAM::SampleHigh,
            2 => MTIME_SAM::SampleLow,
            3 => MTIME_SAM::SampleOther,
            _ => unreachable!(),
        }
    }
    #[doc = "No sampling"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MTIME_SAM::None
    }
    #[doc = "Sample the upper half of the system counter when the lower half of MTIME is read"]
    #[inline(always)]
    pub fn is_sample_high(&self) -> bool {
        *self == MTIME_SAM::SampleHigh
    }
    #[doc = "Sample the lower half of the system counter when the upper half of MTIME is read"]
    #[inline(always)]
    pub fn is_sample_low(&self) -> bool {
        *self == MTIME_SAM::SampleLow
    }
    #[doc = "Sample the other half of the system counter when either half of MTIME is read"]
    #[inline(always)]
    pub fn is_sample_other(&self) -> bool {
        *self == MTIME_SAM::SampleOther
    }
}
#[doc = "Field `MTIME_SAM` writer - Configures the sampling mode of MTIME, to allow reading the 64-bit counter value consistently."]
pub type MTIME_SAM_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MTIME_SAM, crate::Safe>;
impl<'a, REG> MTIME_SAM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No sampling"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(MTIME_SAM::None)
    }
    #[doc = "Sample the upper half of the system counter when the lower half of MTIME is read"]
    #[inline(always)]
    pub fn sample_high(self) -> &'a mut crate::W<REG> {
        self.variant(MTIME_SAM::SampleHigh)
    }
    #[doc = "Sample the lower half of the system counter when the upper half of MTIME is read"]
    #[inline(always)]
    pub fn sample_low(self) -> &'a mut crate::W<REG> {
        self.variant(MTIME_SAM::SampleLow)
    }
    #[doc = "Sample the other half of the system counter when either half of MTIME is read"]
    #[inline(always)]
    pub fn sample_other(self) -> &'a mut crate::W<REG> {
        self.variant(MTIME_SAM::SampleOther)
    }
}
impl R {
    #[doc = "Bit 0 - Configures whether to enable the system counter. This bit is implemented in the CLINT of core 0 only."]
    #[inline(always)]
    pub fn mtime_en(&self) -> MTIME_EN_R {
        MTIME_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set by hardware when the system counter reaches its maximum value. Software can clear this bit by writing 0 to it."]
    #[inline(always)]
    pub fn mtime_ovf(&self) -> MTIME_OVF_R {
        MTIME_OVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the sampling mode of MTIME, to allow reading the 64-bit counter value consistently."]
    #[inline(always)]
    pub fn mtime_sam(&self) -> MTIME_SAM_R {
        MTIME_SAM_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTIMECTL")
            .field("mtime_en", &self.mtime_en())
            .field("mtime_ovf", &self.mtime_ovf())
            .field("mtime_sam", &self.mtime_sam())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable the system counter. This bit is implemented in the CLINT of core 0 only."]
    #[inline(always)]
    pub fn mtime_en(&mut self) -> MTIME_EN_W<'_, MTIMECTL_SPEC> {
        MTIME_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set by hardware when the system counter reaches its maximum value. Software can clear this bit by writing 0 to it."]
    #[inline(always)]
    pub fn mtime_ovf(&mut self) -> MTIME_OVF_W<'_, MTIMECTL_SPEC> {
        MTIME_OVF_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the sampling mode of MTIME, to allow reading the 64-bit counter value consistently."]
    #[inline(always)]
    pub fn mtime_sam(&mut self) -> MTIME_SAM_W<'_, MTIMECTL_SPEC> {
        MTIME_SAM_W::new(self, 2)
    }
}
#[doc = "Core-local machine timer interrupt control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimectl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimectl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECTL_SPEC;
impl crate::RegisterSpec for MTIMECTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimectl::R`](R) reader structure"]
impl crate::Readable for MTIMECTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimectl::W`](W) writer structure"]
impl crate::Writable for MTIMECTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTIMECTL to value 0x01"]
impl crate::Resettable for MTIMECTL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
