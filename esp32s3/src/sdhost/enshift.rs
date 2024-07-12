#[doc = "Register `ENSHIFT` reader"]
pub type R = crate::R<ENSHIFT_SPEC>;
#[doc = "Register `ENSHIFT` writer"]
pub type W = crate::W<ENSHIFT_SPEC>;
#[doc = "Field `ENABLE_SHIFT` reader - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
pub type ENABLE_SHIFT_R = crate::FieldReader;
#[doc = "Field `ENABLE_SHIFT` writer - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
pub type ENABLE_SHIFT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
    #[inline(always)]
    pub fn enable_shift(&self) -> ENABLE_SHIFT_R {
        ENABLE_SHIFT_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENSHIFT")
            .field("enable_shift", &self.enable_shift())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Control for the amount of phase shift provided on the default enables in the design.Two bits assigned for each card. 2'b00-Default phase shift. 2'b01-Enables shifted to next immediate positive edge. 2'b10-Enables shifted to next immediate negative edge. 2'b11-Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn enable_shift(&mut self) -> ENABLE_SHIFT_W<ENSHIFT_SPEC> {
        ENABLE_SHIFT_W::new(self, 0)
    }
}
#[doc = "Enable Phase Shift register\n\nYou can [`read`](crate::Reg::read) this register and get [`enshift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enshift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENSHIFT_SPEC;
impl crate::RegisterSpec for ENSHIFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enshift::R`](R) reader structure"]
impl crate::Readable for ENSHIFT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enshift::W`](W) writer structure"]
impl crate::Writable for ENSHIFT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENSHIFT to value 0"]
impl crate::Resettable for ENSHIFT_SPEC {
    const RESET_VALUE: u32 = 0;
}
