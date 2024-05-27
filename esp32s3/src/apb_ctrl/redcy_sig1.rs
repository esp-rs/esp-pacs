///Register `REDCY_SIG1` reader
pub type R = crate::R<REDCY_SIG1_SPEC>;
///Register `REDCY_SIG1` writer
pub type W = crate::W<REDCY_SIG1_SPEC>;
///Field `REDCY_SIG1` reader - ******* Description ***********
pub type REDCY_SIG1_R = crate::FieldReader<u32>;
///Field `REDCY_SIG1` writer - ******* Description ***********
pub type REDCY_SIG1_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
///Field `REDCY_NANDOR` reader - ******* Description ***********
pub type REDCY_NANDOR_R = crate::BitReader;
impl R {
    ///Bits 0:30 - ******* Description ***********
    #[inline(always)]
    pub fn redcy_sig1(&self) -> REDCY_SIG1_R {
        REDCY_SIG1_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bit 31 - ******* Description ***********
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
    ///Bits 0:30 - ******* Description ***********
    #[inline(always)]
    #[must_use]
    pub fn redcy_sig1(&mut self) -> REDCY_SIG1_W<REDCY_SIG1_SPEC> {
        REDCY_SIG1_W::new(self, 0)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`redcy_sig1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redcy_sig1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REDCY_SIG1_SPEC;
impl crate::RegisterSpec for REDCY_SIG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`redcy_sig1::R`](R) reader structure
impl crate::Readable for REDCY_SIG1_SPEC {}
///`write(|w| ..)` method takes [`redcy_sig1::W`](W) writer structure
impl crate::Writable for REDCY_SIG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets REDCY_SIG1 to value 0
impl crate::Resettable for REDCY_SIG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
