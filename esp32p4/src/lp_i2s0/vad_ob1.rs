#[doc = "Register `VAD_OB1` reader"]
pub type R = crate::R<VAD_OB1_SPEC>;
#[doc = "Field `MAX_SIGNAL1_OB` reader - Reg max signal1 observe"]
pub type MAX_SIGNAL1_OB_R = crate::FieldReader<u16>;
#[doc = "Field `MAX_SIGNAL2_OB` reader - Reg max signal2 observe"]
pub type MAX_SIGNAL2_OB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Reg max signal1 observe"]
    #[inline(always)]
    pub fn max_signal1_ob(&self) -> MAX_SIGNAL1_OB_R {
        MAX_SIGNAL1_OB_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reg max signal2 observe"]
    #[inline(always)]
    pub fn max_signal2_ob(&self) -> MAX_SIGNAL2_OB_R {
        MAX_SIGNAL2_OB_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VAD_OB1")
            .field(
                "max_signal1_ob",
                &format_args!("{}", self.max_signal1_ob().bits()),
            )
            .field(
                "max_signal2_ob",
                &format_args!("{}", self.max_signal2_ob().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VAD_OB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2S VAD Observe register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vad_ob1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAD_OB1_SPEC;
impl crate::RegisterSpec for VAD_OB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vad_ob1::R`](R) reader structure"]
impl crate::Readable for VAD_OB1_SPEC {}
#[doc = "`reset()` method sets VAD_OB1 to value 0"]
impl crate::Resettable for VAD_OB1_SPEC {
    const RESET_VALUE: u32 = 0;
}
