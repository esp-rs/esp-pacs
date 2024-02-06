#[doc = "Register `INTSTATUS0` reader"]
pub type R = crate::R<INTSTATUS0_SPEC>;
#[doc = "Field `CH1_INTSTAT` reader - NA"]
pub type CH1_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH2_INTSTAT` reader - NA"]
pub type CH2_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH3_INTSTAT` reader - NA"]
pub type CH3_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH4_INTSTAT` reader - NA"]
pub type CH4_INTSTAT_R = crate::BitReader;
#[doc = "Field `COMMONREG_INTSTAT` reader - NA"]
pub type COMMONREG_INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_intstat(&self) -> CH1_INTSTAT_R {
        CH1_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch2_intstat(&self) -> CH2_INTSTAT_R {
        CH2_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch3_intstat(&self) -> CH3_INTSTAT_R {
        CH3_INTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch4_intstat(&self) -> CH4_INTSTAT_R {
        CH4_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn commonreg_intstat(&self) -> COMMONREG_INTSTAT_R {
        COMMONREG_INTSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTATUS0")
            .field("ch1_intstat", &format_args!("{}", self.ch1_intstat().bit()))
            .field("ch2_intstat", &format_args!("{}", self.ch2_intstat().bit()))
            .field("ch3_intstat", &format_args!("{}", self.ch3_intstat().bit()))
            .field("ch4_intstat", &format_args!("{}", self.ch4_intstat().bit()))
            .field(
                "commonreg_intstat",
                &format_args!("{}", self.commonreg_intstat().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTSTATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTATUS0_SPEC;
impl crate::RegisterSpec for INTSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus0::R`](R) reader structure"]
impl crate::Readable for INTSTATUS0_SPEC {}
#[doc = "`reset()` method sets INTSTATUS0 to value 0"]
impl crate::Resettable for INTSTATUS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
