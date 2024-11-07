#[doc = "Register `RETENTION_CTRL` reader"]
pub type R = crate::R<RETENTION_CTRL_SPEC>;
#[doc = "Register `RETENTION_CTRL` writer"]
pub type W = crate::W<RETENTION_CTRL_SPEC>;
#[doc = "Field `RETENTION_CPU_LINK_ADDR` reader - ******* Description ***********"]
pub type RETENTION_CPU_LINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RETENTION_CPU_LINK_ADDR` writer - ******* Description ***********"]
pub type RETENTION_CPU_LINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `NOBYPASS_CPU_ISO_RST` reader - ******* Description ***********"]
pub type NOBYPASS_CPU_ISO_RST_R = crate::BitReader;
#[doc = "Field `NOBYPASS_CPU_ISO_RST` writer - ******* Description ***********"]
pub type NOBYPASS_CPU_ISO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_cpu_link_addr(&self) -> RETENTION_CPU_LINK_ADDR_R {
        RETENTION_CPU_LINK_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 27 - ******* Description ***********"]
    #[inline(always)]
    pub fn nobypass_cpu_iso_rst(&self) -> NOBYPASS_CPU_ISO_RST_R {
        NOBYPASS_CPU_ISO_RST_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL")
            .field("retention_cpu_link_addr", &self.retention_cpu_link_addr())
            .field("nobypass_cpu_iso_rst", &self.nobypass_cpu_iso_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_cpu_link_addr(&mut self) -> RETENTION_CPU_LINK_ADDR_W<RETENTION_CTRL_SPEC> {
        RETENTION_CPU_LINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 27 - ******* Description ***********"]
    #[inline(always)]
    pub fn nobypass_cpu_iso_rst(&mut self) -> NOBYPASS_CPU_ISO_RST_W<RETENTION_CTRL_SPEC> {
        NOBYPASS_CPU_ISO_RST_W::new(self, 27)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`retention_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`retention_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CTRL_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_ctrl::R`](R) reader structure"]
impl crate::Readable for RETENTION_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_ctrl::W`](W) writer structure"]
impl crate::Writable for RETENTION_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL to value 0"]
impl crate::Resettable for RETENTION_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
