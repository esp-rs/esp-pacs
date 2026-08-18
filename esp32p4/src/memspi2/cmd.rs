#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `MST_ST` reader - "]
pub type MST_ST_R = crate::FieldReader;
#[doc = "Field `SLV_ST` reader - "]
pub type SLV_ST_R = crate::FieldReader;
#[doc = "Field `USR` reader - "]
pub type USR_R = crate::BitReader;
#[doc = "Field `USR` writer - "]
pub type USR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn mst_st(&self) -> MST_ST_R {
        MST_ST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn slv_st(&self) -> SLV_ST_R {
        SLV_ST_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usr(&self) -> USR_R {
        USR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("mst_st", &self.mst_st())
            .field("slv_st", &self.slv_st())
            .field("usr", &self.usr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn usr(&mut self) -> USR_W<'_, CMD_SPEC> {
        USR_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {}
