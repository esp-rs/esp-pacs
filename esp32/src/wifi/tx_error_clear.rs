#[doc = "Register `TX_ERROR_CLEAR` reader"]
pub type R = crate::R<TX_ERROR_CLEAR_SPEC>;
#[doc = "Register `TX_ERROR_CLEAR` writer"]
pub type W = crate::W<TX_ERROR_CLEAR_SPEC>;
#[doc = "Field `SLOT_COLLISION` reader - "]
pub type SLOT_COLLISION_R = crate::FieldReader;
#[doc = "Field `SLOT_COLLISION` writer - "]
pub type SLOT_COLLISION_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SLOT_TIMEOUT` reader - "]
pub type SLOT_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `SLOT_TIMEOUT` writer - "]
pub type SLOT_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn slot_collision(&self) -> SLOT_COLLISION_R {
        SLOT_COLLISION_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn slot_timeout(&self) -> SLOT_TIMEOUT_R {
        SLOT_TIMEOUT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ERROR_CLEAR")
            .field("slot_collision", &self.slot_collision())
            .field("slot_timeout", &self.slot_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn slot_collision(&mut self) -> SLOT_COLLISION_W<TX_ERROR_CLEAR_SPEC> {
        SLOT_COLLISION_W::new(self, 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    #[must_use]
    pub fn slot_timeout(&mut self) -> SLOT_TIMEOUT_W<TX_ERROR_CLEAR_SPEC> {
        SLOT_TIMEOUT_W::new(self, 16)
    }
}
#[doc = "Clear the error status of a slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_error_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_error_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ERROR_CLEAR_SPEC;
impl crate::RegisterSpec for TX_ERROR_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_error_clear::R`](R) reader structure"]
impl crate::Readable for TX_ERROR_CLEAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_error_clear::W`](W) writer structure"]
impl crate::Writable for TX_ERROR_CLEAR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_ERROR_CLEAR to value 0"]
impl crate::Resettable for TX_ERROR_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
