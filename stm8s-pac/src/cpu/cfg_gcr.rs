#[doc = "Register `CFG_GCR` reader"]
pub type R = crate::R<CFG_GCR_SPEC>;
#[doc = "Register `CFG_GCR` writer"]
pub type W = crate::W<CFG_GCR_SPEC>;
#[doc = "Field `SWO` reader - "]
pub type SWO_R = crate::BitReader;
#[doc = "Field `SWO` writer - "]
pub type SWO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AL` reader - "]
pub type AL_R = crate::BitReader;
#[doc = "Field `AL` writer - "]
pub type AL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swo(&self) -> SWO_R {
        SWO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn swo(&mut self) -> SWO_W<CFG_GCR_SPEC> {
        SWO_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn al(&mut self) -> AL_W<CFG_GCR_SPEC> {
        AL_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_GCR_SPEC;
impl crate::RegisterSpec for CFG_GCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfg_gcr::R`](R) reader structure"]
impl crate::Readable for CFG_GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_gcr::W`](W) writer structure"]
impl crate::Writable for CFG_GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_GCR to value 0"]
impl crate::Resettable for CFG_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
