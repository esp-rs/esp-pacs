#[doc = "Register `SEC_CTRL` reader"]
pub type R = crate::R<SEC_CTRL_SPEC>;
#[doc = "Register `SEC_CTRL` writer"]
pub type W = crate::W<SEC_CTRL_SPEC>;
#[doc = "Field `SEC_EN` reader - "]
pub type SEC_EN_R = crate::BitReader;
#[doc = "Field `SEC_EN` writer - "]
pub type SEC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEC_PAYLOAD_OFFSET` reader - "]
pub type SEC_PAYLOAD_OFFSET_R = crate::FieldReader;
#[doc = "Field `SEC_PAYLOAD_OFFSET` writer - "]
pub type SEC_PAYLOAD_OFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sec_en(&self) -> SEC_EN_R {
        SEC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn sec_payload_offset(&self) -> SEC_PAYLOAD_OFFSET_R {
        SEC_PAYLOAD_OFFSET_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_CTRL")
            .field("sec_en", &self.sec_en())
            .field("sec_payload_offset", &self.sec_payload_offset())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sec_en(&mut self) -> SEC_EN_W<SEC_CTRL_SPEC> {
        SEC_EN_W::new(self, 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    #[must_use]
    pub fn sec_payload_offset(&mut self) -> SEC_PAYLOAD_OFFSET_W<SEC_CTRL_SPEC> {
        SEC_PAYLOAD_OFFSET_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_CTRL_SPEC;
impl crate::RegisterSpec for SEC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl::R`](R) reader structure"]
impl crate::Readable for SEC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl::W`](W) writer structure"]
impl crate::Writable for SEC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CTRL to value 0"]
impl crate::Resettable for SEC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
