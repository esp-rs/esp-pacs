#[doc = "Register `REDCY_SIG1` reader"]
pub type R = crate::R<REDCY_SIG1_SPEC>;
#[doc = "Register `REDCY_SIG1` writer"]
pub type W = crate::W<REDCY_SIG1_SPEC>;
#[doc = "Field `REDCY_SIG1` reader - "]
pub type REDCY_SIG1_R = crate::FieldReader<u32>;
#[doc = "Field `REDCY_SIG1` writer - "]
pub type REDCY_SIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `REDCY_NANDOR` reader - "]
pub type REDCY_NANDOR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn redcy_sig1(&self) -> REDCY_SIG1_R {
        REDCY_SIG1_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn redcy_nandor(&self) -> REDCY_NANDOR_R {
        REDCY_NANDOR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REDCY_SIG1")
            .field("redcy_sig1", &self.redcy_sig1())
            .field("redcy_nandor", &self.redcy_nandor())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn redcy_sig1(&mut self) -> REDCY_SIG1_W<REDCY_SIG1_SPEC> {
        REDCY_SIG1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`redcy_sig1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`redcy_sig1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDCY_SIG1_SPEC;
impl crate::RegisterSpec for REDCY_SIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`redcy_sig1::R`](R) reader structure"]
impl crate::Readable for REDCY_SIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`redcy_sig1::W`](W) writer structure"]
impl crate::Writable for REDCY_SIG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REDCY_SIG1 to value 0"]
impl crate::Resettable for REDCY_SIG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
