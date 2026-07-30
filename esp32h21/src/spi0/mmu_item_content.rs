#[doc = "Register `MMU_ITEM_CONTENT` reader"]
pub type R = crate::R<MMU_ITEM_CONTENT_SPEC>;
#[doc = "Register `MMU_ITEM_CONTENT` writer"]
pub type W = crate::W<MMU_ITEM_CONTENT_SPEC>;
#[doc = "Field `PADDR` reader - Physical page number"]
pub type PADDR_R = crate::FieldReader<u16>;
#[doc = "Field `PADDR` writer - Physical page number"]
pub type PADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `VALID` reader - Entry is valid when set"]
pub type VALID_R = crate::BitReader;
#[doc = "Field `VALID` writer - Entry is valid when set"]
pub type VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSITIVE` reader - Decrypt via flash encryption when set"]
pub type SENSITIVE_R = crate::BitReader;
#[doc = "Field `SENSITIVE` writer - Decrypt via flash encryption when set"]
pub type SENSITIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Physical page number"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Entry is valid when set"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Decrypt via flash encryption when set"]
    #[inline(always)]
    pub fn sensitive(&self) -> SENSITIVE_R {
        SENSITIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMU_ITEM_CONTENT")
            .field("paddr", &self.paddr())
            .field("valid", &self.valid())
            .field("sensitive", &self.sensitive())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Physical page number"]
    #[inline(always)]
    pub fn paddr(&mut self) -> PADDR_W<'_, MMU_ITEM_CONTENT_SPEC> {
        PADDR_W::new(self, 0)
    }
    #[doc = "Bit 9 - Entry is valid when set"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W<'_, MMU_ITEM_CONTENT_SPEC> {
        VALID_W::new(self, 9)
    }
    #[doc = "Bit 10 - Decrypt via flash encryption when set"]
    #[inline(always)]
    pub fn sensitive(&mut self) -> SENSITIVE_W<'_, MMU_ITEM_CONTENT_SPEC> {
        SENSITIVE_W::new(self, 10)
    }
}
#[doc = "MSPI-MMU item content register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmu_item_content::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmu_item_content::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMU_ITEM_CONTENT_SPEC;
impl crate::RegisterSpec for MMU_ITEM_CONTENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmu_item_content::R`](R) reader structure"]
impl crate::Readable for MMU_ITEM_CONTENT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmu_item_content::W`](W) writer structure"]
impl crate::Writable for MMU_ITEM_CONTENT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMU_ITEM_CONTENT to value 0x037c"]
impl crate::Resettable for MMU_ITEM_CONTENT_SPEC {
    const RESET_VALUE: u32 = 0x037c;
}
