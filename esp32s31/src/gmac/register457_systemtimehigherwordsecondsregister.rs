#[doc = "Register `REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER` reader"]
pub type R = crate::R<REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC>;
#[doc = "Register `REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER` writer"]
pub type W = crate::W<REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC>;
#[doc = "Field `TSHWR` reader - Timestamp Higher Word Register This field contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration The register is directly written to initialize the value This register is incremented when there is an overflow from the 32bits of the System Time Seconds register"]
pub type TSHWR_R = crate::FieldReader<u16>;
#[doc = "Field `TSHWR` writer - Timestamp Higher Word Register This field contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration The register is directly written to initialize the value This register is incremented when there is an overflow from the 32bits of the System Time Seconds register"]
pub type TSHWR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register This field contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration The register is directly written to initialize the value This register is incremented when there is an overflow from the 32bits of the System Time Seconds register"]
    #[inline(always)]
    pub fn tshwr(&self) -> TSHWR_R {
        TSHWR_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER")
            .field("tshwr", &self.tshwr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Higher Word Register This field contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the Enable IEEE 1588 Higher Word Register option during core configuration The register is directly written to initialize the value This register is incremented when there is an overflow from the 32bits of the System Time Seconds register"]
    #[inline(always)]
    pub fn tshwr(&mut self) -> TSHWR_W<'_, REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC> {
        TSHWR_W::new(self, 0)
    }
}
#[doc = "Contains the most significant 16bits of the timestamp seconds value This register is optional and can be selected using the parameter mentioned in “IEEE 1588 Timestamp Block” on page 492\n\nYou can [`read`](crate::Reg::read) this register and get [`register457_systemtimehigherwordsecondsregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register457_systemtimehigherwordsecondsregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register457_systemtimehigherwordsecondsregister::R`](R) reader structure"]
impl crate::Readable for REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register457_systemtimehigherwordsecondsregister::W`](W) writer structure"]
impl crate::Writable for REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER to value 0"]
impl crate::Resettable for REGISTER457_SYSTEMTIMEHIGHERWORDSECONDSREGISTER_SPEC {}
