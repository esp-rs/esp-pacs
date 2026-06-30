#[doc = "Register `PUF_MEM_CTRL` reader"]
pub type R = crate::R<PUF_MEM_CTRL_SPEC>;
#[doc = "Register `PUF_MEM_CTRL` writer"]
pub type W = crate::W<PUF_MEM_CTRL_SPEC>;
#[doc = "Field `PUF_MEM_PSW` reader - need_des"]
pub type PUF_MEM_PSW_R = crate::BitReader;
#[doc = "Field `PUF_MEM_PSW` writer - need_des"]
pub type PUF_MEM_PSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUF_MEM_ISO` reader - need_des"]
pub type PUF_MEM_ISO_R = crate::BitReader;
#[doc = "Field `PUF_MEM_ISO` writer - need_des"]
pub type PUF_MEM_ISO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUF_MEM_DISCHARGE` reader - need_des"]
pub type PUF_MEM_DISCHARGE_R = crate::BitReader;
#[doc = "Field `PUF_MEM_DISCHARGE` writer - need_des"]
pub type PUF_MEM_DISCHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn puf_mem_psw(&self) -> PUF_MEM_PSW_R {
        PUF_MEM_PSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn puf_mem_iso(&self) -> PUF_MEM_ISO_R {
        PUF_MEM_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn puf_mem_discharge(&self) -> PUF_MEM_DISCHARGE_R {
        PUF_MEM_DISCHARGE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUF_MEM_CTRL")
            .field("puf_mem_psw", &self.puf_mem_psw())
            .field("puf_mem_iso", &self.puf_mem_iso())
            .field("puf_mem_discharge", &self.puf_mem_discharge())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn puf_mem_psw(&mut self) -> PUF_MEM_PSW_W<'_, PUF_MEM_CTRL_SPEC> {
        PUF_MEM_PSW_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn puf_mem_iso(&mut self) -> PUF_MEM_ISO_W<'_, PUF_MEM_CTRL_SPEC> {
        PUF_MEM_ISO_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn puf_mem_discharge(&mut self) -> PUF_MEM_DISCHARGE_W<'_, PUF_MEM_CTRL_SPEC> {
        PUF_MEM_DISCHARGE_W::new(self, 2)
    }
}
#[doc = "used for future potential eco, others don't care\n\nYou can [`read`](crate::Reg::read) this register and get [`puf_mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`puf_mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUF_MEM_CTRL_SPEC;
impl crate::RegisterSpec for PUF_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`puf_mem_ctrl::R`](R) reader structure"]
impl crate::Readable for PUF_MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`puf_mem_ctrl::W`](W) writer structure"]
impl crate::Writable for PUF_MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PUF_MEM_CTRL to value 0x01"]
impl crate::Resettable for PUF_MEM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
