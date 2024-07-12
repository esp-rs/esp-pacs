#[doc = "Register `DBI_CMDSIZE` reader"]
pub type R = crate::R<DBI_CMDSIZE_SPEC>;
#[doc = "Register `DBI_CMDSIZE` writer"]
pub type W = crate::W<DBI_CMDSIZE_SPEC>;
#[doc = "Field `WR_CMD_SIZE` reader - NA"]
pub type WR_CMD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `WR_CMD_SIZE` writer - NA"]
pub type WR_CMD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ALLOWED_CMD_SIZE` reader - NA"]
pub type ALLOWED_CMD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `ALLOWED_CMD_SIZE` writer - NA"]
pub type ALLOWED_CMD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn wr_cmd_size(&self) -> WR_CMD_SIZE_R {
        WR_CMD_SIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NA"]
    #[inline(always)]
    pub fn allowed_cmd_size(&self) -> ALLOWED_CMD_SIZE_R {
        ALLOWED_CMD_SIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBI_CMDSIZE")
            .field("wr_cmd_size", &self.wr_cmd_size())
            .field("allowed_cmd_size", &self.allowed_cmd_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn wr_cmd_size(&mut self) -> WR_CMD_SIZE_W<DBI_CMDSIZE_SPEC> {
        WR_CMD_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn allowed_cmd_size(&mut self) -> ALLOWED_CMD_SIZE_W<DBI_CMDSIZE_SPEC> {
        ALLOWED_CMD_SIZE_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_cmdsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_cmdsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_CMDSIZE_SPEC;
impl crate::RegisterSpec for DBI_CMDSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_cmdsize::R`](R) reader structure"]
impl crate::Readable for DBI_CMDSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_cmdsize::W`](W) writer structure"]
impl crate::Writable for DBI_CMDSIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBI_CMDSIZE to value 0"]
impl crate::Resettable for DBI_CMDSIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
