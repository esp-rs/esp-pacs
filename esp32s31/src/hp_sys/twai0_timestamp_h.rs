#[doc = "Register `TWAI0_TIMESTAMP_H` reader"]
pub type R = crate::R<TWAI0_TIMESTAMP_H_SPEC>;
#[doc = "Register `TWAI0_TIMESTAMP_H` writer"]
pub type W = crate::W<TWAI0_TIMESTAMP_H_SPEC>;
#[doc = "Field `TWAI0_TIMESTAMP_H` reader - This field used to set upper 32bits of timestamp hp twai0"]
pub type TWAI0_TIMESTAMP_H_R = crate::FieldReader<u32>;
#[doc = "Field `TWAI0_TIMESTAMP_H` writer - This field used to set upper 32bits of timestamp hp twai0"]
pub type TWAI0_TIMESTAMP_H_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field used to set upper 32bits of timestamp hp twai0"]
    #[inline(always)]
    pub fn twai0_timestamp_h(&self) -> TWAI0_TIMESTAMP_H_R {
        TWAI0_TIMESTAMP_H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TWAI0_TIMESTAMP_H")
            .field("twai0_timestamp_h", &self.twai0_timestamp_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This field used to set upper 32bits of timestamp hp twai0"]
    #[inline(always)]
    pub fn twai0_timestamp_h(&mut self) -> TWAI0_TIMESTAMP_H_W<'_, TWAI0_TIMESTAMP_H_SPEC> {
        TWAI0_TIMESTAMP_H_W::new(self, 0)
    }
}
#[doc = "twai0 timestamp high bit reg\n\nYou can [`read`](crate::Reg::read) this register and get [`twai0_timestamp_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twai0_timestamp_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TWAI0_TIMESTAMP_H_SPEC;
impl crate::RegisterSpec for TWAI0_TIMESTAMP_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`twai0_timestamp_h::R`](R) reader structure"]
impl crate::Readable for TWAI0_TIMESTAMP_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`twai0_timestamp_h::W`](W) writer structure"]
impl crate::Writable for TWAI0_TIMESTAMP_H_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TWAI0_TIMESTAMP_H to value 0xffff_ffff"]
impl crate::Resettable for TWAI0_TIMESTAMP_H_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
